use crate::sgf::{Action, SgfResult, SgfTree};
use crate::Board;
use crate::Stone;
use std::fs;
use std::path::PathBuf;

/// Contains the Board and additional info about the game
/// that can be manipulated by actions in the ui
pub struct Game {
    pub board: Board,
    pub turn: Stone,

    pub tree: crate::SgfTree,
    pub path: Option<PathBuf>,
}
impl Game {
    pub fn do_action(&mut self, a: &Action) {
        match a {
            Action::PlayBlack(x, y) => {
                if !self.board.attempt_set(*x, *y, Stone::Black) {
                    self.board.set(*x, *y, Stone::Black);
                }
                self.turn = Stone::White;
            }
            Action::PlayWhite(x, y) => {
                if !self.board.attempt_set(*x, *y, Stone::White) {
                    self.board.set(*x, *y, Stone::White);
                }
                self.turn = Stone::Black;
            }

            _ => {}
        }
    }

    /// Starting from a blank board, performs all the actions up to this point in `self.tree`
    pub fn do_to_now(&mut self) {
        let (w, h) = self.board.size();
        self.board = Board::new(w, h);
        self.turn = Stone::Black;

        let all = self.tree.get_all_parent_action();

        for s in all.iter().rev() {
            for a in s {
                self.do_action(a);
            }
        }
    }

    pub fn write_to_file(&mut self) -> SgfResult<()> {
        if let Some(p) = &self.path {
            fs::write(p, self.tree.to_text().as_bytes())?;
        } else {
            self.path = rfd::FileDialog::new()
                .add_filter("sgf", &["sgf"])
                .save_file();
            self.write_to_file()?;
        }

        Ok(())
    }
}

pub struct GameBuilder {
    pub size: (usize, usize),
    pub path: Option<PathBuf>,
}
impl GameBuilder {
    pub fn build(&self) -> Game {
        if let Some(p) = &self.path {
            // TODO: handle this unwrap
            return build_game_from_path(p.clone()).unwrap();
        }

        let mut tree = crate::SgfTree::default();
        let root = format!(
            "FF[4]CA[UTF-8]GM[1]{}",
            Action::Size(self.size.0, self.size.1)
        );

        if let Err(e) = tree.set_root(root) {
            #[cfg(debug_assertions)]
            crate::log(format!("FAILED TO SET ROOT WITH {:?}", e));
        }

        Game {
            board: Board::new(self.size.0, self.size.1),
            turn: Stone::Black,
            tree,
            path: self.path.clone(),
        }
    }
}

impl Default for GameBuilder {
    fn default() -> Self {
        Self {
            size: (19, 19),
            path: None,
        }
    }
}

fn build_game_from_path(p: PathBuf) -> SgfResult<Game> {
    let s = fs::read_to_string(&p)?;
    let tree = SgfTree::parse(s)?;

    let mut size = (19, 19);

    for a in &tree.root().actions {
        if let Action::Size(w, h) = a {
            size = (*w, *h);
        }
    }

    Ok(Game {
        board: Board::new(size.0, size.1),
        tree,
        turn: Stone::Black,
        path: Some(p),
    })
}
