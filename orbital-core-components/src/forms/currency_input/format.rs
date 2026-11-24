/// ISO 4217 currencies [`CurrencyInput`](super::currency_input::CurrencyInput) knows.
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub enum CurrencyCode {
    #[default]
    USD,
    EUR,
    GBP,
    JPY,
    CHF,
    INR,
    BRL,
}

impl CurrencyCode {
    pub const ALL: [CurrencyCode; 7] = [
        Self::USD,
        Self::EUR,
        Self::GBP,
        Self::JPY,
        Self::CHF,
        Self::INR,
        Self::BRL,
    ];

    /// Three-letter ISO code.
    pub fn code(&self) -> &'static str {
        match self {
            Self::USD => "USD",
            Self::EUR => "EUR",
            Self::GBP => "GBP",
            Self::JPY => "JPY",
            Self::CHF => "CHF",
            Self::INR => "INR",
            Self::BRL => "BRL",
        }
    }

    /// Symbol drawn beside the amount.
    pub fn symbol(&self) -> &'static str {
        match self {
            Self::USD => "$",
            Self::EUR => "€",
            Self::GBP => "£",
            Self::JPY => "¥",
            Self::CHF => "CHF",
            Self::INR => "₹",
            Self::BRL => "R$",
        }
    }

    /// Digits after the decimal mark (the currency's minor unit).
    pub fn minor_digits(&self) -> usize {
        match self {
            Self::JPY => 0,
            _ => 2,
        }
    }

    /// Parses an ISO code, ignoring case.
    pub fn from_code(code: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|c| c.code().eq_ignore_ascii_case(code))
    }
}

/// Locales with their own grouping, decimal mark and symbol side.
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub enum CurrencyLocale {
    #[default]
    EnUs,
    EnGb,
    DeDe,
    FrFr,
    DeCh,
    PtBr,
    JaJp,
    EnIn,
}

/// Which side of the amount the symbol sits on.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SymbolSide {
    Prefix,
    Suffix,
}

impl CurrencyLocale {
    pub const ALL: [CurrencyLocale; 8] = [
        Self::EnUs,
        Self::EnGb,
        Self::DeDe,
        Self::FrFr,
        Self::DeCh,
        Self::PtBr,
        Self::JaJp,
        Self::EnIn,
    ];

    /// BCP 47 tag, such as `de-DE`.
    pub fn tag(&self) -> &'static str {
        match self {
            Self::EnUs => "en-US",
            Self::EnGb => "en-GB",
            Self::DeDe => "de-DE",
            Self::FrFr => "fr-FR",
            Self::DeCh => "de-CH",
            Self::PtBr => "pt-BR",
            Self::JaJp => "ja-JP",
            Self::EnIn => "en-IN",
        }
    }

    /// Looks up a tag (`de-DE`, `de_de`); a bare language picks its main region and
    /// anything unknown falls back to `en-US`.
    pub fn from_tag(tag: &str) -> Self {
        let norm = tag.replace('_', "-").to_ascii_lowercase();
        if let Some(found) = Self::ALL
            .into_iter()
            .find(|l| l.tag().eq_ignore_ascii_case(&norm))
        {
            return found;
        }
        match norm.split('-').next().unwrap_or("") {
            "en" => Self::EnUs,
            "de" => Self::DeDe,
            "fr" => Self::FrFr,
            "pt" => Self::PtBr,
            "ja" => Self::JaJp,
            _ => Self::EnUs,
        }
    }

    pub fn group_separator(&self) -> char {
        match self {
            Self::EnUs | Self::EnGb | Self::JaJp | Self::EnIn => ',',
            Self::DeDe | Self::PtBr => '.',
            Self::FrFr => '\u{202F}',
            Self::DeCh => '’',
        }
    }

    pub fn decimal_mark(&self) -> char {
        match self {
            Self::EnUs | Self::EnGb | Self::JaJp | Self::EnIn | Self::DeCh => '.',
            Self::DeDe | Self::PtBr | Self::FrFr => ',',
        }
    }

    pub fn symbol_side(&self) -> SymbolSide {
        match self {
            Self::DeDe | Self::FrFr => SymbolSide::Suffix,
            _ => SymbolSide::Prefix,
        }
    }

    /// Indian grouping: three digits, then twos.
    fn lakh_grouping(&self) -> bool {
        matches!(self, Self::EnIn)
    }
}

