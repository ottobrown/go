use super::util::*;
use super::SgfResult;

/// An action done on the ui that can be converted to an sgf prop
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Action {
    NoOp,
    PlayBlack(usize, usize),
    PlayWhite(usize, usize),
}
impl Action {
    pub fn to_sgf_text(self) -> SgfResult<String> {
        use Action::*;

        let s = match self {
            NoOp => String::new(),
            PlayBlack(x, y) => format!(";B[{}{}]", to_sgf_coord(x)?, to_sgf_coord(y)?),
            PlayWhite(x, y) => format!(";W[{}{}]", to_sgf_coord(x)?, to_sgf_coord(y)?),
        };

        Ok(s)
    }
}
