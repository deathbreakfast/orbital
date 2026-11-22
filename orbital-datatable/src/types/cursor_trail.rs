use std::collections::HashMap;

/// Cursors that lead to each server page the table has loaded, keyed by offset.
///
/// Server mode keeps one of these next to
/// [`server_offset`](crate::types::DataTableTableState::server_offset). When a
/// page at offset `o` returns [`Page::next_cursor`](orbital_paging::Page::next_cursor),
/// the table records it under `o + limit`, the next page's offset. A fetch
/// then sends the cursor recorded for its offset:
///
/// * **Next page:** the cursor the current page returned.
/// * **Previous page:** the cursor recorded when that page was first reached
///   (none for page 1, which pages from offset 0).
/// * **Jump to a page never reached by Next:** no cursor, so the server
///   pages by offset.
///
/// The trail belongs to one query. A different query key (sort, filter,
/// quick search or page size, via
/// [`PageRequest::query_fingerprint`](orbital_paging::PageRequest::query_fingerprint))
/// or a new refresh generation clears it, so a cursor is never sent with a
/// query it didn't come from.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CursorTrail {
    query: Option<(String, u32)>,
    cursors: HashMap<u32, String>,
}

impl CursorTrail {
    /// Cursor to send for `offset` under this query and refresh generation.
    ///
    /// Clears the trail first when the query or refresh generation changed.
    pub fn cursor_for(&mut self, query_key: &str, refresh: u32, offset: u32) -> Option<String> {
        self.align(query_key, refresh);
        if offset == 0 {
            return None;
        }
        self.cursors.get(&offset).cloned()
    }

    /// Record the cursor a page returned, under the next page's offset.
    ///
    /// `None` forgets any cursor recorded there, so a later fetch at that
    /// offset pages by offset.
    pub fn record(
        &mut self,
        query_key: &str,
        refresh: u32,
        next_offset: u32,
        cursor: Option<String>,
    ) {
        self.align(query_key, refresh);
        match cursor {
            Some(token) => {
                self.cursors.insert(next_offset, token);
            }
            None => {
                self.cursors.remove(&next_offset);
            }
        }
    }

    /// Forget every recorded cursor.
    pub fn clear(&mut self) {
        self.query = None;
        self.cursors.clear();
    }

    /// Number of recorded cursors.
    pub fn len(&self) -> usize {
        self.cursors.len()
    }

    /// Whether no cursor is recorded.
    pub fn is_empty(&self) -> bool {
        self.cursors.is_empty()
    }

    fn align(&mut self, query_key: &str, refresh: u32) {
        let same = self
            .query
            .as_ref()
            .is_some_and(|(key, gen)| key == query_key && *gen == refresh);
        if !same {
            self.cursors.clear();
            self.query = Some((query_key.to_string(), refresh));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const Q: &str = "query-a";

    /// Walk pages 1→2→3 with Next, as the table does after each response.
    fn walked_trail() -> CursorTrail {
        let mut trail = CursorTrail::default();
        assert_eq!(trail.cursor_for(Q, 0, 0), None);
        trail.record(Q, 0, 10, Some("c10".into()));
        assert_eq!(trail.cursor_for(Q, 0, 10), Some("c10".into()));
        trail.record(Q, 0, 20, Some("c20".into()));
        assert_eq!(trail.cursor_for(Q, 0, 20), Some("c20".into()));
        trail.record(Q, 0, 30, Some("c30".into()));
        trail
    }

    #[test]
    fn next_page_carries_the_cursor_the_current_page_returned() {
        let mut trail = walked_trail();
        assert_eq!(trail.cursor_for(Q, 0, 30), Some("c30".into()));
    }

    #[test]
    fn previous_page_reuses_the_recorded_cursor_and_page_one_uses_offset() {
        let mut trail = walked_trail();
        assert_eq!(trail.cursor_for(Q, 0, 10), Some("c10".into()));
        assert_eq!(trail.cursor_for(Q, 0, 0), None);
    }

    #[test]
    fn jump_to_an_unvisited_page_sends_no_cursor() {
        let mut trail = walked_trail();
        assert_eq!(trail.cursor_for(Q, 0, 70), None);
        assert_eq!(trail.len(), 3);
    }

    #[test]
    fn query_change_drops_back_to_offset() {
        let mut trail = walked_trail();
        assert_eq!(trail.cursor_for("query-b", 0, 20), None);
        assert!(trail.is_empty());
        // Returning to the old query doesn't resurrect its cursors.
        assert_eq!(trail.cursor_for(Q, 0, 20), None);
    }

    #[test]
    fn refresh_drops_back_to_offset() {
        let mut trail = walked_trail();
        assert_eq!(trail.cursor_for(Q, 1, 20), None);
        assert!(trail.is_empty());
    }

    #[test]
    fn page_without_next_cursor_forgets_a_stale_entry() {
        let mut trail = walked_trail();
        trail.record(Q, 0, 20, None);
        assert_eq!(trail.cursor_for(Q, 0, 20), None);
    }

    #[test]
    fn clear_forgets_everything() {
        let mut trail = walked_trail();
        trail.clear();
        assert!(trail.is_empty());
        assert_eq!(trail.cursor_for(Q, 0, 10), None);
    }
}