fn group_digits(whole: &str, locale: CurrencyLocale) -> String {
    let digits: Vec<char> = whole.chars().collect();
    let mut out = String::new();
    for (i, c) in digits.iter().enumerate() {
        let from_end = digits.len() - i;
        if i > 0 {
            let boundary = if locale.lakh_grouping() {
                from_end == 3 || (from_end > 3 && (from_end - 3) % 2 == 0)
            } else {
                from_end % 3 == 0
            };
            if boundary {
                out.push(locale.group_separator());
            }
        }
        out.push(*c);
    }
    out
}

/// Splits canonical text (`-`, digits, `.`) into sign, whole and fraction digits.
fn split_canonical(canonical: &str) -> (bool, &str, &str) {
    let (neg, rest) = match canonical.strip_prefix('-') {
        Some(r) => (true, r),
        None => (false, canonical),
    };
    match rest.split_once('.') {
        Some((w, f)) => (neg, w, f),
        None => (neg, rest, ""),
    }
}

/// Rounds half up to `digits` fraction digits; returns whole and fraction digits.
fn round_digits(whole: &str, frac: &str, digits: usize) -> (String, String) {
    let whole = if whole.is_empty() { "0" } else { whole };
    if frac.len() <= digits {
        return (whole.to_string(), format!("{frac:0<digits$}"));
    }
    let round_up = frac.as_bytes()[digits] >= b'5';
    let mut all: Vec<u8> = whole
        .bytes()
        .chain(frac.bytes().take(digits))
        .map(|b| b - b'0')
        .collect();
    if round_up {
        let mut i = all.len();
        loop {
            if i == 0 {
                all.insert(0, 1);
                break;
            }
            i -= 1;
            if all[i] == 9 {
                all[i] = 0;
            } else {
                all[i] += 1;
                break;
            }
        }
    }
    let text: String = all.iter().map(|d| (d + b'0') as char).collect();
    let split = text.len() - digits;
    (text[..split].to_string(), text[split..].to_string())
}

/// Display text for a canonical amount (`1234.5`): grouped, padded to the
/// currency's minor digits, rounded half up. The symbol is not included.
/// Empty or unreadable input gives an empty string.
pub fn format_amount(canonical: &str, locale: CurrencyLocale, currency: CurrencyCode) -> String {
    if canonical.is_empty() || canonical == "-" {
        return String::new();
    }
    let (neg, whole, frac) = split_canonical(canonical);
    if !whole.chars().all(|c| c.is_ascii_digit()) || !frac.chars().all(|c| c.is_ascii_digit()) {
        return String::new();
    }
    let digits = currency.minor_digits();
    let (whole, frac) = round_digits(whole, frac, digits);
    let mut out = String::new();
    if neg {
        out.push('-');
    }
    out.push_str(&group_digits(&whole, locale));
    if digits > 0 {
        out.push(locale.decimal_mark());
        out.push_str(&frac);
    }
    out
}

/// Rounds a canonical amount half up to the currency's minor digits, dropping a
/// fraction that rounds to nothing (`1234.56` in JPY becomes `1235`).
pub fn round_canonical(canonical: &str, currency: CurrencyCode) -> String {
    let (neg, whole, frac) = split_canonical(canonical);
    let digits = currency.minor_digits();
    if frac.len() <= digits
        || !whole.chars().all(|c| c.is_ascii_digit())
        || !frac.chars().all(|c| c.is_ascii_digit())
    {
        return canonical.to_string();
    }
    let (w, f) = round_digits(whole, frac, digits);
    let sign = if neg { "-" } else { "" };
    if digits == 0 {
        format!("{sign}{w}")
    } else {
        format!("{sign}{w}.{f}")
    }
}

