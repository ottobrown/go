#[derive(Clone, Copy)]
pub struct Rules {
    /// In half-points
    pub komi: u32,
    pub suicide_allowed: bool,
}
impl Rules {
    #[allow(unused)]
    pub const CHINESE: Self = Self {
        // 7.5
        komi: 15,
        suicide_allowed: false,
    };

    #[allow(unused)]
    pub const JAPANESE: Self = Self {
        /// 6.5
        komi: 13,
        suicide_allowed: false,
    };

    #[allow(unused)]
    pub const NEW_ZEALAND: Self = Self {
        // 7
        komi: 14,
        suicide_allowed: true,
    };
}
