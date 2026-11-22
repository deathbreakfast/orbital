//! Shared paging primitives for Orbital apps.
//!
//! This crate provides [`Page`] and [`PageRequest`], the wire types every
//! Orbital list uses between a Leptos server function and the client
//! ([`DataTable`](https://docs.rs/orbital-datatable) server mode, infinite
//! scroll, history timelines). It has no UI or framework dependencies, so any
//! crate can depend on it without introducing cycles.
//!
//! A list pages in one of two ways:
//!
//! * **Offset.** The client sends `offset` and `limit`; the server skips
//!   `offset` rows. Simple, and fine for small or SQL-sorted lists. Deep pages
//!   re-read every earlier row.
//! * **Cursor.** The server returns [`Page::next_cursor`], an opaque token
//!   built with [`PageCursor`], and the client sends it back as
//!   [`PageRequest::cursor`]. The server continues from where the last page
//!   stopped (a Valence resume position, or a keyset like `(created_at, id)`)
//!   instead of re-reading from the start. Use it for lists behind read
//!   privacy or Rust-side filters, and for tables that grow quickly.
//!
//! The two mix freely. A request always carries `offset`, so a server that
//! can't use the cursor (absent, stale, or from another query) pages by
//! `offset` as before, and a page with `has_more` but no `next_cursor` tells
//! the client to use `offset` for the next one.
//!
//! When the **`leptos`** feature is enabled, this crate also provides
//! [`use_paged_infinite_scroll`], a hook that loads pages as the user scrolls
//! and sends each page's `next_cursor` with the next fetch.
//!
//! # Example: a server function that pages by cursor
//!
//! ```rust
//! use orbital_paging::{Page, PageCursor, PageRequest};
//!
//! // Stand-in for a store read that starts after a known position and
//! // reports where it stopped.
//! fn read_from(position: u64, limit: u32) -> (Vec<u64>, Option<u64>) {
//!     const ROWS: u64 = 30;
//!     let end = (position + u64::from(limit)).min(ROWS);
//!     let rows = (position..end).collect();
//!     (rows, (end < ROWS).then_some(end))
//! }
//!
//! fn list_rows(request: &PageRequest) -> Page<u64> {
//!     let scope = "rows"; // what else the result depends on: user, hub, …
//!     // A cursor from this exact query continues where it stopped;
//!     // anything else falls back to the offset path.
//!     let start = match PageCursor::from_request(request, scope) {
//!         Some(cursor) => cursor.position,
//!         None => u64::from(request.offset),
//!     };
//!     let (rows, resume) = read_from(start, request.limit);
//!     let next = resume.map(|at| PageCursor::new(at, request.query_fingerprint(scope)).encode());
//!     Page {
//!         items: rows,
//!         has_more: resume.is_some(),
//!         // Counting would need a full read, so later pages leave it unknown.
//!         total_count: request.is_first_page().then_some(30),
//!         ..Page::default()
//!     }
//!     .with_next_cursor(next)
//! }
//!
//! // Page 1, then page 2 from its cursor.
//! let first = list_rows(&PageRequest::new(0, 10));
//! assert_eq!(first.items, (0..10).collect::<Vec<_>>());
//! let second = list_rows(&PageRequest::new(10, 10).with_cursor(first.next_cursor.clone()));
//! assert_eq!(second.items, (10..20).collect::<Vec<_>>());
//!
//! // A cursor from a different query is ignored, and offset takes over.
//! let mut searched = PageRequest::new(10, 10).with_cursor(first.next_cursor);
//! searched.quick_search = Some("ada".into());
//! assert_eq!(list_rows(&searched).items, (10..20).collect::<Vec<_>>());
//! ```

use orbital_data::DataValue;
use serde::{Deserialize, Serialize};

mod cursor;
pub use cursor::{PageCursor, MAX_CURSOR_TOKEN_LEN};

#[cfg(feature = "leptos")]
mod infinite_scroll;
#[cfg(feature = "leptos")]
pub use infinite_scroll::*;

/// Sort direction on the wire.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortDirectionWire {
    Asc,
    Desc,
}

/// Single sort column on the wire.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SortParam {
    pub field: String,
    pub direction: SortDirectionWire,
}

/// How multiple filter rules combine on the wire.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FilterLogicWire {
    #[default]
    And,
    Or,
}

/// Single filter rule on the wire.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilterRuleParam {
    pub field: String,
    pub operator: String,
    pub value: DataValue,
}

/// Structured filter query on the wire.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilterQuery {
    pub items: Vec<FilterRuleParam>,
    pub logic: FilterLogicWire,
}