/// Plain editing text for a canonical amount: the locale's decimal mark, no grouping.
pub fn edit_text(canonical: &str, locale: CurrencyLocale) -> String {
    canonical.replace('.', &locale.decimal_mark().to_string())
}

/// Result of reading typed or pasted text.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ParsedAmount {
    /// Plain text to show while editing (locale decimal mark, no grouping).
    pub edit: String,
    /// Canonical value: `-`, digits and `.`; empty while there is no number.
    pub canonical: String,
}

/// Normalises typed or pasted text. Drops the currency symbol and code, spaces and the
/// locale's group separator, then accepts an optional leading minus (when
/// `allow_negative`), digits and the locale's decimal mark with at most the currency's
/// minor digits after it. `None` rejects the edit.
pub fn parse_amount(
    raw: &str,
    locale: CurrencyLocale,
    currency: CurrencyCode,
    allow_negative: bool,
) -> Option<ParsedAmount> {
    let mut text = raw.replace(currency.symbol(), "");
    let code = currency.code();
    if let Some(at) = text.to_ascii_uppercase().find(code) {
        text.replace_range(at..at + code.len(), "");
    }
    let group = locale.group_separator();
    let decimal = locale.decimal_mark();
    let cleaned: String = text
        .chars()
        .filter(|c| {
            !c.is_whitespace()
                && *c != group
                && !matches!(c, '$' | '€' | '£' | '¥' | '₹' | '\'' | '’')
                || *c == decimal
        })
        .collect();
    let (neg, digits) = match cleaned.strip_prefix('-') {
        Some(rest) if allow_negative => (true, rest),
        _ => (false, cleaned.as_str()),
    };
    let (whole, frac) = match digits.split_once(decimal) {
        Some((w, f)) => (w, Some(f)),
        None => (digits, None),
    };
    let max_frac = currency.minor_digits();
    let whole_ok = whole.chars().all(|c| c.is_ascii_digit());
    let frac_ok = frac.is_none_or(|f| {
        max_frac > 0 && f.len() <= max_frac && f.chars().all(|c| c.is_ascii_digit())
    });
    if !whole_ok || !frac_ok {
        return None;
    }
    let sign = if neg { "-" } else { "" };
    let mark = if frac.is_some() {
        decimal.to_string()
    } else {
        String::new()
    };
    let edit = format!("{sign}{whole}{mark}{}", frac.unwrap_or(""));
    let canonical = if whole.is_empty() && frac.is_none_or(str::is_empty) {
        String::new()
    } else {
        let w = if whole.is_empty() { "0" } else { whole };
        match frac {
            Some(f) if !f.is_empty() => format!("{sign}{w}.{f}"),
            _ => format!("{sign}{w}"),
        }
    };
    Some(ParsedAmount { edit, canonical })
}

#[cfg(test)]
mod tests {
    use super::*;
    use CurrencyCode::*;
    use CurrencyLocale::*;

    #[test]
    fn formats_per_locale() {
        assert_eq!(format_amount("1234.56", EnUs, USD), "1,234.56");
        assert_eq!(format_amount("1234.56", DeDe, EUR), "1.234,56");
        assert_eq!(format_amount("1234.56", FrFr, EUR), "1\u{202F}234,56");
        assert_eq!(format_amount("1234.56", DeCh, CHF), "1’234.56");
        assert_eq!(format_amount("1234.56", PtBr, BRL), "1.234,56");
        assert_eq!(format_amount("1234567", EnIn, INR), "12,34,567.00");
        assert_eq!(format_amount("1234.5", EnUs, USD), "1,234.50");
        assert_eq!(format_amount("-1234.5", EnUs, USD), "-1,234.50");
        assert_eq!(format_amount("", EnUs, USD), "");
    }

    #[test]
    fn jpy_has_no_minor_unit_and_rounds_half_up() {
        assert_eq!(format_amount("1234.56", JaJp, JPY), "1,235");
        assert_eq!(format_amount("999.5", JaJp, JPY), "1,000");
        assert_eq!(format_amount("0.4", JaJp, JPY), "0");
    }

