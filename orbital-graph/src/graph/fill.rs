//! Node fills: six theme-token triples with stable wire values.

/// Fill of a graph node: picks the background, border and text tokens.
///
/// Apps store [`GraphFill::wire_value`] in their own settings (colour rules,
/// say), so the wire values never change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GraphFill {
    /// Subtle surface with the default border.
    #[default]
    Neutral,
    /// Brand background with on-brand text.
    Brand,
    /// Red palette.
    Red,
    /// Orange palette.
    Orange,
    /// Yellow palette, on its subtle background so dark text stays readable.
    Yellow,
    /// Green palette.
    Green,
}

/// Background, border and text CSS values for one fill.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FillTokens {
    /// Box background.
    pub background: &'static str,
    /// Box border.
    pub border: &'static str,
    /// Text colour.
    pub text: &'static str,
}

impl GraphFill {
    /// Every fill, in palette order.
    pub const ALL: [GraphFill; 6] = [
        GraphFill::Neutral,
        GraphFill::Brand,
        GraphFill::Red,
        GraphFill::Orange,
        GraphFill::Yellow,
        GraphFill::Green,
    ];

    /// Stable string for storage: `neutral`, `brand`, `red`, `orange`,
    /// `yellow` or `green`.
    #[must_use]
    pub fn wire_value(self) -> &'static str {
        match self {
            GraphFill::Neutral => "neutral",
            GraphFill::Brand => "brand",
            GraphFill::Red => "red",
            GraphFill::Orange => "orange",
            GraphFill::Yellow => "yellow",
            GraphFill::Green => "green",
        }
    }

    /// Parses a [`wire_value`](Self::wire_value); `None` for anything else.
    #[must_use]
    pub fn from_wire_value(value: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|fill| fill.wire_value() == value)
    }

    /// The theme tokens this fill draws with.
    #[must_use]
    pub fn tokens(self) -> FillTokens {
        let tokens = |background, border, text| FillTokens {
            background,
            border,
            text,
        };
        match self {
            GraphFill::Neutral => tokens(
                "var(--orb-color-surface-subtle)",
                "var(--orb-color-border-default)",
                "var(--orb-color-text-primary)",
            ),
            GraphFill::Brand => tokens(
                "var(--orb-color-brand-bg)",
                "var(--orb-color-brand-stroke)",
                "var(--orb-color-text-on-brand)",
            ),
            GraphFill::Red => tokens(
                "var(--orb-color-palette-red-bg)",
                "var(--orb-color-palette-red-border)",
                "var(--orb-color-palette-red-fg)",
            ),
            GraphFill::Orange => tokens(
                "var(--orb-color-palette-orange-bg)",
                "var(--orb-color-palette-orange-border)",
                "var(--orb-color-palette-orange-fg)",
            ),
            GraphFill::Yellow => tokens(
                "var(--orb-color-palette-yellow-bg-subtle)",
                "var(--orb-color-palette-yellow-border)",
                "var(--orb-color-palette-yellow-fg)",
            ),
            GraphFill::Green => tokens(
                "var(--orb-color-palette-green-bg)",
                "var(--orb-color-palette-green-border)",
                "var(--orb-color-palette-green-fg)",
            ),
        }
    }

    /// Inline style that sets the node's custom properties for this fill.
    pub(crate) fn style_vars(self) -> String {
        let t = self.tokens();
        format!(
            "--orbital-graph-bg: {}; --orbital-graph-border: {}; --orbital-graph-text: {};",
            t.background, t.border, t.text
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fill_wire_values_round_trip_happy() {
        let wire: Vec<_> = GraphFill::ALL.iter().map(|f| f.wire_value()).collect();
        assert_eq!(
            wire,
            ["neutral", "brand", "red", "orange", "yellow", "green"]
        );
        for fill in GraphFill::ALL {
            assert_eq!(GraphFill::from_wire_value(fill.wire_value()), Some(fill));
        }
    }

    #[test]
    fn fill_unknown_wire_value_none_sad() {
        assert_eq!(GraphFill::from_wire_value("purple"), None);
        assert_eq!(GraphFill::from_wire_value("Red"), None);
        assert_eq!(GraphFill::from_wire_value(""), None);
    }

    #[test]
    fn fill_tokens_use_theme_vars_only_happy() {
        for fill in GraphFill::ALL {
            let t = fill.tokens();
            for value in [t.background, t.border, t.text] {
                assert!(value.starts_with("var(--orb-color-"), "{value}");
            }
        }
        assert!(GraphFill::Yellow.tokens().background.contains("bg-subtle"));
    }
}
