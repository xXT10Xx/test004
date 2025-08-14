use crate::html::tokenizer::{HtmlTokenizer, HtmlToken};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Element {
    pub tag_name: String,
    pub attributes: HashMap<String, String>,
    pub children: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Element(Element),
    Text(String),
    Comment(String),
}

pub struct HtmlParser<'a> {
    tokenizer: HtmlTokenizer<'a>,
    current_token: Option<HtmlToken<'a>>,
}

impl<'a> HtmlParser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut tokenizer = HtmlTokenizer::new(input);
        let current_token = tokenizer.next_token();
        
        Self {
            tokenizer,
            current_token,
        }
    }

    pub fn parse(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();
        
        while let Some(token) = self.current_token.clone() {
            match token {
                HtmlToken::StartTag { name, attributes, self_closing } => {
                    let element = self.parse_element(&name, &attributes, self_closing);
                    nodes.push(Node::Element(element));
                }
                HtmlToken::Text(text) => {
                    if !text.trim().is_empty() {
                        nodes.push(Node::Text(text.to_string()));
                    }
                    self.advance();
                }
                HtmlToken::Comment(comment) => {
                    nodes.push(Node::Comment(comment.to_string()));
                    self.advance();
                }
                HtmlToken::Doctype(_) => {
                    // Skip doctype for now
                    self.advance();
                }
                HtmlToken::EndTag { .. } => {
                    // Unexpected end tag at root level
                    break;
                }
            }
        }
        
        nodes
    }

    fn parse_element(&mut self, name: &str, attributes: &[(&str, &str)], self_closing: bool) -> Element {
        let mut element = Element {
            tag_name: name.to_string(),
            attributes: attributes.iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            children: Vec::new(),
        };

        self.advance(); // Move past start tag

        if self_closing || self.is_void_element(name) {
            return element;
        }

        // Parse children until we find the matching end tag
        while let Some(token) = self.current_token.clone() {
            match token {
                HtmlToken::EndTag { name: end_name } => {
                    if end_name == name {
                        self.advance(); // Consume the end tag
                        break;
                    } else {
                        // Mismatched end tag, treat as text
                        let text = format!("</{}>", end_name);
                        element.children.push(Node::Text(text));
                        self.advance();
                    }
                }
                HtmlToken::StartTag { name: child_name, attributes: child_attrs, self_closing } => {
                    let child_element = self.parse_element(&child_name, &child_attrs, self_closing);
                    element.children.push(Node::Element(child_element));
                }
                HtmlToken::Text(text) => {
                    if !text.trim().is_empty() {
                        element.children.push(Node::Text(text.to_string()));
                    }
                    self.advance();
                }
                HtmlToken::Comment(comment) => {
                    element.children.push(Node::Comment(comment.to_string()));
                    self.advance();
                }
                HtmlToken::Doctype(_) => {
                    // Skip doctype
                    self.advance();
                }
            }
        }

        element
    }

    fn advance(&mut self) {
        self.current_token = self.tokenizer.next_token();
    }

    fn is_void_element(&self, name: &str) -> bool {
        matches!(name.to_lowercase().as_str(),
            "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input" |
            "link" | "meta" | "param" | "source" | "track" | "wbr"
        )
    }
}