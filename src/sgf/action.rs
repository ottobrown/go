use super::util::*;
use super::SgfError;
use super::SgfResult;

/// An action done on the ui that can be converted to an sgf prop
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Action {
    NoOp,
    /// B[xy]
    PlayBlack(usize, usize),
    /// W[xy]
    PlayWhite(usize, usize),
    /// AB[xy][xy]...
    AddBlack(Vec<(usize, usize)>),
    /// AW[xy][xy]...
    AddWhite(Vec<(usize, usize)>),
    /// B[]
    PassBlack,
    /// W[]
    PassWhite,
    /// SZ[wh]
    Size(usize, usize),

    /// CR[xy][xy] ...
    Circle(Vec<(usize, usize)>),
    /// MA[xy][xy] ...
    Cross(Vec<(usize, usize)>),
    /// SQ[xy][xy] ...
    Square(Vec<(usize, usize)>),
    /// TR[xy][xy] ...
    Triangle(Vec<(usize, usize)>),
    /// DD[xy][xy] ...
    Dim(Vec<(usize, usize)>),
    // LB[xy:text]
    Label(Vec<(usize, usize, String)>),

    /// AR[xy:xy][xy:xy] ...
    Arrow(Vec<[(usize, usize); 2]>),
    /// LN[xy:xy][xy:xy] ...
    Line(Vec<[(usize, usize); 2]>),

    Comment(String),

    Other(String, String),
    OtherMany(String, Vec<String>),
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
            AddBlack(v) => coord_list("AB", v)?,
            AddWhite(v) => coord_list("AW", v)?,
            PassBlack => String::from("B[]"),
            PassWhite => String::from("W[]"),
            Size(w, h) => {
                if *w == *h {
                    format!("SZ[{}]", w)
                } else {
                    format!("SZ[{}:{}]", w, h)
                }
            }
            Circle(v) => coord_list("CR", v)?,
            Cross(v) => coord_list("MA", v)?,
            Square(v) => coord_list("SQ", v)?,
            Triangle(v) => coord_list("TR", v)?,
            Dim(v) => coord_list("DD", v)?,
            Label(v) => {
                let mut string = String::from("LB");
                for (x, y, l) in v {
                    string.push_str(&format!(
                        "[{}{}:{}]",
                        to_sgf_coord(*x)?,
                        to_sgf_coord(*y)?,
                        l
                    ));
                }

                string
            }

            Arrow(v) => {
                let mut string = String::from("AR");
                for p in v {
                    let (x1, y1) = p[0];
                    let (x2, y2) = p[1];
                    string.push_str(&format!(
                        "[{}{}:{}{}]",
                        to_sgf_coord(x1)?,
                        to_sgf_coord(y1)?,
                        to_sgf_coord(x2)?,
                        to_sgf_coord(y2)?
                    ));
                }

                string
            }
            Line(v) => {
                let mut string = String::from("LN");
                for p in v {
                    let (x1, y1) = p[0];
                    let (x2, y2) = p[1];
                    string.push_str(&format!(
                        "[{}{}:{}{}]",
                        to_sgf_coord(x1)?,
                        to_sgf_coord(y1)?,
                        to_sgf_coord(x2)?,
                        to_sgf_coord(y2)?
                    ));
                }

                string
            }

            Comment(s) => format!("C[{}]", s),

            Other(k, v) => format!("{}[{}]", &k, &v),
            OtherMany(k, v) => {
                let mut string = k.to_string();
                for i in v {
                    string.push_str(&format!("[{}]", i));
                }

                string
            }
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

            "C" => Self::Comment(String::from(v)),

            _ => Self::Other(String::from(k), String::from(v)),
        })
    }

    // clippy wants `v` to be &[String] instead of &Vec<String>
    #[allow(clippy::ptr_arg)]
    pub fn from_many(k: &str, v: &Vec<String>) -> SgfResult<Self> {
        let a = match k {
            "AB" => Action::AddBlack(points_list(v)?),
            "AW" => Action::AddWhite(points_list(v)?),
            "CR" => Action::Circle(points_list(v)?),
            "MA" => Action::Cross(points_list(v)?),
            "SQ" => Action::Square(points_list(v)?),
            "TR" => Action::Triangle(points_list(v)?),
            "DD" => Action::Dim(points_list(v)?),
            "LB" => {
                let mut points = Vec::with_capacity(v.len());
                for i in v {
                    let mut split = i.split(':');
                    let a: [&str; 2] = [
                        split.next().ok_or(SgfError::InvalidComposedLength)?,
                        split.next().ok_or(SgfError::InvalidComposedLength)?,
                    ];

                    if split.next().is_some() {
                        return Err(SgfError::InvalidComposedLength);
                    }

                    let (x, y) = string_coords(a[0])?;
                    let s = a[1].to_string();

                    points.push((x, y, s));
                }

                Action::Label(points)
            }
            "AR" => Action::Arrow(points_pair_list(v)?),
            "LN" => Action::Line(points_pair_list(v)?),

            _ => Action::OtherMany(k.to_string(), v.clone()),
        };

        Ok(a)
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

#[derive(Clone, Debug, PartialEq)]
enum PropFragment {
    /// Out of []
    Name(String),
    /// Within []
    Value(String),
}

fn to_fragments(s: &str) -> Vec<PropFragment> {
    let mut fragments = Vec::new();

    let mut buffer = String::new();

    for c in s.chars() {
        match c {
            ';' => {}
            '[' => {
                if !buffer.is_empty() {
                    fragments.push(PropFragment::Name(buffer.clone()));
                    buffer.clear();
                }
            }
            ']' => {
                fragments.push(PropFragment::Value(buffer.clone()));
                buffer.clear();
            }

            _ => buffer.push(c),
        }
    }

    fragments
}

pub fn to_actions(s: &str) -> Vec<Action> {
    let mut actions = Vec::new();

    let fragments = to_fragments(s);

    let mut name = String::new();
    let mut props: Vec<String> = Vec::new();

    for fragment in fragments {
        match fragment {
            PropFragment::Name(n) => {
                if props.len() == 0 {
                    name = n;
                    continue;
                }
                if !is_list(&name) {
                    let a = Action::from_pair(&name, &props[0]);
                    match a {
                        Ok(i) => actions.push(i),
                        Err(_e) => crate::log("[WARNING] Action::from_pair failed"),
                    };
                } else {
                    let a = Action::from_many(&name, &props);
                    match a {
                        Ok(i) => actions.push(i),
                        Err(_e) => crate::log("[WARNING] Action::from_many failed"),
                    };
                }

                props.clear();
                name = n;
            }
            PropFragment::Value(v) => props.push(v),
        };
    }

    if !is_list(&name) && props.len() != 0 {
        let a = Action::from_pair(&name, &props[0]);
        match a {
            Ok(i) => actions.push(i),
            Err(_e) => crate::log("[WARNING] Action::from_pair failed"),
        };
    } else {
        let a = Action::from_many(&name, &props);
        match a {
            Ok(i) => actions.push(i),
            Err(_e) => crate::log("[WARNING] Action::from_many failed"),
        };
    }

    actions
}

#[test]
fn to_frag_test() {
    assert_eq!(
        to_fragments(";AB[aa][bb]W[cc]"),
        vec![
            PropFragment::Name("AB".to_string()),
            PropFragment::Value("aa".to_string()),
            PropFragment::Value("bb".to_string()),
            PropFragment::Name("W".to_string()),
            PropFragment::Value("cc".to_string()),
        ]
    )
}

#[test]
fn to_actions_test() {
    assert_eq!(
        to_actions(";B[aa]W[bb]"),
        vec![Action::PlayBlack(0, 0), Action::PlayWhite(1, 1)]
    );

    assert_eq!(
        to_actions(";AB[aa][bb]W[cc]"),
        vec![
            Action::AddBlack(vec![(0, 0), (1, 1)]),
            Action::PlayWhite(2, 2)
        ]
    );

    assert_eq!(
        to_actions(";SQ[aa][bb]AR[ac:ca][ad:da]C[text text text]"),
        vec![
            Action::Square(vec![(0, 0), (1, 1)]),
            Action::Arrow(vec![[(0, 2), (2, 0)], [(0, 3), (3, 0)]]),
            Action::Comment("text text text".to_string()),
        ]
    );
}
