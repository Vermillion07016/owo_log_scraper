use std::str::FromStr;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Rank {
    Common,
    UnCommon,
    Rare,
    Epic,
    Mythical,
    Legendary,
}
impl FromStr for Rank {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if "common".eq_ignore_ascii_case(s) {
            Ok(Self::Common)
        }else if "uncommon".eq_ignore_ascii_case(s) {
            Ok(Self::UnCommon)
        }else if "rare".eq_ignore_ascii_case(s) {
            Ok(Self::Rare)
        }else if "epic".eq_ignore_ascii_case(s) {
            Ok(Self::Epic)
        }else if "mythical".eq_ignore_ascii_case(s) {
            Ok(Self::Mythical)
        }else if "legendary".eq_ignore_ascii_case(s) {
            Ok(Self::Legendary)
        }else {
            Err(format!("can't parse: {}", s))
        }
    }
}
impl Default for Rank {
    fn default() -> Self {
        Self::Common
    }
}