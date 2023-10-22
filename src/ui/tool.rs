#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ToolType {
    /// Place alternating black and white stones
    Play,
    Circle,
    Cross,
    Square,
    Triangle,
    Dim,
    Arrow,
    Line,
    Number,
    Letter,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct UiTool {
    pub tool: ToolType,
    /// The base of a line or arrow
    /// used when the first point of a line or arrow has been placed, but not the other
    pub base: Option<(usize, usize)>,
    pub letter: char,
    pub number: u8,
}
impl UiTool {
    pub fn clear(&mut self) {
        self.base = None;
        self.number = 1;
        self.letter = 'A';
    }
}
