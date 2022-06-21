use crate::Stone;

#[derive(Copy, Clone, PartialEq)]
pub struct Rules {
    /// In half-points
    pub komi: u32,
    pub suicide_allowed: bool,
    /// ban repeated board states
    pub superko: bool,
}
impl Rules {
    pub const CHINESE: Self = Self {
        // 7.5
        komi: 15,
        suicide_allowed: false,
        // Note: chinese rules actually use a slightly different superko rule
        // [see here](https://senseis.xmp.net/?ChineseSuperko)
        superko: true,
    };

    pub const JAPANESE: Self = Self {
        /// 6.5
        komi: 13,
        suicide_allowed: false,
        superko: false,
    };

    pub const NEW_ZEALAND: Self = Self {
        // 7
        komi: 14,
        suicide_allowed: true,
        superko: true,
    };
}

/// The winner and method of winning
#[derive(Clone)]
#[allow(unused)]
pub enum EndGame {
    // Score in half points
    Score(Stone, u32),
    Resign(Stone),
    Time(Stone),
    Forfiet(Stone),
}
impl EndGame {
    pub fn display(&self) -> String {
        match self {
            Self::Score(s, p) => format!("{:?} won by {} points.", s, 0.5 * (*p as f32)),
            Self::Resign(s) => format!("{:?} won by resignation.", s),
            Self::Time(s) => format!("{:?} won by time.", s),
            Self::Forfiet(s) => format!("{:?} won by forfiet", s),
        }
    }
}
