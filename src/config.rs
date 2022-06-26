use crate::ui::BoardStyle;

use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Default, Serialize, Deserialize)]
pub struct Config {
    pub style: BoardStyle,
}

