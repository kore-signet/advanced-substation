use serde::{Deserialize, Serialize};
use std::{borrow::Cow, str::FromStr};
use strum::{EnumString, FromRepr};

use crate::KeyValueSection;

use super::OptionStr;

#[derive(EnumString, Clone, Copy)]
#[strum(ascii_case_insensitive, use_phf)]
pub enum ScriptInfoFields {
    Title,
    #[strum(serialize = "Original Script", serialize = "OriginalScript")]
    OriginalScript,
    #[strum(serialize = "Original Translation", serialize = "OriginalTranslation")]
    OriginalTranslation,
    #[strum(serialize = "Original Editing", serialize = "OriginalEditing")]
    OriginalEditing,
    #[strum(serialize = "Original Timing", serialize = "OriginalTiming")]
    OriginalTiming,
    #[strum(serialize = "Synch Point", serialize = "SynchPoint")]
    SynchPoint,
    #[strum(serialize = "Script Updated By", serialize = "ScriptUpdatedBy")]
    ScriptUpdatedBy,
    #[strum(serialize = "Update Details", serialize = "UpdatDetails")]
    UpdateDetails,
    #[strum(serialize = "Script Type", serialize = "ScriptType")]
    ScriptType,
    Collisions,
    PlayResX,
    PlayResY,
    PlayDepth,
    Timer,
    #[strum(
        serialize = "ScaledBorderAndShadow",
        serialize = "Scaled Border And Shadow"
    )]
    ScaledBorderAndShadow,
    #[strum(serialize = "WrapStyle", serialize = "Wrap Style")]
    WrapStyle,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ScriptInfo<'a> {
    #[serde(borrow)]
    pub title: Cow<'a, str>,
    #[serde(borrow)]
    pub authors: Authors<'a>,
    #[serde(borrow)]
    pub synch_point: OptionStr<'a>,
    #[serde(borrow)]
    pub script_type: OptionStr<'a>,
    pub collisions: CollisionHandling,
    #[serde(borrow)]
    pub play_info: PlayInfo<'a>,
    pub timer: Option<f64>,
    pub scaled_border_and_shadow: bool,
    pub wrap_style: Option<WrapStyle>,
}

impl<'data> KeyValueSection<'data> for ScriptInfo<'data> {
    type Output<'a, 'b> = ScriptInfo<'a> where 'a: 'b, 'data: 'b;
    type Fields = ScriptInfoFields;

    fn parse<'b>(
        source: crate::KeyValueSectionIter<'data, 'b, Self::Fields>,
    ) -> Option<Self::Output<'data, 'b>> {
        if !(source.title.eq_ignore_ascii_case("Script Info")
            || source.title.eq_ignore_ascii_case("ScriptInfo"))
        {
            return None;
        }

        let mut section = ScriptInfo::default();

        for (field, value) in source {
            use ScriptInfoFields::*;
            match field {
                Title => section.title = value.into(),
                OriginalScript => section.authors.script = Some(value.into()),
                OriginalTranslation => section.authors.translation = Some(value.into()),
                OriginalEditing => section.authors.editing = Some(value.into()),
                OriginalTiming => section.authors.timing = Some(value.into()),
                SynchPoint => section.synch_point = Some(value.into()),
                ScriptUpdatedBy => section.authors.updated_by = Some(value.into()),
                UpdateDetails => section.authors.update_details = Some(value.into()),
                ScriptType => section.script_type = Some(value.into()),
                Collisions => {
                    if let Ok(col) = CollisionHandling::from_str(value) {
                        section.collisions = col;
                    }
                }
                PlayResX => section.play_info.play_res_x = value.parse().ok(),
                PlayResY => section.play_info.play_res_y = value.parse().ok(),
                PlayDepth => section.play_info.play_depth = Some(value.into()),
                Timer => section.timer = value.parse().ok(),
                ScaledBorderAndShadow => {
                    section.scaled_border_and_shadow = value.eq_ignore_ascii_case("yes")
                }
                WrapStyle => {
                    section.wrap_style = u8::from_str(value)
                        .ok()
                        .and_then(self::WrapStyle::from_repr)
                }
            }
        }

        Some(section)
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Authors<'a> {
    #[serde(borrow)]
    pub script: OptionStr<'a>,
    #[serde(borrow)]
    pub translation: OptionStr<'a>,
    #[serde(borrow)]
    pub editing: OptionStr<'a>,
    #[serde(borrow)]
    pub timing: OptionStr<'a>,
    #[serde(borrow)]
    pub updated_by: OptionStr<'a>,
    #[serde(borrow)]
    pub update_details: OptionStr<'a>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct PlayInfo<'a> {
    pub play_res_x: Option<i64>,
    pub play_res_y: Option<i64>,
    #[serde(borrow)]
    pub play_depth: OptionStr<'a>,
}

#[derive(EnumString, Debug, Serialize, Deserialize)]
#[strum(ascii_case_insensitive)]
#[derive(Default)]
pub enum CollisionHandling {
    #[default]
    Normal,
    Reverse,
}

#[derive(FromRepr, Debug, Serialize, Deserialize)]
#[repr(u8)]
#[derive(Default)]
pub enum WrapStyle {
    #[default]
    Smart = 0,
    EndOfLine = 1,
    NoWrapping = 2,
    WiderLowerLine = 3,
}
