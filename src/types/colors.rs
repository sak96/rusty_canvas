use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(EnumString, EnumIter, Display, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
pub enum Color {
    #[default]
    Black,
    Red,
    Green,
    DarkBlue,
    Orange,
}

#[derive(EnumString, EnumIter, Display, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum BackgroundColor {
    Magenta,
    Blue,
    Cyan,
    Yellow,
}
