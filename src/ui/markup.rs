use std::collections::HashSet;
use crate::sgf::Action;
use std::hash::Hash;

/// 1. Turn all the markup properties on the current node into a `MarkupState`.
/// 2. Allow the user to edit the `MarkupState`, adding and removing properties
/// 3. Turn the `MarkupState` back into `Action`s and save to the node.
#[derive(Default, Debug)]
pub struct MarkupState {
    circles: HashSet<(usize, usize)>,
    crosses: HashSet<(usize, usize)>,
    squares: HashSet<(usize, usize)>,
    triangles: HashSet<(usize, usize)>,
    dim: HashSet<(usize, usize)>,
    labels: HashSet<(usize, usize, String)>,
    arrows: HashSet<[(usize, usize); 2]>,
    lines: HashSet<[(usize, usize); 2]>,
    comments: HashSet<String>,
}
impl MarkupState {
    pub fn from_actions(actions: &Vec<Action>) -> Self {
        use Action::*;

        let mut r = Self::default();
        for a in actions {
            match a {
                Circle(v) => push_vec_to_hashset(v, &mut r.circles),
                Cross(v) => push_vec_to_hashset(v, &mut r.crosses),
                Square(v) => push_vec_to_hashset(v, &mut r.squares),
                Triangle(v) => push_vec_to_hashset(v, &mut r.triangles),
                Dim(v) => push_vec_to_hashset(v, &mut r.dim),
                Label(v) => push_vec_to_hashset(v, &mut r.labels),

                Arrow(v) => push_vec_to_hashset(v, &mut r.arrows),
                Line(v) => push_vec_to_hashset(v, &mut r.lines),

                Comment(s) => { r.comments.insert(s.clone()); },

                _ => {}
            }
        }

        return r;
    }

    /// remove all `Markup` actions and push the new ones on
    pub fn edit_actions(&self, v: &mut Vec<Action>) {
        for i in (0..v.len()).rev() {
            if v[i].is_markup() {
                v.remove(i);
            }
        }

        v.push(Action::Circle(self.circles.iter().map(|x| *x).collect::<Vec<_>>()));
        v.push(Action::Cross(self.crosses.iter().map(|x| *x).collect::<Vec<_>>()));
        v.push(Action::Square(self.squares.iter().map(|x| *x).collect::<Vec<_>>()));
        v.push(Action::Triangle(self.triangles.iter().map(|x| *x).collect::<Vec<_>>()));
        v.push(Action::Dim(self.dim.iter().map(|x| *x).collect::<Vec<_>>()));
        v.push(Action::Label(self.labels.iter().map(|x| x.clone()).collect::<Vec<_>>()));
        v.push(Action::Arrow(self.arrows.iter().map(|x| *x).collect::<Vec<_>>()));
        v.push(Action::Line(self.lines.iter().map(|x| *x).collect::<Vec<_>>()));

        for c in &self.comments {
            v.push(Action::Comment(c.clone()));
        }
    }
}

fn push_vec_to_hashset<T: Clone>(v: &Vec<T>, set: &mut HashSet<T>) where T: Eq, T: Hash {
    for i in v {
        set.insert(i.clone());
    }
}
