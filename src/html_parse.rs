use dom::{AttrMap, ElementData, Node, NodeType};

use std::iter::Peekable;
use std::str::Chars;

pub struct HtmlParser<'a> {
    chars: Peekable<Chars<'a>>,
    node_q: Vec<String>,
}

impl<'a> HtmlParser<'a> {
    pub fn new(full_html: &str) -> HtmlParser {
        HtmlParser {
            chars: full_html.chars().peekable(),
            node_q: Vec::new(),
        }
    }

    pub fn parse_nodes(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();
        while self.chars.peek().is_some(){
            self.consume_while(char::is_whitespace);
            if self.chars.peek().map_or(false, |c| *c == '<'){
                self.chars.next();
                if self.chars.peek().map_or(false, |c| *c == '/'){
                    self.chars.next();
                    self.consume_while(char::is_whitespace);

                    let close_tag_name = self.consume_while(is_valid_tag_name);

                    self.consume_while(|x| x != '>');
                    self.chars.next();

                    self.node_q.push(close_tag_name);
                    break;
                } else if self.chars.peek().map_or(false, |c| *c == '!') {
                    self.chars.next();
                    nodes.push(self.parse_comment_node());
                }
            }
        }
    }

    fn parse_comment_node(&mut self) -> Node {
        let mut comment_content = String::new();

        if self.chars.peek().map_or(false, |c| *c == '-'){
            self.chars.next();
            if self.chars.peek().map_or(false, |c| *c == '-'){
                self.chars.next();
            } else {
                self.consume_while(|c| c != '>');
                return Node::new(NodeType::Comment(comment_content), Vec::new());
            }
        }
    }

    fn consume_while<F>(&mut self, condition: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while self.chars.peek().map_or(false, |c| condition(*c)) {
            result.push(self.chars.next().unwrap());
        }
        result
    }

    fn is_valid_tag_name(ch: char) -> bool {
            ch.is_digit(36)
        }
}
