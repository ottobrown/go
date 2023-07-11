use super::util::*;
use super::SgfError;
use super::SgfResult;

/// An action done on the ui that can be converted to an sgf prop
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Action {
    NoOp,
    PlayBlack(usize, usize),
    PlayWhite(usize, usize),
    PassBlack,
    PassWhite,
    Size(usize, usize),
    Other(String, String),
}
impl Action {
    #[allow(clippy::wrong_self_convention)]
    pub fn to_sgf_text(&self) -> SgfResult<String> {
        use Action::*;

        let s = match self {
            NoOp => String::new(),
            // Should never fail because it would have failed at the construction
            // of the PlayBlack or PlayWhite
            PlayBlack(x, y) => format!("B[{}{}]", to_sgf_coord(*x)?, to_sgf_coord(*y)?),
            PlayWhite(x, y) => format!("W[{}{}]", to_sgf_coord(*x)?, to_sgf_coord(*y)?),
            PassBlack => String::from("B[]"),
            PassWhite => String::from("W[]"),
            Size(w, h) => {
                if *w == *h {
                    format!("SZ[{}]", w)
                } else {
                    format!("SZ[{}:{}]", w, h)
                }
            }
            Other(k, v) => format!("{}[{}]", &k, &v),
        };

        Ok(s)
    }

    pub fn from_pair(k: &str, v: &str) -> SgfResult<Action> {
        let upper = k.to_uppercase();
        Ok(match upper.as_str() {
            "B" => {
                if v.is_empty() {
                    return Ok(Action::PassBlack);
                }
                let coords = string_coords(v)?;

                Self::PlayBlack(coords.0, coords.1)
            }
            "W" => {
                if v.is_empty() {
                    return Ok(Action::PassWhite);
                }
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

            _ => Self::Other(String::from(k), String::from(v)),
        })
    }

    pub fn other(k: &str, v: &str) -> Self {
        Self::Other(String::from(k), String::from(v))
    }
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_sgf_text().unwrap())
    }
}

pub fn to_actions(s: &str) -> Vec<Action> {
    let mut actions = Vec::new();

    // split the sgf text at every opening and closing bracket.
    // this alternates between prop names and prop values
    let mut split = s.split(['[', ']']);

    while let (Some(l), Some(r)) = (split.next(), split.next()) {
        let k = l.trim().trim_matches(';');
        let v = r.trim().trim_matches(';');

        match Action::from_pair(k, v) {
            Ok(a) => actions.push(a),
            Err(e) => {
                #[cfg(debug_assertions)]
                crate::log(&format!(
                    "[WARNING] Action::from_pair failed with {:?} on {}[{}] {} {}:{} ",
                    e,
                    k,
                    v,
                    file!(),
                    line!(),
                    column!()
                ));
                actions.push(Action::other(k, v))
            }
        };
    }

    actions
}

#[test]
fn to_actions_test() {
    assert_eq!(
        to_actions(";B[aa]W[bb]"),
        vec![Action::PlayBlack(0, 0), Action::PlayWhite(1, 1)]
    );
}
