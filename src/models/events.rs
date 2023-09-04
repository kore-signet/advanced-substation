use std::{borrow::Cow, time::Duration};

use arraystring::ArrayString;
use serde::{Deserialize, Serialize};
use strum::EnumString;

use crate::LineItem;
use std::str::FromStr;

use super::OptionStr;

pub const MAX_FIELDS: usize = 10;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct EventLine<'a> {
    pub is_comment: bool,
    #[serde(borrow)]
    pub marked: OptionStr<'a>,
    pub layer: Option<i64>,
    pub start: Duration,
    pub end: Duration,
    #[serde(borrow)]
    pub style: Cow<'a, str>,
    #[serde(borrow)]
    pub name: Cow<'a, str>,
    pub margin_left: i64,
    pub margin_right: i64,
    pub margin_vertical: i64,
    #[serde(borrow)]
    pub effect: Cow<'a, str>,
    #[serde(borrow)]
    pub text: Cow<'a, str>,
}

#[derive(Copy, Clone, EnumString, Debug)]
#[strum(ascii_case_insensitive, use_phf)]
#[repr(i8)]
pub enum EventFields {
    Marked = -1,
    Layer = 0,
    Start = 1,
    End = 2,
    Style = 3,
    Name = 4,
    MarginL = 5,
    MarginR = 6,
    MarginV = 7,
    Effect = 8,
    Text = 9,
    #[strum(default)]
    Other(ArrayString<arraystring::typenum::U32>) = 10,
}

impl Default for EventFields {
    fn default() -> Self {
        Self::Other("unknown".into())
    }
}

impl<'data> LineItem<MAX_FIELDS> for EventLine<'data> {
    type Fields = EventFields;

    type Item<'a>
     = EventLine<'a>;

    fn parse_from_fields<'a>(
        key: &'a str,
        fields: [(Self::Fields, OptionStr<'a>); MAX_FIELDS],
    ) -> Option<Self::Item<'a>> {
        let mut event = EventLine::default();
        event.is_comment = key.eq_ignore_ascii_case("Comment");

        for (field, value) in fields {
            use EventFields::*;
            match field {
                Layer => event.layer = value.and_then(|v| i64::from_str(&v).ok()),
                Marked => event.marked = value,
                Start => event.start = value.and_then(parse_time)?,
                End => event.end = value.and_then(parse_time)?,
                Style => event.style = value?,
                Name => event.name = value?,
                MarginL => event.margin_left = value.and_then(|v| i64::from_str(&v).ok())?,
                MarginR => event.margin_right = value.and_then(|v| i64::from_str(&v).ok())?,
                MarginV => event.margin_vertical = value.and_then(|v| i64::from_str(&v).ok())?,
                Effect => event.effect = value?,
                Text => event.text = value?,
                Other(_) => continue,
            }
        }

        Some(event)
    }

    fn validate_section_name(name: &str) -> bool {
        name.eq_ignore_ascii_case("Events")
    }
}

fn parse_time(s: impl AsRef<str>) -> Option<Duration> {
    let mut time_split = s
        .as_ref()
        .splitn(4, &[':', '.'])
        .filter_map(|v| u64::from_str(v).ok());

    let (hours, mins, secs, hundredths) = (
        time_split.next()?,
        time_split.next()?,
        time_split.next()?,
        time_split.next()?,
    );
    Some(Duration::from_millis(
        hours * 3_600_000 + mins * 60_000 + secs * 1_000 + hundredths * 10,
    ))
}
