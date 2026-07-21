use orbital_base_components::DatetimeFormat;

/// Logical segment within a segmented datetime field.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SegmentKind {
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Meridiem,
}

impl SegmentKind {
    pub fn label(self) -> &'static str {
        match self {
            SegmentKind::Year => "Year",
            SegmentKind::Month => "Month",
            SegmentKind::Day => "Day",
            SegmentKind::Hour => "Hour",
            SegmentKind::Minute => "Minute",
            SegmentKind::Meridiem => "Meridiem",
        }
    }

    pub fn aria_value(self, text: &str) -> Option<i32> {
        match self {
            SegmentKind::Meridiem => None,
            _ => text.parse().ok(),
        }
    }

    pub fn testid_suffix(self) -> &'static str {
        match self {
            SegmentKind::Year => "year",
            SegmentKind::Month => "month",
            SegmentKind::Day => "day",
            SegmentKind::Hour => "hour",
            SegmentKind::Minute => "minute",
            SegmentKind::Meridiem => "meridiem",
        }
    }

    /// BEM modifier suffix for segment styling (`orb-date-field__segment--year`, etc.).
    pub fn modifier(self) -> &'static str {
        self.testid_suffix()
    }
}

/// Layout metadata for one editable segment.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SegmentSpec {
    pub kind: SegmentKind,
    pub max_len: usize,
    pub width_ch: usize,
    pub placeholder: &'static str,
    /// Literal separator rendered after this segment (`None` for the last segment).
    pub separator: Option<&'static str>,
}

/// Ordered segment layout for a [`DatetimeFormat`].
pub fn segment_specs(format: DatetimeFormat) -> &'static [SegmentSpec] {
    match format {
        DatetimeFormat::IsoDate => &ISO_DATE,
        DatetimeFormat::UsDate => &US_DATE,
        DatetimeFormat::Time24 => &TIME_24,
        DatetimeFormat::Time12 => &TIME_12,
    }
}

/// Combined date + time segment layout from separate date and time formats.
pub fn combined_segment_specs(
    date_format: DatetimeFormat,
    time_format: DatetimeFormat,
) -> Vec<SegmentSpec> {
    let date_specs = segment_specs(date_format);
    let time_specs = segment_specs(time_format);
    let mut combined = Vec::with_capacity(date_specs.len() + time_specs.len());

    for (index, spec) in date_specs.iter().enumerate() {
        let mut copy = *spec;
        if index == date_specs.len() - 1 {
            copy.separator = Some(" ");
        }
        combined.push(copy);
    }
    combined.extend_from_slice(time_specs);
    combined
}

const ISO_DATE: [SegmentSpec; 3] = [
    SegmentSpec {
        kind: SegmentKind::Year,
        max_len: 4,
        width_ch: 4,
        placeholder: "yyyy",
        separator: Some("-"),
    },
    SegmentSpec {
        kind: SegmentKind::Month,
        max_len: 2,
        width_ch: 2,
        placeholder: "mm",
        separator: Some("-"),
    },
    SegmentSpec {
        kind: SegmentKind::Day,
        max_len: 2,
        width_ch: 2,
        placeholder: "dd",
        separator: None,
    },
];

const US_DATE: [SegmentSpec; 3] = [
    SegmentSpec {
        kind: SegmentKind::Month,
        max_len: 2,
        width_ch: 2,
        placeholder: "mm",
        separator: Some("/"),
    },
    SegmentSpec {
        kind: SegmentKind::Day,
        max_len: 2,
        width_ch: 2,
        placeholder: "dd",
        separator: Some("/"),
    },
    SegmentSpec {
        kind: SegmentKind::Year,
        max_len: 4,
        width_ch: 4,
        placeholder: "yyyy",
        separator: None,
    },
];

const TIME_24: [SegmentSpec; 2] = [
    SegmentSpec {
        kind: SegmentKind::Hour,
        max_len: 2,
        width_ch: 2,
        placeholder: "hh",
        separator: Some(":"),
    },
    SegmentSpec {
        kind: SegmentKind::Minute,
        max_len: 2,
        width_ch: 2,
        placeholder: "mm",
        separator: None,
    },
];

const TIME_12: [SegmentSpec; 3] = [
    SegmentSpec {
        kind: SegmentKind::Hour,
        max_len: 2,
        width_ch: 2,
        placeholder: "hh",
        separator: Some(":"),
    },
    SegmentSpec {
        kind: SegmentKind::Minute,
        max_len: 2,
        width_ch: 2,
        placeholder: "mm",
        separator: Some(" "),
    },
    SegmentSpec {
        kind: SegmentKind::Meridiem,
        max_len: 2,
        width_ch: 2,
        placeholder: "AM",
        separator: None,
    },
];
