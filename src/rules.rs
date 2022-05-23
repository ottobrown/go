#[derive(Clone, Copy)]
pub struct Rules {
    /// In half-points
    pub komi: u32,
    pub suicide_allowed: bool,
}
impl Rules {
    pub const CHINESE: Self = Self {
        // 7.5
        komi: 15,
        suicide_allowed: false,
    };

    pub const JAPANESE: Self = Self {
        /// 6.5
        komi: 13,
        suicide_allowed: false,
    };
}
