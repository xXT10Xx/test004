use crate::css::tokenizer::{CssTokenizer, CssToken};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Selector {
    Type(String),
    Class(String),
    Id(String),
    Universal,
    Descendant(Box<Selector>, Box<Selector>),
    Child(Box<Selector>, Box<Selector>),
    Adjacent(Box<Selector>, Box<Selector>),
    GeneralSibling(Box<Selector>, Box<Selector>),
}

pub struct CssParser<'a> {
    tokenizer: CssTokenizer<'a>,
    current_token: Option<CssToken<'a>>,
}

impl<'a> CssParser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut tokenizer = CssTokenizer::new(input);
        let current_token = tokenizer.next_token();
        
        Self {
            tokenizer,
            current_token,
        }
    }

    pub fn parse(&mut self) -> Vec<Rule> {
        let mut rules = Vec::new();
        
        while self.current_token.is_some() {
            self.skip_whitespace();
            
            if let Some(rule) = self.parse_rule() {
                rules.push(rule);
            } else {
                // Skip invalid tokens
                self.advance();
            }
        }
        
        rules
    }

    fn parse_rule(&mut self) -> Option<Rule> {
        let selectors = self.parse_selectors()?;
        
        self.skip_whitespace();
        
        // Expect '{'
        if !matches!(self.current_token, Some(CssToken::LeftBrace)) {
            return None;
        }
        self.advance(); // Skip '{'
        
        let declarations = self.parse_declarations();
        
        // Expect '}'
        if matches!(self.current_token, Some(CssToken::RightBrace)) {
            self.advance(); // Skip '}'
        }
        
        Some(Rule {
            selectors,
            declarations,
        })
    }

    fn parse_selectors(&mut self) -> Option<Vec<Selector>> {
        let mut selectors = Vec::new();
        
        loop {
            self.skip_whitespace();
            
            if let Some(selector) = self.parse_selector() {
                selectors.push(selector);
            } else {
                break;
            }
            
            self.skip_whitespace();
            
            if matches!(self.current_token, Some(CssToken::Comma)) {
                self.advance(); // Skip ','
                continue;
            } else {
                break;
            }
        }
        
        if selectors.is_empty() {
            None
        } else {
            Some(selectors)
        }
    }

    fn parse_selector(&mut self) -> Option<Selector> {
        self.skip_whitespace();
        
        let mut selector = self.parse_simple_selector()?;
        
        loop {
            self.skip_whitespace();
            
            match &self.current_token {
                Some(CssToken::LeftBrace) | Some(CssToken::Comma) | None => break,
                Some(CssToken::Delim('>')) => {
                    self.advance(); // Skip '>'
                    self.skip_whitespace();
                    if let Some(right) = self.parse_simple_selector() {
                        selector = Selector::Child(Box::new(selector), Box::new(right));
                    }
                }
                Some(CssToken::Delim('+')) => {
                    self.advance(); // Skip '+'
                    self.skip_whitespace();
                    if let Some(right) = self.parse_simple_selector() {
                        selector = Selector::Adjacent(Box::new(selector), Box::new(right));
                    }
                }
                Some(CssToken::Delim('~')) => {
                    self.advance(); // Skip '~'
                    self.skip_whitespace();
                    if let Some(right) = self.parse_simple_selector() {
                        selector = Selector::GeneralSibling(Box::new(selector), Box::new(right));
                    }
                }
                _ => {
                    // Descendant combinator (whitespace)
                    if let Some(right) = self.parse_simple_selector() {
                        selector = Selector::Descendant(Box::new(selector), Box::new(right));
                    } else {
                        break;
                    }
                }
            }
        }
        
        Some(selector)
    }

    fn parse_simple_selector(&mut self) -> Option<Selector> {
        match &self.current_token {
            Some(CssToken::Ident(name)) => {
                let selector = Selector::Type(name.to_string());
                self.advance();
                Some(selector)
            }
            Some(CssToken::Hash(id)) => {
                let selector = Selector::Id(id.to_string());
                self.advance();
                Some(selector)
            }
            Some(CssToken::Delim('.')) => {
                self.advance(); // Skip '.'
                if let Some(CssToken::Ident(class)) = &self.current_token {
                    let selector = Selector::Class(class.to_string());
                    self.advance();
                    Some(selector)
                } else {
                    None
                }
            }
            Some(CssToken::Delim('*')) => {
                self.advance();
                Some(Selector::Universal)
            }
            _ => None,
        }
    }

    fn parse_declarations(&mut self) -> HashMap<String, String> {
        let mut declarations = HashMap::new();
        
        loop {
            self.skip_whitespace();
            
            if matches!(self.current_token, Some(CssToken::RightBrace)) || self.current_token.is_none() {
                break;
            }
            
            if let Some((property, value)) = self.parse_declaration() {
                declarations.insert(property, value);
            }
            
            // Skip semicolon if present
            if matches!(self.current_token, Some(CssToken::Semicolon)) {
                self.advance();
            }
        }
        
        declarations
    }

    fn parse_declaration(&mut self) -> Option<(String, String)> {
        // Parse property name
        let property = match &self.current_token {
            Some(CssToken::Ident(name)) => {
                let prop = name.to_string();
                self.advance();
                prop
            }
            _ => return None,
        };
        
        self.skip_whitespace();
        
        // Expect ':'
        if !matches!(self.current_token, Some(CssToken::Colon)) {
            return None;
        }
        self.advance(); // Skip ':'
        
        self.skip_whitespace();
        
        // Parse value
        let mut value_parts = Vec::new();
        
        loop {
            match &self.current_token {
                Some(CssToken::Semicolon) | Some(CssToken::RightBrace) | None => break,
                Some(CssToken::Whitespace) => {
                    if !value_parts.is_empty() {
                        value_parts.push(" ".to_string());
                    }
                    self.advance();
                }
                Some(token) => {
                    value_parts.push(self.token_to_string(token));
                    self.advance();
                }
            }
        }
        
        if value_parts.is_empty() {
            None
        } else {
            let value = value_parts.join("").trim().to_string();
            Some((property, value))
        }
    }

    fn token_to_string(&self, token: &CssToken) -> String {
        match token {
            CssToken::Ident(s) => s.to_string(),
            CssToken::String(s) => format!("\"{}\"", s),
            CssToken::Number(n) => n.to_string(),
            CssToken::Dimension { value, unit } => format!("{}{}", value, unit),
            CssToken::Percentage(p) => format!("{}%", p),
            CssToken::Hash(h) => format!("#{}", h),
            CssToken::Delim(c) => c.to_string(),
            CssToken::Url(url) => format!("url({})", url),
            _ => String::new(),
        }
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.current_token, Some(CssToken::Whitespace) | Some(CssToken::Comment(_))) {
            self.advance();
        }
    }

    fn advance(&mut self) {
        self.current_token = self.tokenizer.next_token();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_rule() {
        let mut parser = CssParser::new("div { color: red; }");
        let rules = parser.parse();
        
        assert_eq!(rules.len(), 1);
        
        let rule = &rules[0];
        assert_eq!(rule.selectors.len(), 1);
        assert!(matches!(rule.selectors[0], Selector::Type(ref name) if name == "div"));
        assert_eq!(rule.declarations.get("color"), Some(&"red".to_string()));
    }

    #[test]
    fn test_multiple_selectors() {
        let mut parser = CssParser::new("div, p, span { margin: 0; }");
        let rules = parser.parse();
        
        assert_eq!(rules.len(), 1);
        
        let rule = &rules[0];
        assert_eq!(rule.selectors.len(), 3);
        assert!(matches!(rule.selectors[0], Selector::Type(ref name) if name == "div"));
        assert!(matches!(rule.selectors[1], Selector::Type(ref name) if name == "p"));
        assert!(matches!(rule.selectors[2], Selector::Type(ref name) if name == "span"));
    }

    #[test]
    fn test_class_selector() {
        let mut parser = CssParser::new(".container { width: 100%; }");
        let rules = parser.parse();
        
        assert_eq!(rules.len(), 1);
        
        let rule = &rules[0];
        assert_eq!(rule.selectors.len(), 1);
        assert!(matches!(rule.selectors[0], Selector::Class(ref name) if name == "container"));
        assert_eq!(rule.declarations.get("width"), Some(&"100%".to_string()));
    }

    #[test]
    fn test_id_selector() {
        let mut parser = CssParser::new("#main { display: block; }");
        let rules = parser.parse();
        
        assert_eq!(rules.len(), 1);
        
        let rule = &rules[0];
        assert_eq!(rule.selectors.len(), 1);
        assert!(matches!(rule.selectors[0], Selector::Id(ref name) if name == "main"));
        assert_eq!(rule.declarations.get("display"), Some(&"block".to_string()));
    }

    #[test]
    fn test_universal_selector() {
        let mut parser = CssParser::new("* { box-sizing: border-box; }");
        let rules = parser.parse();
        
        assert_eq!(rules.len(), 1);
        
        let rule = &rules[0];
        assert_eq!(rule.selectors.len(), 1);
        assert!(matches!(rule.selectors[0], Selector::Universal));
        assert_eq!(rule.declarations.get("box-sizing"), Some(&"border-box".to_string()));
    }

    #[test]
    fn test_descendant_selector() {
        let mut parser = CssParser::new("div p { font-size: 14px; }");
        let rules = parser.parse();
        
        assert_eq!(rules.len(), 1);
        
        let rule = &rules[0];
        assert_eq!(rule.selectors.len(), 1);
        
        if let Selector::Descendant(left, right) = &rule.selectors[0] {
            assert!(matches!(**left, Selector::Type(ref name) if name == "div"));
            assert!(matches!(**right, Selector::Type(ref name) if name == "p"));
        } else {
            panic!("Expected descendant selector");
        }
    }

    #[test]
    fn test_child_selector() {
        let mut parser = CssParser::new("div > p { margin: 10px; }");
        let rules = parser.parse();
        
        assert_eq!(rules.len(), 1);
        
        let rule = &rules[0];
        assert_eq!(rule.selectors.len(), 1);
        
        if let Selector::Child(left, right) = &rule.selectors[0] {
            assert!(matches!(**left, Selector::Type(ref name) if name == "div"));
            assert!(matches!(**right, Selector::Type(ref name) if name == "p"));
        } else {
            panic!("Expected child selector");
        }
    }

    #[test]
    fn test_multiple_declarations() {
        let mut parser = CssParser::new("div { color: red; background: blue; font-size: 16px; }");
        let rules = parser.parse();
        
        assert_eq!(rules.len(), 1);
        
        let rule = &rules[0];
        assert_eq!(rule.declarations.len(), 3);
        assert_eq!(rule.declarations.get("color"), Some(&"red".to_string()));
        assert_eq!(rule.declarations.get("background"), Some(&"blue".to_string()));
        assert_eq!(rule.declarations.get("font-size"), Some(&"16px".to_string()));
    }

    #[test]
    fn test_multiple_rules() {
        let css = r#"
            div { color: red; }
            .container { width: 100%; }
            #main { display: block; }
        "#;
        
        let mut parser = CssParser::new(css);
        let rules = parser.parse();
        
        assert_eq!(rules.len(), 3);
        
        assert!(matches!(rules[0].selectors[0], Selector::Type(ref name) if name == "div"));
        assert!(matches!(rules[1].selectors[0], Selector::Class(ref name) if name == "container"));
        assert!(matches!(rules[2].selectors[0], Selector::Id(ref name) if name == "main"));
    }
}