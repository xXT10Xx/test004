#[derive(Debug, Clone, PartialEq)]
pub enum CssToken<'a> {
    Ident(&'a str),
    String(&'a str),
    Number(f64),
    Dimension { value: f64, unit: &'a str },
    Percentage(f64),
    Hash(&'a str),
    Delim(char),
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    Semicolon,
    Comma,
    Whitespace,
    Comment(&'a str),
    AtKeyword(&'a str),
    Url(&'a str),
}

pub struct CssTokenizer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> CssTokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, position: 0 }
    }

    pub fn next_token(&mut self) -> Option<CssToken<'a>> {
        if self.position >= self.input.len() {
            return None;
        }

        let current_char = self.current_char()?;

        match current_char {
            ' ' | '\t' | '\n' | '\r' => self.consume_whitespace(),
            '/' if self.peek_char(1) == Some('*') => self.consume_comment(),
            '{' => {
                self.advance();
                Some(CssToken::LeftBrace)
            }
            '}' => {
                self.advance();
                Some(CssToken::RightBrace)
            }
            '(' => {
                self.advance();
                Some(CssToken::LeftParen)
            }
            ')' => {
                self.advance();
                Some(CssToken::RightParen)
            }
            '[' => {
                self.advance();
                Some(CssToken::LeftBracket)
            }
            ']' => {
                self.advance();
                Some(CssToken::RightBracket)
            }
            ':' => {
                self.advance();
                Some(CssToken::Colon)
            }
            ';' => {
                self.advance();
                Some(CssToken::Semicolon)
            }
            ',' => {
                self.advance();
                Some(CssToken::Comma)
            }
            '"' | '\'' => self.consume_string(current_char),
            '#' => self.consume_hash(),
            '@' => self.consume_at_keyword(),
            '0'..='9' => self.consume_number(),
            '.' if self.peek_char(1).map_or(false, |c| c.is_ascii_digit()) => self.consume_number(),
            '-' if self.is_number_start() => self.consume_number(),
            'a'..='z' | 'A'..='Z' | '_' | '-' => self.consume_ident_or_url(),
            _ => {
                self.advance();
                Some(CssToken::Delim(current_char))
            }
        }
    }

    fn current_char(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    fn peek_char(&self, offset: usize) -> Option<char> {
        self.input.chars().nth(self.position + offset)
    }

    fn advance(&mut self) {
        if self.position < self.input.len() {
            self.position += 1;
        }
    }

    fn consume_whitespace(&mut self) -> Option<CssToken<'a>> {
        while let Some(ch) = self.current_char() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
        Some(CssToken::Whitespace)
    }

    fn consume_comment(&mut self) -> Option<CssToken<'a>> {
        self.advance(); // Skip '/'
        self.advance(); // Skip '*'
        
        let start = self.position;
        
        while self.position + 1 < self.input.len() {
            if self.current_char() == Some('*') && self.peek_char(1) == Some('/') {
                let content = &self.input[start..self.position];
                self.advance(); // Skip '*'
                self.advance(); // Skip '/'
                return Some(CssToken::Comment(content));
            }
            self.advance();
        }

        // Unclosed comment
        let content = &self.input[start..];
        self.position = self.input.len();
        Some(CssToken::Comment(content))
    }

    fn consume_string(&mut self, quote: char) -> Option<CssToken<'a>> {
        self.advance(); // Skip opening quote
        let start = self.position;

        while let Some(ch) = self.current_char() {
            if ch == quote {
                let content = &self.input[start..self.position];
                self.advance(); // Skip closing quote
                return Some(CssToken::String(content));
            } else if ch == '\\' {
                self.advance(); // Skip backslash
                if self.current_char().is_some() {
                    self.advance(); // Skip escaped character
                }
            } else {
                self.advance();
            }
        }

        // Unclosed string
        let content = &self.input[start..];
        self.position = self.input.len();
        Some(CssToken::String(content))
    }

    fn consume_hash(&mut self) -> Option<CssToken<'a>> {
        self.advance(); // Skip '#'
        let start = self.position;

        while let Some(ch) = self.current_char() {
            if ch.is_alphanumeric() || ch == '-' || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }

        if start == self.position {
            Some(CssToken::Delim('#'))
        } else {
            let content = &self.input[start..self.position];
            Some(CssToken::Hash(content))
        }
    }

    fn consume_at_keyword(&mut self) -> Option<CssToken<'a>> {
        self.advance(); // Skip '@'
        let start = self.position;

        while let Some(ch) = self.current_char() {
            if ch.is_alphanumeric() || ch == '-' || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }

        if start == self.position {
            Some(CssToken::Delim('@'))
        } else {
            let content = &self.input[start..self.position];
            Some(CssToken::AtKeyword(content))
        }
    }

    fn consume_number(&mut self) -> Option<CssToken<'a>> {
        let start = self.position;
        let mut has_dot = false;

        // Handle optional minus sign
        if self.current_char() == Some('-') {
            self.advance();
        }

        // Consume digits and optional decimal point
        while let Some(ch) = self.current_char() {
            if ch.is_ascii_digit() {
                self.advance();
            } else if ch == '.' && !has_dot {
                has_dot = true;
                self.advance();
            } else {
                break;
            }
        }

        let number_str = &self.input[start..self.position];
        let value = number_str.parse::<f64>().unwrap_or(0.0);

        // Check for unit or percentage
        if self.current_char() == Some('%') {
            self.advance();
            Some(CssToken::Percentage(value))
        } else if let Some(ch) = self.current_char() {
            if ch.is_alphabetic() {
                let unit_start = self.position;
                while let Some(ch) = self.current_char() {
                    if ch.is_alphanumeric() {
                        self.advance();
                    } else {
                        break;
                    }
                }
                let unit = &self.input[unit_start..self.position];
                Some(CssToken::Dimension { value, unit })
            } else {
                Some(CssToken::Number(value))
            }
        } else {
            Some(CssToken::Number(value))
        }
    }

    fn consume_ident_or_url(&mut self) -> Option<CssToken<'a>> {
        let start = self.position;

        while let Some(ch) = self.current_char() {
            if ch.is_alphanumeric() || ch == '-' || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let ident = &self.input[start..self.position];

        // Check if this is a url() function
        if ident == "url" && self.current_char() == Some('(') {
            self.advance(); // Skip '('
            self.skip_whitespace();

            let _url_start = self.position;
            let mut in_quotes = false;
            let mut quote_char = None;

            if let Some(ch) = self.current_char() {
                if ch == '"' || ch == '\'' {
                    in_quotes = true;
                    quote_char = Some(ch);
                    self.advance();
                }
            }

            let url_content_start = self.position;

            while let Some(ch) = self.current_char() {
                if in_quotes {
                    if Some(ch) == quote_char {
                        let url = &self.input[url_content_start..self.position];
                        self.advance(); // Skip closing quote
                        self.skip_whitespace();
                        if self.current_char() == Some(')') {
                            self.advance(); // Skip ')'
                        }
                        return Some(CssToken::Url(url));
                    }
                } else if ch == ')' {
                    let url = &self.input[url_content_start..self.position].trim();
                    self.advance(); // Skip ')'
                    return Some(CssToken::Url(url));
                }
                self.advance();
            }

            // Unclosed url
            let url = &self.input[url_content_start..];
            self.position = self.input.len();
            Some(CssToken::Url(url))
        } else {
            Some(CssToken::Ident(ident))
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn is_number_start(&self) -> bool {
        if let Some(next) = self.peek_char(1) {
            next.is_ascii_digit() || next == '.'
        } else {
            false
        }
    }
}

impl<'a> Iterator for CssTokenizer<'a> {
    type Item = CssToken<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_tokens() {
        let tokenizer = CssTokenizer::new("{ } ( ) [ ] : ; ,");
        
        let tokens: Vec<_> = tokenizer.collect();
        
        assert!(matches!(tokens[0], CssToken::LeftBrace));
        assert!(matches!(tokens[1], CssToken::Whitespace));
        assert!(matches!(tokens[2], CssToken::RightBrace));
        assert!(matches!(tokens[3], CssToken::Whitespace));
        assert!(matches!(tokens[4], CssToken::LeftParen));
        assert!(matches!(tokens[5], CssToken::Whitespace));
        assert!(matches!(tokens[6], CssToken::RightParen));
        assert!(matches!(tokens[7], CssToken::Whitespace));
        assert!(matches!(tokens[8], CssToken::LeftBracket));
    }

    #[test]
    fn test_identifiers() {
        let tokenizer = CssTokenizer::new("div class-name _private");
        
        let tokens: Vec<_> = tokenizer.collect();
        
        assert!(matches!(tokens[0], CssToken::Ident("div")));
        assert!(matches!(tokens[1], CssToken::Whitespace));
        assert!(matches!(tokens[2], CssToken::Ident("class-name")));
        assert!(matches!(tokens[3], CssToken::Whitespace));
        assert!(matches!(tokens[4], CssToken::Ident("_private")));
    }

    #[test]
    fn test_numbers() {
        let tokenizer = CssTokenizer::new("42 3.14 -10 50% 16px");
        
        let tokens: Vec<_> = tokenizer.collect();
        
        assert!(matches!(tokens[0], CssToken::Number(42.0)));
        assert!(matches!(tokens[1], CssToken::Whitespace));
        assert!(matches!(tokens[2], CssToken::Number(3.14)));
        assert!(matches!(tokens[3], CssToken::Whitespace));
        assert!(matches!(tokens[4], CssToken::Number(-10.0)));
        assert!(matches!(tokens[5], CssToken::Whitespace));
        assert!(matches!(tokens[6], CssToken::Percentage(50.0)));
        assert!(matches!(tokens[7], CssToken::Whitespace));
        assert!(matches!(tokens[8], CssToken::Dimension { value: 16.0, unit: "px" }));
    }

    #[test]
    fn test_strings() {
        let tokenizer = CssTokenizer::new(r#""hello" 'world'"#);
        
        let tokens: Vec<_> = tokenizer.collect();
        
        assert!(matches!(tokens[0], CssToken::String("hello")));
        assert!(matches!(tokens[1], CssToken::Whitespace));
        assert!(matches!(tokens[2], CssToken::String("world")));
    }

    #[test]
    fn test_hash() {
        let tokenizer = CssTokenizer::new("#main #ff0000");
        
        let tokens: Vec<_> = tokenizer.collect();
        
        assert!(matches!(tokens[0], CssToken::Hash("main")));
        assert!(matches!(tokens[1], CssToken::Whitespace));
        assert!(matches!(tokens[2], CssToken::Hash("ff0000")));
    }

    #[test]
    fn test_at_keyword() {
        let tokenizer = CssTokenizer::new("@media @import");
        
        let tokens: Vec<_> = tokenizer.collect();
        
        assert!(matches!(tokens[0], CssToken::AtKeyword("media")));
        assert!(matches!(tokens[1], CssToken::Whitespace));
        assert!(matches!(tokens[2], CssToken::AtKeyword("import")));
    }

    #[test]
    fn test_url() {
        let tokenizer = CssTokenizer::new(r#"url(image.png) url("path/to/file.jpg")"#);
        
        let tokens: Vec<_> = tokenizer.collect();
        
        assert!(matches!(tokens[0], CssToken::Url("image.png")));
        assert!(matches!(tokens[1], CssToken::Whitespace));
        assert!(matches!(tokens[2], CssToken::Url("path/to/file.jpg")));
    }

    #[test]
    fn test_comments() {
        let tokenizer = CssTokenizer::new("/* comment */ div");
        
        let tokens: Vec<_> = tokenizer.collect();
        
        assert!(matches!(tokens[0], CssToken::Comment(" comment ")));
        assert!(matches!(tokens[1], CssToken::Whitespace));
        assert!(matches!(tokens[2], CssToken::Ident("div")));
    }
}