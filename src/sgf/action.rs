use super::{SgfError, SgfResult};

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
            PlayBlack(x, y) => format!(";B[{}{}]", sgf_coord(x)?, sgf_coord(y)?),
            PlayWhite(x, y) => format!(";W[{}{}]", sgf_coord(x)?, sgf_coord(y)?),
        };

        Ok(s)
    }
}

pub fn sgf_coord(x: usize) -> SgfResult<char> {
    if x <= 25 {
        return Ok((x as u8 + 97) as char);
    } else if x <= 51 {
        return Ok((x as u8 + 39) as char);
    }
    Err(SgfError::CoordTooBig)
}

#[test]
fn coord_test() {
    assert_eq!(sgf_coord(0), Ok('a'));
    assert_eq!(sgf_coord(25), Ok('z'));
    assert_eq!(sgf_coord(26), Ok('A'));
    assert_eq!(sgf_coord(51), Ok('Z'));
    assert_eq!(sgf_coord(52), Err(SgfError::CoordTooBig));
}