/// Client-to-server pagination and query parameters.
///
/// `Default` is an empty request (`offset` and `limit` 0, no query); use it
/// with struct-update syntax so new optional fields don't break callers.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PageRequest {
    /// Number of items to skip.
    pub offset: u32,
    /// Maximum number of items to return.
    pub limit: u32,
    /// Optional multi-column sort (server-side).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<SortParam>>,
    /// Optional structured filter (server-side).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<FilterQuery>,
    /// Optional quick-search tokens (server-side).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quick_search: Option<String>,
    /// The previous page's [`Page::next_cursor`], passed back unchanged.
    ///
    /// When it decodes for this query ([`PageCursor::from_request`]), the
    /// server continues after the previous page and ignores `offset`.
    /// Otherwise the server pages by `offset`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl PageRequest {
    /// Create a new page request with offset and limit only.
    pub fn new(offset: u32, limit: u32) -> Self {
        Self {
            offset,
            limit,
            ..Self::default()
        }
    }

    /// Attach sort, filter, and quick-search query parameters.
    pub fn with_query(
        offset: u32,
        limit: u32,
        sort: Option<Vec<SortParam>>,
        filter: Option<FilterQuery>,
        quick_search: Option<String>,
    ) -> Self {
        Self {
            offset,
            limit,
            sort,
            filter,
            quick_search,
            cursor: None,
        }
    }

    /// Set the continuation cursor from the previous page's
    /// [`Page::next_cursor`]. `None` clears it.
    #[must_use]
    pub fn with_cursor(mut self, cursor: Option<String>) -> Self {
        self.cursor = cursor;
        self
    }

    /// Convenience: is this the first page?
    pub fn is_first_page(&self) -> bool {
        self.offset == 0
    }

    /// Stable hash of everything that decides which rows this query returns
    /// and in what order: sort, filter, quick search, `limit`, and a
    /// caller-supplied `scope`.
    ///
    /// `offset` and `cursor` are left out, so every page of one query shares
    /// a fingerprint. Put anything else the result depends on in `scope`
    /// (the user, hub or parent record), so a cursor issued for one scope is
    /// rejected in another. The value is the same on every platform and
    /// release of this crate.
    pub fn query_fingerprint(&self, scope: &str) -> String {
        #[derive(Serialize)]
        struct FingerprintInput<'a> {
            scope: &'a str,
            limit: u32,
            sort: &'a Option<Vec<SortParam>>,
            filter: &'a Option<FilterQuery>,
            quick_search: &'a Option<String>,
        }
        let input = FingerprintInput {
            scope,
            limit: self.limit,
            sort: &self.sort,
            filter: &self.filter,
            quick_search: &self.quick_search,
        };
        // Serializing plain data can't fail; an empty input still hashes.
        let bytes = serde_json::to_vec(&input).unwrap_or_default();
        format!("{:016x}", cursor::fnv1a64(&bytes))
    }
}

/// A single page of results returned by a server function.
///
/// `T` is the item type (e.g. `NotificationDto`). The struct derives
/// `Serialize` and `Deserialize` so it crosses the Leptos server boundary
/// transparently.
///
/// * `has_more` — `true` when more items exist beyond this page.
/// * `total_count` — optionally present on the **first page** (`offset == 0`)
///   to give the client the full result-set size without a separate call.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Page<T> {
    /// The items in this page.
    pub items: Vec<T>,
    /// Whether additional pages exist after this one.
    pub has_more: bool,
    /// Total number of matching items (typically provided only on the first
    /// page to avoid repeated count queries).
    pub total_count: Option<u64>,
    /// When set, the next `(offset, limit)` fetch should use **this** value as
    /// `offset` instead of `accumulated_items.len()`.
    ///
    /// Use when the server applies `offset`/`limit` at the **database row**
    /// layer but returns **fewer** items after filtering (e.g. resolving a
    /// join skips some rows). Without this, the client would skip the wrong
    /// slice and can show a short first page plus a premature “end of list”.
    #[serde(default)]
    pub next_request_offset: Option<u32>,
    /// Token for continuing after this page, from [`PageCursor::encode`].
    ///
    /// Set when the server can continue from here cheaply; the client sends
    /// it back as [`PageRequest::cursor`]. `has_more` stays authoritative: a
    /// page with `has_more` and no `next_cursor` means "use `offset` for the
    /// next one".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

/// An empty page with `has_more: false` and an unknown total.
///
/// Use it with struct-update syntax (`Page { items, has_more, ..Page::default() }`)
/// so new optional fields don't break callers. [`Page::empty`] differs only in
/// reporting `total_count: Some(0)`.
impl<T> Default for Page<T> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            has_more: false,
            total_count: None,
            next_request_offset: None,
            next_cursor: None,
        }
    }
}

