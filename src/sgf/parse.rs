use super::SgfError;
use super::SgfResult;
use super::SgfTree;

#[derive(Clone, Debug, PartialEq)]
pub enum ParserToken {
    /// '('
    LParen,
    /// ')'
    RParen,
    /// ';' followed by text
    Node(String),
}

pub fn lex(s: String) -> Vec<ParserToken> {
    use ParserToken::*;

    let mut tokens = Vec::new();
    let mut node = String::new();

    for ch in s.chars() {
        match ch {
            '(' => {
                if node.starts_with(';') {
                    tokens.push(Node(node.trim().to_string()));
                    node.clear();
                }
                tokens.push(LParen);
            }
            ')' => {
                if node.starts_with(';') {
                    tokens.push(Node(node.trim().to_string()));
                    node.clear();
                }
                tokens.push(RParen);
            }
            ';' => {
                if node.starts_with(';') {
                    tokens.push(Node(node.trim().to_string()));
                    node.clear();
                }
                node.push(';');
            }

            _ => {
                if node.starts_with(';') {
                    node.push(ch);
                }
            }
        }
    }

    tokens
}

pub fn parse(tokens: Vec<ParserToken>) -> SgfResult<SgfTree> {
    let mut tree = SgfTree::default();
    let mut iter = tokens.iter();
    let mut stack = Vec::new();

    // parse root node
    if iter.next() != Some(&ParserToken::LParen) {
        return Err(SgfError::MissingLParen);
    }
    if let Some(ParserToken::Node(s)) = iter.next() {
        tree.set_root(s.clone());
    }

    for token in iter {
        match token {
            ParserToken::Node(s) => {
                tree.handle_new_text(s.clone());

                if let Some(n) = stack.last_mut() {
                    *n += 1;
                }
            }

            ParserToken::LParen => {
                stack.push(0);
            }

            ParserToken::RParen => {
                if let Some(n) = stack.pop() {
                    for _ in 0..n {
                        tree.select_parent()?;
                    }
                }
            }
        }
    }

    tree.select_root();
    Ok(tree)
}

#[cfg(test)]
mod parse_tests {
    use super::*;

    #[test]
    fn lex_test() {
        use ParserToken::*;

        let s = String::from("(;FF[4]  ;B[pd]   ;W[dp];B[dd](;W[qp];B[oq])(;W[pq];B[qo]))");
        let l = vec![
            LParen,
            Node(String::from(";FF[4]")),
            Node(String::from(";B[pd]")),
            Node(String::from(";W[dp]")),
            Node(String::from(";B[dd]")),
            LParen,
            Node(String::from(";W[qp]")),
            Node(String::from(";B[oq]")),
            RParen,
            LParen,
            Node(String::from(";W[pq]")),
            Node(String::from(";B[qo]")),
            RParen,
            RParen,
        ];

        assert_eq!(lex(s), l);
    }
}
