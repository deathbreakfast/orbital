//! Opaque continuation tokens for [`PageRequest::cursor`](crate::PageRequest::cursor)
//! and [`Page::next_cursor`](crate::Page::next_cursor).
//!
//! A server that can continue a list from where the previous page stopped
//! (a Valence resume position, a keyset such as `(created_at, id)`) encodes
//! that position in a [`PageCursor`] and returns it as `next_cursor`. The
//! client passes the string back unchanged on the next request. The token
//! also carries a fingerprint of the query that produced it, so a cursor
//! from a different sort, filter, quick search, page size or scope decodes
//! to `None` and the server falls back to `offset`.
//!
//! A cursor is client-controlled input. The fingerprint detects a stale or
//! mismatched cursor; it is not a signature and grants no access. Servers
//! still apply read privacy and must validate the decoded position and key
//! before using them.

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine as _;
use serde::{Deserialize, Serialize};

use crate::PageRequest;

/// Current token format version. Tokens with any other version decode to `None`.
const CURSOR_VERSION: u8 = 1;

/// Largest token [`PageCursor::decode`] will look at, in bytes.
///
/// Keeps a hostile client from making the server base64-decode and parse
/// an arbitrarily large string.
pub const MAX_CURSOR_TOKEN_LEN: usize = 2048;

/// A decoded continuation position.
///
/// What `position` and `key` mean is up to the server that issued the
/// cursor; the client never looks inside the token.
///
/// * `position` — a numeric position, such as a Valence resume position
///   (`QueryStats::resume`) or a row offset.
/// * `key` — a keyset that doesn't fit in a number, such as
///   `"<created_at>|<id>"`. `None` when `position` is enough.
/// * `fingerprint` — from [`PageRequest::query_fingerprint`] for the request
///   that produced this page.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageCursor {
    /// Server-defined numeric position.
    pub position: u64,
    /// Server-defined keyset, when a number isn't enough.
    pub key: Option<String>,
    /// Fingerprint of the query this cursor belongs to.
    pub fingerprint: String,
}

#[derive(Serialize, Deserialize)]
struct CursorWire {
    v: u8,
    p: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    k: Option<String>,
    f: String,
}

impl PageCursor {
    /// Cursor holding a numeric position only.
    pub fn new(position: u64, fingerprint: impl Into<String>) -> Self {
        Self {
            position,
            key: None,
            fingerprint: fingerprint.into(),
        }
    }

    /// Cursor holding a keyset string (and a position, often `0`).
    pub fn with_key(position: u64, key: impl Into<String>, fingerprint: impl Into<String>) -> Self {
        Self {
            position,
            key: Some(key.into()),
            fingerprint: fingerprint.into(),
        }
    }

    /// Encode as a URL-safe token for [`Page::next_cursor`](crate::Page::next_cursor).
    pub fn encode(&self) -> String {
        let wire = CursorWire {
            v: CURSOR_VERSION,
            p: self.position,
            k: self.key.clone(),
            f: self.fingerprint.clone(),
        };
        // Serializing a struct of integers and strings can't fail.
        let json = serde_json::to_vec(&wire).unwrap_or_default();
        URL_SAFE_NO_PAD.encode(json)
    }

    /// Decode a token from [`PageRequest::cursor`](crate::PageRequest::cursor).
    ///
    /// Returns `None` when the token is too long, isn't valid base64 or
    /// JSON, has an unknown version, or its fingerprint differs from
    /// `expected_fingerprint`. Callers treat `None` as "no cursor" and page
    /// by `offset`.
    pub fn decode(token: &str, expected_fingerprint: &str) -> Option<Self> {
        if token.is_empty() || token.len() > MAX_CURSOR_TOKEN_LEN {
            return None;
        }
        let bytes = URL_SAFE_NO_PAD.decode(token.as_bytes()).ok()?;
        let wire: CursorWire = serde_json::from_slice(&bytes).ok()?;
        if wire.v != CURSOR_VERSION || wire.f != expected_fingerprint {
            return None;
        }
        Some(Self {
            position: wire.p,
            key: wire.k,
            fingerprint: wire.f,
        })
    }

