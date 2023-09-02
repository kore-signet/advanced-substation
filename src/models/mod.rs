use std::{borrow::Cow, str::FromStr};

use serde::{Deserialize, Serialize};

pub mod events;
pub mod script_info;
pub mod style;
pub(crate) type OptionStr<'a> = Option<Cow<'a, str>>;

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct Color {
    pub alpha: Option<u8>,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("&H").ok_or(())?;
        if s.len() == 8 {
            Ok(Color {
                alpha: Some(u8::from_str_radix(&s[0..2], 16).map_err(|_| ())?),
                blue: u8::from_str_radix(&s[2..4], 16).map_err(|_| ())?,
                green: u8::from_str_radix(&s[4..6], 16).map_err(|_| ())?,
                red: u8::from_str_radix(&s[6..8], 16).map_err(|_| ())?,
            })
        } else {
            Ok(Color {
                alpha: None,
                blue: u8::from_str_radix(&s[0..2], 16).map_err(|_| ())?,
                green: u8::from_str_radix(&s[2..4], 16).map_err(|_| ())?,
                red: u8::from_str_radix(&s[4..6], 16).map_err(|_| ())?,
            })
        }
    }
}