    #[test]
    fn rounds_canonical_to_minor_digits() {
        assert_eq!(round_canonical("1234.56", JPY), "1235");
        assert_eq!(round_canonical("1234.5", USD), "1234.5");
        assert_eq!(round_canonical("1.005", USD), "1.01");
    }

    #[test]
    fn rounds_carry_into_whole() {
        assert_eq!(format_amount("0.999", EnUs, USD), "1.00");
        assert_eq!(format_amount("99.995", EnUs, USD), "100.00");
    }

    #[test]
    fn symbol_side_follows_locale() {
        assert_eq!(DeDe.symbol_side(), SymbolSide::Suffix);
        assert_eq!(FrFr.symbol_side(), SymbolSide::Suffix);
        assert_eq!(EnUs.symbol_side(), SymbolSide::Prefix);
        assert_eq!(PtBr.symbol_side(), SymbolSide::Prefix);
    }

    #[test]
    fn locale_tags_fall_back() {
        assert_eq!(CurrencyLocale::from_tag("de_de"), DeDe);
        assert_eq!(CurrencyLocale::from_tag("de-AT"), DeDe);
        assert_eq!(CurrencyLocale::from_tag("xx-YY"), EnUs);
    }

    #[test]
    fn parses_typed_text() {
        let p = parse_amount("1234.5", EnUs, USD, false).unwrap();
        assert_eq!(
            (p.edit.as_str(), p.canonical.as_str()),
            ("1234.5", "1234.5")
        );
        let p = parse_amount("1234,5", DeDe, EUR, false).unwrap();
        assert_eq!(
            (p.edit.as_str(), p.canonical.as_str()),
            ("1234,5", "1234.5")
        );
        let p = parse_amount("12,", DeDe, EUR, false).unwrap();
        assert_eq!((p.edit.as_str(), p.canonical.as_str()), ("12,", "12"));
        let p = parse_amount("", EnUs, USD, false).unwrap();
        assert_eq!(p.canonical, "");
        let p = parse_amount(".5", EnUs, USD, false).unwrap();
        assert_eq!(p.canonical, "0.5");
    }

    #[test]
    fn normalises_pasted_text() {
        let p = parse_amount("$1,234.56", EnUs, USD, false).unwrap();
        assert_eq!(
            (p.edit.as_str(), p.canonical.as_str()),
            ("1234.56", "1234.56")
        );
        let p = parse_amount("€ 1.234,56", DeDe, EUR, false).unwrap();
        assert_eq!(p.canonical, "1234.56");
        let p = parse_amount("USD 12.5", EnUs, USD, false).unwrap();
        assert_eq!(p.canonical, "12.5");
        let p = parse_amount("1\u{202F}234,56 €", FrFr, EUR, false).unwrap();
        assert_eq!(p.canonical, "1234.56");
        let p = parse_amount("R$ 1.234,56", PtBr, BRL, false).unwrap();
        assert_eq!(p.canonical, "1234.56");
    }

    #[test]
    fn rejects_unreadable_edits() {
        assert!(parse_amount("12.345", EnUs, USD, false).is_none());
        assert!(parse_amount("abc", EnUs, USD, false).is_none());
        assert!(parse_amount("1.2.3", EnUs, USD, false).is_none());
        assert!(parse_amount("-5", EnUs, USD, false).is_none());
        assert!(parse_amount("12.5", JaJp, JPY, false).is_none());
        assert!(parse_amount("12.", JaJp, JPY, false).is_none());
    }

    #[test]
    fn minus_when_allowed() {
        let p = parse_amount("-5.25", EnUs, USD, true).unwrap();
        assert_eq!(p.canonical, "-5.25");
        let p = parse_amount("-", EnUs, USD, true).unwrap();
        assert_eq!((p.edit.as_str(), p.canonical.as_str()), ("-", ""));
        assert!(parse_amount("--5", EnUs, USD, true).is_none());
    }
}
