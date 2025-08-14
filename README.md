# HTML & CSS Parser

A performant and resource-efficient HTML and CSS parser written in Rust from scratch. This library provides zero-copy tokenization and efficient parsing for both HTML and CSS content.

## Features

- **Zero-copy tokenization**: Efficient parsing without unnecessary string allocations
- **HTML parsing**: Complete HTML tokenizer and DOM tree parser
- **CSS parsing**: CSS tokenizer and rule parser with selector support
- **Performance focused**: Designed for speed and low memory usage
- **Comprehensive testing**: Extensive test suite ensuring correctness
- **Benchmarked**: Performance benchmarks included

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
html-css-parser = "0.1.0"
```

### HTML Parsing

```rust
use html_css_parser::{HtmlParser, HtmlTokenizer};

// Tokenize HTML
let tokenizer = HtmlTokenizer::new("<div class='test'>Hello World</div>");
let tokens: Vec<_> = tokenizer.collect();

// Parse HTML into DOM tree
let mut parser = HtmlParser::new("<div class='test'>Hello World</div>");
let nodes = parser.parse();

println!("Parsed {} nodes", nodes.len());
```

### CSS Parsing

```rust
use html_css_parser::{CssParser, CssTokenizer};

// Tokenize CSS
let tokenizer = CssTokenizer::new(".container { width: 100%; color: red; }");
let tokens: Vec<_> = tokenizer.collect();

// Parse CSS rules
let mut parser = CssParser::new(".container { width: 100%; color: red; }");
let rules = parser.parse();

println!("Parsed {} rules", rules.len());
```

## HTML Features

### Supported HTML Elements

- Start tags with attributes: `<div class="test" id="main">`
- End tags: `</div>`
- Self-closing tags: `<br/>`, `<img src="test.jpg"/>`
- Void elements: `<br>`, `<hr>`, `<img>`, etc.
- Text content
- Comments: `<!-- comment -->`
- DOCTYPE declarations: `<!DOCTYPE html>`

### HTML Parser Output

The HTML parser produces a tree of `Node` elements:

```rust
pub enum Node {
    Element(Element),
    Text(String),
    Comment(String),
}

pub struct Element {
    pub tag_name: String,
    pub attributes: HashMap<String, String>,
    pub children: Vec<Node>,
}
```

## CSS Features

### Supported CSS Selectors

- Type selectors: `div`, `p`, `span`
- Class selectors: `.container`, `.nav-item`
- ID selectors: `#main`, `#header`
- Universal selector: `*`
- Descendant combinator: `div p`
- Child combinator: `div > p`
- Adjacent sibling: `h1 + p`
- General sibling: `h1 ~ p`

### Supported CSS Tokens

- Identifiers: `div`, `color`, `margin`
- Strings: `"Arial"`, `'Helvetica'`
- Numbers: `42`, `3.14`, `-10`
- Dimensions: `16px`, `2em`, `100%`
- Colors: `#ff0000`, `#333`
- URLs: `url(image.png)`
- Comments: `/* comment */`

### CSS Parser Output

The CSS parser produces a list of `Rule` elements:

```rust
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: HashMap<String, String>,
}

pub enum Selector {
    Type(String),
    Class(String),
    Id(String),
    Universal,
    Descendant(Box<Selector>, Box<Selector>),
    Child(Box<Selector>, Box<Selector>),
    // ... more combinators
}
```

## Performance

This parser is designed for performance and efficiency:

- **Zero-copy tokenization**: String slices reference the original input
- **Minimal allocations**: Only allocate when building the final tree structure
- **Efficient parsing**: Single-pass parsing with minimal backtracking
- **Memory efficient**: Low memory footprint even for large documents

### Running Benchmarks

```bash
cargo bench
```

The benchmarks test parsing performance on both small and large HTML/CSS documents.

## Examples

### Complete HTML Document

```rust
use html_css_parser::HtmlParser;

let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Test</title>
</head>
<body>
    <div class="container">
        <h1>Hello World</h1>
        <p>This is a test.</p>
    </div>
</body>
</html>
"#;

let mut parser = HtmlParser::new(html);
let nodes = parser.parse();

// Process the parsed nodes...
```

### Complex CSS Rules

```rust
use html_css_parser::CssParser;

let css = r#"
.container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 20px;
}

.container > .header {
    background: linear-gradient(45deg, #ff6b6b, #4ecdc4);
    color: white;
    padding: 1rem;
}

@media (max-width: 768px) {
    .container {
        padding: 10px;
    }
}
"#;

let mut parser = CssParser::new(css);
let rules = parser.parse();

for rule in &rules {
    println!("Selectors: {:?}", rule.selectors);
    println!("Declarations: {:?}", rule.declarations);
}
```

## Testing

Run the test suite:

```bash
cargo test
```

The library includes comprehensive tests for:
- HTML tokenization edge cases
- HTML parsing with nested elements
- CSS tokenization of all token types
- CSS parsing with complex selectors
- Error handling and malformed input

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.