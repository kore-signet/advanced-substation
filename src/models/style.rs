use std::{borrow::Cow, str::FromStr};

use arraystring::ArrayString;
use serde::{Deserialize, Serialize};
use strum::EnumString;

use crate::LineItem;

use super::{Color, OptionStr};

pub const MAX_FIELDS: usize = 23;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Style<'a> {
    #[serde(borrow)]
    pub name: Cow<'a, str>,
    #[serde(borrow)]
    pub font_name: Cow<'a, str>,
    pub font_size: i64,
    pub primary_color: Color,
    pub secondary_color: Color,
    pub outline_color: Option<Color>,
    pub back_color: Color,
    pub bold: bool,
    pub italic: bool,
    pub underline: Option<bool>,
    pub strikeout: Option<bool>,
    pub scale_x: Option<f64>,
    pub scale_y: Option<f64>,
    pub spacing: Option<i64>,
    pub angle: Option<f64>,
    pub border_style: i64,
    pub outline: i64,
    pub shadow: i64,
    pub alignment: i64,
    pub margin_left: i64,
    pub margin_right: i64,
    pub margin_vertical: i64,
    #[serde(borrow)]
    pub encoding: OptionStr<'a>,
}

#[derive(Copy, Clone, EnumString, Debug)]
#[strum(ascii_case_insensitive, use_phf)]
#[repr(u8)]
pub enum StyleFields {
    Name = 0,
    Fontname = 1,
    Fontsize = 2,
    #[strum(serialize = "PrimaryColour", serialize = "PrimaryColor")]
    PrimaryColor = 3,
    #[strum(serialize = "SecondaryColour", serialize = "SecondaryColor")]
    SecondaryColor = 4,
    #[strum(
        serialize = "OutlineColour",
        serialize = "OutlineColor",
        serialize = "TertiaryColour"
    )]
    OutlineColor = 5,
    #[strum(serialize = "BackColour", serialize = "BackColor")]
    BackColor = 6,
    Bold = 7,
    Italic = 8,
    Underline = 9,
    Strikeout = 10,
    ScaleX = 11,
    ScaleY = 12,
    Spacing = 13,
    Angle = 14,
    BorderStyle = 15,
    Outline = 16,
    Shadow = 17,
    Alignment = 18,
    MarginL = 19,
    MarginR = 20,
    MarginV = 21,
    Encoding = 22,
    #[strum(default)]
    Other(ArrayString<arraystring::typenum::U32>) = 23,
}

impl Default for StyleFields {
    fn default() -> Self {
        Self::Other("unknown".into())
    }
}

impl<'data> LineItem<MAX_FIELDS> for Style<'data> {
    type Fields = StyleFields;

    type Item<'a>
     = Style<'a>;

    fn parse_from_fields<'a>(
        key: &'a str,
        fields: [(Self::Fields, OptionStr<'a>); MAX_FIELDS],
    ) -> Option<Self::Item<'a>> {
        if !key.eq_ignore_ascii_case("Style") {
            return None;
        }

        let mut style = Style::default();

        for (field, value) in fields {
            use StyleFields::*;
            match field {
                Name => style.name = value?,
                Fontname => style.font_name = value?,
                Fontsize => style.font_size = value.and_then(|v| v.parse().ok())?,
                PrimaryColor => {
                    style.primary_color = value.and_then(|v| Color::from_str(&v).ok())?
                }
                SecondaryColor => {
                    style.secondary_color = value.and_then(|v| Color::from_str(&v).ok())?
                }
                OutlineColor => style.outline_color = value.and_then(|v| Color::from_str(&v).ok()),
                BackColor => style.back_color = value.and_then(|v| Color::from_str(&v).ok())?,
                Bold => style.bold = value.and_then(bool_from_int)?,
                Italic => style.italic = value.and_then(bool_from_int)?,
                Underline => style.underline = value.and_then(bool_from_int),
                Strikeout => style.strikeout = value.and_then(bool_from_int),
                ScaleX => style.scale_x = value.and_then(|v| f64::from_str(&v).ok()),
                ScaleY => style.scale_y = value.and_then(|v| f64::from_str(&v).ok()),
                Spacing => style.spacing = value.and_then(|v| i64::from_str(&v).ok()),
                Angle => style.angle = value.and_then(|v| f64::from_str(&v).ok()),
                BorderStyle => style.border_style = value.and_then(|v| i64::from_str(&v).ok())?,
                Outline => style.outline = value.and_then(|v| i64::from_str(&v).ok())?,
                Shadow => style.shadow = value.and_then(|v| i64::from_str(&v).ok())?,
                Alignment => style.alignment = value.and_then(|v| i64::from_str(&v).ok())?,
                MarginL => style.margin_left = value.and_then(|v| i64::from_str(&v).ok())?,
                MarginR => style.margin_right = value.and_then(|v| i64::from_str(&v).ok())?,
                MarginV => style.margin_vertical = value.and_then(|v| i64::from_str(&v).ok())?,
                Encoding => style.encoding = value,
                Other(_) => continue,
            }
        }

        Some(style)
    }

    fn validate_section_name(name: &str) -> bool {
        name.eq_ignore_ascii_case("Styles")
            || name.eq_ignore_ascii_case("V4 Styles")
            || name.eq_ignore_ascii_case("V4+ Styles")
    }
}

fn bool_from_int(v: impl AsRef<str>) -> Option<bool> {
    match v.as_ref() {
        "-1" => Some(true),
        "0" => Some(false),
        _ => None,
    }
}
