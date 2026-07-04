/// How the footer labels paged row counts.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PaginationDisplayFormat {
    /// Locale-aware range label (`1–10 of 100`).
    #[default]
    Locale,
    /// Legacy total-only label via [`DataTableLocale::footer_rows`].
    Plain,
}

/// Localized strings for DataTable chrome (footer, overlays, infinite scroll).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DataTableLocale {
    /// Footer row count format. Use `{count}` as placeholder.
    pub footer_rows: String,
    /// Paged footer range format. Placeholders: `{from}`, `{to}`, `{total}`.
    pub pagination_range: String,
    /// Page-size selector label in the footer.
    pub rows_per_page: String,
    /// Shown when the dataset has no rows at all.
    pub no_rows: String,
    /// Shown when filters/search yield zero rows.
    pub no_results: String,
    /// Loading overlay label.
    pub loading: String,
    /// Infinite scroll end-of-list message.
    pub infinite_end: String,
    /// Quick search input placeholder.
    pub quick_search_placeholder: String,
}

impl Default for DataTableLocale {
    fn default() -> Self {
        Self {
            footer_rows: "{count} rows".to_string(),
            pagination_range: "{from}–{to} of {total}".to_string(),
            rows_per_page: "Rows per page".to_string(),
            no_rows: "No rows".to_string(),
            no_results: "No rows match your filters.".to_string(),
            loading: "Loading".to_string(),
            infinite_end: "End of list".to_string(),
            quick_search_placeholder: "Search".to_string(),
        }
    }
}

impl DataTableLocale {
    /// Format the footer row count string.
    pub fn format_footer_rows(&self, count: usize) -> String {
        self.footer_rows.replace("{count}", &count.to_string())
    }

    /// Format the paged footer range label.
    ///
    /// When `estimated` is true (server total unknown), `{total}` is prefixed with
    /// `"more than "` before substitution.
    pub fn format_pagination_range(
        &self,
        from: usize,
        to: usize,
        total: usize,
        estimated: bool,
    ) -> String {
        let total_str = if estimated {
            format!("more than {total}")
        } else {
            total.to_string()
        };
        self.pagination_range
            .replace("{from}", &from.to_string())
            .replace("{to}", &to.to_string())
            .replace("{total}", &total_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_pagination_range_default() {
        let locale = DataTableLocale::default();
        assert_eq!(
            locale.format_pagination_range(1, 10, 100, false),
            "1–10 of 100"
        );
    }

    #[test]
    fn format_pagination_range_estimated() {
        let locale = DataTableLocale::default();
        assert_eq!(
            locale.format_pagination_range(1, 5, 5, true),
            "1–5 of more than 5"
        );
    }

    #[test]
    fn format_pagination_range_custom_template() {
        let locale = DataTableLocale {
            pagination_range: "{from} à {to} sur {total}".into(),
            ..Default::default()
        };
        assert_eq!(
            locale.format_pagination_range(1, 10, 100, false),
            "1 à 10 sur 100"
        );
    }
}