impl<T> Page<T> {
    /// Build a `Page` from a result set fetched with `limit + 1` rows.
    ///
    /// If `raw_items` contains more than `limit` entries the extra row is
    /// removed and `has_more` is set to `true`.
    pub fn from_oversized(mut raw_items: Vec<T>, limit: u32, total_count: Option<u64>) -> Self {
        let has_more = raw_items.len() as u32 > limit;
        if has_more {
            raw_items.truncate(limit as usize);
        }
        Self {
            items: raw_items,
            has_more,
            total_count,
            ..Self::default()
        }
    }

    /// Create an empty page (no items, no more pages).
    pub fn empty() -> Self {
        Self {
            total_count: Some(0),
            ..Self::default()
        }
    }

    /// Set the continuation token for the next request. `None` clears it.
    #[must_use]
    pub fn with_next_cursor(mut self, next_cursor: Option<String>) -> Self {
        self.next_cursor = next_cursor;
        self
    }

    /// Number of items in this page.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Whether this page contains zero items.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_oversized_trims_and_sets_has_more() {
        let items: Vec<u32> = (1..=11).collect(); // 11 items, limit 10
        let page = Page::from_oversized(items, 10, Some(25));
        assert_eq!(page.items.len(), 10);
        assert!(page.has_more);
        assert_eq!(page.total_count, Some(25));
    }

    #[test]
    fn from_oversized_no_extra_item() {
        let items: Vec<u32> = (1..=10).collect(); // exactly 10
        let page = Page::from_oversized(items, 10, None);
        assert_eq!(page.items.len(), 10);
        assert!(!page.has_more);
        assert_eq!(page.total_count, None);
    }

    #[test]
    fn page_request_first_page() {
        assert!(PageRequest::new(0, 10).is_first_page());
        assert!(!PageRequest::new(10, 10).is_first_page());
    }

    #[test]
    fn page_request_legacy_deserialize() {
        let json = r#"{"offset":0,"limit":10}"#;
        let req: PageRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.offset, 0);
        assert_eq!(req.limit, 10);
        assert!(req.sort.is_none());
        assert!(req.filter.is_none());
        assert!(req.quick_search.is_none());
        assert!(req.cursor.is_none());
    }

    #[test]
    fn page_request_without_cursor_serializes_as_before() {
        let json = serde_json::to_string(&PageRequest::new(20, 10)).unwrap();
        assert_eq!(json, r#"{"offset":20,"limit":10}"#);
    }

    #[test]
    fn page_request_cursor_round_trips() {
        let req = PageRequest::new(20, 10).with_cursor(Some("abc".into()));
        let json = serde_json::to_string(&req).unwrap();
        assert_eq!(json, r#"{"offset":20,"limit":10,"cursor":"abc"}"#);
        assert_eq!(serde_json::from_str::<PageRequest>(&json).unwrap(), req);
    }

    #[test]
    fn page_legacy_json_deserializes_without_cursor() {
        let json = r#"{"items":[1,2],"has_more":true,"total_count":null}"#;
        let page: Page<u32> = serde_json::from_str(json).unwrap();
        assert_eq!(page.items, vec![1, 2]);
        assert!(page.has_more);
        assert_eq!(page.next_request_offset, None);
        assert_eq!(page.next_cursor, None);
    }

    #[test]
    fn page_without_cursor_serializes_as_before() {
        let page = Page::from_oversized(vec![1u32, 2, 3], 2, Some(3));
        let json = serde_json::to_string(&page).unwrap();
        assert_eq!(
            json,
            r#"{"items":[1,2],"has_more":true,"total_count":3,"next_request_offset":null}"#
        );
        let with_cursor = page.with_next_cursor(Some("t".into()));
        let json = serde_json::to_string(&with_cursor).unwrap();
        assert!(json.ends_with(r#","next_cursor":"t"}"#));
        assert_eq!(
            serde_json::from_str::<Page<u32>>(&json).unwrap(),
            with_cursor
        );
    }

    #[test]
    fn fingerprint_ignores_offset_and_cursor_but_not_the_query() {
        let base = PageRequest::new(0, 10);
        let fp = base.query_fingerprint("s");
        assert_eq!(fp.len(), 16);
        assert_eq!(
            PageRequest::new(40, 10)
                .with_cursor(Some("x".into()))
                .query_fingerprint("s"),
            fp
        );
        assert_ne!(PageRequest::new(0, 20).query_fingerprint("s"), fp);
        assert_ne!(base.query_fingerprint("t"), fp);
        let mut searched = base.clone();
        searched.quick_search = Some("a".into());
        assert_ne!(searched.query_fingerprint("s"), fp);
        let mut sorted = base;
        sorted.sort = Some(vec![SortParam {
            field: "name".into(),
            direction: SortDirectionWire::Asc,
        }]);
        assert_ne!(sorted.query_fingerprint("s"), fp);
    }

    #[test]
    fn empty_page() {
        let page: Page<String> = Page::empty();
        assert!(page.is_empty());
        assert!(!page.has_more);
        assert_eq!(page.total_count, Some(0));
    }
}
