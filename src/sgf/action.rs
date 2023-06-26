use super::util::*;
use super::SgfError;
use super::SgfResult;

/// An action done on the ui that can be converted to an sgf prop
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Action {
    NoOp,
    PlayBlack(usize, usize),
    PlayWhite(usize, usize),
    Size(usize, usize),
}
impl Action {
    pub fn to_sgf_text(self) -> SgfResult<String> {
        use Action::*;

        let s = match self {
            NoOp => String::new(),
            PlayBlack(x, y) => format!(";B[{}{}]", to_sgf_coord(x)?, to_sgf_coord(y)?),
            PlayWhite(x, y) => format!(";W[{}{}]", to_sgf_coord(x)?, to_sgf_coord(y)?),
            Size(w, h) => {
                if w == h {
                    format!("SZ[{}]", w)
                } else {
                    format!("SZ[{}:{}]", w, h)
                }
            }
        };

        Ok(s)
    }

    pub fn from_pair(k: &str, v: &str) -> SgfResult<Action> {
        let upper = k.to_uppercase();
        let action = match upper.as_str() {
            "B" => {
                let coords = string_coords(v)?;

                Self::PlayBlack(coords.0, coords.1)
            }
            "W" => {
                let coords = string_coords(v)?;

                Self::PlayWhite(coords.0, coords.1)
            }
            "SZ" => {
                let mut s = [0, 0];
                let mut i = 0;

                for c in v.chars() {
                    if c.is_ascii_digit() {
                        s[i] *= 10;
                        s[i] += (c as usize) - 0x30;
                    }

                    if c == ':' {
                        if i == 1 {
                            return Err(SgfError::SizeParse);
                        }
                        i += 1;
                    }
                }

                if i == 0 {
                    s[1] = s[0];
                }

                Self::Size(s[0], s[1])
            }

            _ => Self::NoOp,
        };

        Ok(action)
    }
}

pub fn to_actions(s: &str) -> SgfResult<Vec<Action>> {
    let mut actions = Vec::new();

    // split the sgf text at every opening and closing bracket.
    // this alternates between prop names and prop values
    let mut split = s.split(['[', ']']);

    while let (Some(l), Some(r)) = (split.next(), split.next()) {
        let k = l.trim().trim_matches(';');
        let v = r.trim().trim_matches(';');

        actions.push(Action::from_pair(k, v)?);
    }

    Ok(actions)
}

#[test]
fn to_actions_test() {
    assert_eq!(
        to_actions(";B[aa]W[bb]"),
        Ok(vec![Action::PlayBlack(0, 0), Action::PlayWhite(1, 1)])
    );
}