    /// Decode `request.cursor` against `request.query_fingerprint(scope)`.
    ///
    /// Shorthand for the common server path: `None` when the request has no
    /// cursor or the cursor doesn't belong to this query.
    pub fn from_request(request: &PageRequest, scope: &str) -> Option<Self> {
        let token = request.cursor.as_deref()?;
        Self::decode(token, &request.query_fingerprint(scope))
    }
}

/// FNV-1a, 64-bit. Stable across platforms and releases, unlike
/// `std::hash::DefaultHasher`, which is all a fingerprint needs.
pub(crate) fn fnv1a64(bytes: &[u8]) -> u64 {
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01b3;
    bytes.iter().fold(OFFSET, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(PRIME)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FilterLogicWire, FilterQuery, FilterRuleParam, SortDirectionWire, SortParam};
    use orbital_data::DataValue;

    fn filtered_request() -> PageRequest {
        PageRequest::with_query(
            0,
            25,
            Some(vec![SortParam {
                field: "created".into(),
                direction: SortDirectionWire::Desc,
            }]),
            Some(FilterQuery {
                items: vec![FilterRuleParam {
                    field: "status".into(),
                    operator: "is".into(),
                    value: DataValue::Text("open".into()),
                }],
                logic: FilterLogicWire::And,
            }),
            Some("alpha".into()),
        )
    }

    #[test]
    fn cursor_round_trip_position_only() {
        let fp = filtered_request().query_fingerprint("hub:1");
        let cursor = PageCursor::new(4_200, fp.clone());
        let token = cursor.encode();
        assert!(token
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_'));
        assert_eq!(PageCursor::decode(&token, &fp), Some(cursor));
    }

    #[test]
    fn cursor_round_trip_with_key() {
        let fp = filtered_request().query_fingerprint("");
        let cursor = PageCursor::with_key(0, "2026-11-13T00:00:00Z|run-42", fp.clone());
        assert_eq!(PageCursor::decode(&cursor.encode(), &fp), Some(cursor));
    }

    #[test]
    fn cursor_fingerprint_mismatch_is_rejected() {
        let req = filtered_request();
        let token = PageCursor::new(7, req.query_fingerprint("hub:1")).encode();
        assert_eq!(
            PageCursor::decode(&token, &req.query_fingerprint("hub:2")),
            None
        );
    }

    #[test]
    fn cursor_malformed_tokens_are_rejected() {
        let fp = filtered_request().query_fingerprint("");
        for token in ["", "!!!", "not base64 at all", "e30", "bnVsbA"] {
            assert_eq!(PageCursor::decode(token, &fp), None, "token {token:?}");
        }
        let too_long = "A".repeat(MAX_CURSOR_TOKEN_LEN + 1);
        assert_eq!(PageCursor::decode(&too_long, &fp), None);
    }

    #[test]
    fn cursor_unknown_version_is_rejected() {
        let fp = filtered_request().query_fingerprint("");
        for v in [0u8, 2] {
            let json = serde_json::to_vec(&CursorWire {
                v,
                p: 1,
                k: None,
                f: fp.clone(),
            })
            .unwrap();
            let token = URL_SAFE_NO_PAD.encode(json);
            assert_eq!(PageCursor::decode(&token, &fp), None, "version {v}");
        }
    }

    #[test]
    fn from_request_uses_the_request_fingerprint() {
        let base = filtered_request();
        let token = PageCursor::new(9, base.query_fingerprint("s")).encode();
        let next = PageRequest {
            offset: 25,
            ..base.clone()
        }
        .with_cursor(Some(token));
        assert_eq!(
            PageCursor::from_request(&next, "s").map(|c| c.position),
            Some(9)
        );
        let mut other_search = next.clone();
        other_search.quick_search = Some("beta".into());
        assert_eq!(PageCursor::from_request(&other_search, "s"), None);
        assert_eq!(PageCursor::from_request(&base, "s"), None);
    }

    #[test]
    fn fnv1a64_matches_reference_vectors() {
        assert_eq!(fnv1a64(b""), 0xcbf2_9ce4_8422_2325);
        assert_eq!(fnv1a64(b"a"), 0xaf63_dc4c_8601_ec8c);
        assert_eq!(fnv1a64(b"foobar"), 0x8594_4171_f739_67e8);
    }
}
