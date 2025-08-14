#[derive(Debug, Clone, PartialEq)]
pub enum HtmlToken<'a> {
    StartTag {
        name: &'a str,
        attributes: Vec<(&'a str, &'a str)>,
        self_closing: bool,
    },
    EndTag {
        name: &'a str,
    },
    Text(&'a str),
    Comment(&'a str),
    Doctype(&'a str),
}

pub struct HtmlTokenizer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> HtmlTokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, position: 0 }
    }

    pub fn next_token(&mut self) -> Option<HtmlToken<'a>> {
        self.skip_whitespace();
        
        if self.position >= self.input.len() {
            return None;
        }

        let current_char = self.current_char()?;
        
        if current_char == '<' {
            self.parse_tag_or_comment()
        } else {
            self.parse_text()
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

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn parse_tag_or_comment(&mut self) -> Option<HtmlToken<'a>> {
        let start_pos = self.position;
        self.advance(); // Skip '<'

        // Check for comment
        if self.input[self.position..].starts_with("!--") {
            return self.parse_comment();
        }

        // Check for doctype
        if self.input[self.position..].to_lowercase().starts_with("!doctype") {
            return self.parse_doctype();
        }

        // Check for end tag
        let is_end_tag = self.current_char() == Some('/');
        if is_end_tag {
            self.advance(); // Skip '/'
        }

        // Parse tag name
        let name_start = self.position;
        while let Some(ch) = self.current_char() {
            if ch.is_alphanumeric() || ch == '-' || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }

        if name_start == self.position {
            // Invalid tag, treat as text
            self.position = start_pos;
            return self.parse_text();
        }

        let name = &self.input[name_start..self.position];

        if is_end_tag {
            // Skip to '>'
            while let Some(ch) = self.current_char() {
                if ch == '>' {
                    self.advance();
                    break;
                }
                self.advance();
            }
            return Some(HtmlToken::EndTag { name });
        }

        // Parse attributes
        let mut attributes = Vec::new();
        let mut self_closing = false;

        loop {
            self.skip_whitespace();
            
            match self.current_char() {
                Some('>') => {
                    self.advance();
                    break;
                }
                Some('/') => {
                    self.advance();
                    if self.current_char() == Some('>') {
                        self.advance();
                        self_closing = true;
                        break;
                    }
                }
                Some(_) => {
                    if let Some((attr_name, attr_value)) = self.parse_attribute() {
                        attributes.push((attr_name, attr_value));
                    }
                }
                None => break,
            }
        }

        Some(HtmlToken::StartTag {
            name,
            attributes,
            self_closing,
        })
    }

    fn parse_attribute(&mut self) -> Option<(&'a str, &'a str)> {
        // Parse attribute name
        let name_start = self.position;
        while let Some(ch) = self.current_char() {
            if ch.is_alphanumeric() || ch == '-' || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }

        if name_start == self.position {
            return None;
        }

        let name = &self.input[name_start..self.position];
        
        self.skip_whitespace();

        // Check for '='
        if self.current_char() != Some('=') {
            return Some((name, ""));
        }
        
        self.advance(); // Skip '='
        self.skip_whitespace();

        // Parse attribute value
        let quote_char = self.current_char();
        let value = if quote_char == Some('"') || quote_char == Some('\'') {
            self.advance(); // Skip opening quote
            let value_start = self.position;
            
            while let Some(ch) = self.current_char() {
                if ch == quote_char.unwrap() {
                    let value = &self.input[value_start..self.position];
                    self.advance(); // Skip closing quote
                    return Some((name, value));
                }
                self.advance();
            }
            
            &self.input[value_start..self.position]
        } else {
            // Unquoted value
            let value_start = self.position;
            while let Some(ch) = self.current_char() {
                if ch.is_whitespace() || ch == '>' || ch == '/' {
                    break;
                }
                self.advance();
            }
            &self.input[value_start..self.position]
        };

        Some((name, value))
    }

    fn parse_comment(&mut self) -> Option<HtmlToken<'a>> {
        self.position += 3; // Skip "!--"
        let content_start = self.position;

        while self.position + 2 < self.input.len() {
            if &self.input[self.position..self.position + 3] == "-->" {
                let content = &self.input[content_start..self.position];
                self.position += 3; // Skip "-->"
                return Some(HtmlToken::Comment(content));
            }
            self.advance();
        }

        // Unclosed comment
        let content = &self.input[content_start..];
        self.position = self.input.len();
        Some(HtmlToken::Comment(content))
    }

    fn parse_doctype(&mut self) -> Option<HtmlToken<'a>> {
        let start = self.position;
        
        while let Some(ch) = self.current_char() {
            if ch == '>' {
                let content = &self.input[start..self.position];
                self.advance(); // Skip '>'
                return Some(HtmlToken::Doctype(content));
            }
            self.advance();
        }

        // Unclosed doctype
        let content = &self.input[start..];
        self.position = self.input.len();
        Some(HtmlToken::Doctype(content))
    }

    fn parse_text(&mut self) -> Option<HtmlToken<'a>> {
        let start = self.position;
        
        while let Some(ch) = self.current_char() {
            if ch == '<' {
                break;
            }
            self.advance();
        }

        if start == self.position {
            return None;
        }

        let text = &self.input[start..self.position];
        Some(HtmlToken::Text(text))
    }
}

impl<'a> Iterator for HtmlTokenizer<'a> {
    type Item = HtmlToken<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}