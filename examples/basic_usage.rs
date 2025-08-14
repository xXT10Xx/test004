use html_css_parser::{HtmlParser, HtmlTokenizer, CssParser, CssTokenizer, Node};

fn main() {
    println!("=== HTML Parsing Example ===");
    
    let html = r#"
    <div class="container" id="main">
        <h1>Welcome</h1>
        <p>This is a <strong>test</strong> paragraph.</p>
        <ul>
            <li>Item 1</li>
            <li>Item 2</li>
        </ul>
        <!-- This is a comment -->
    </div>
    "#;
    
    // Tokenize HTML
    println!("\n--- HTML Tokens ---");
    let tokenizer = HtmlTokenizer::new(html);
    for (i, token) in tokenizer.enumerate() {
        println!("{}: {:?}", i, token);
    }
    
    // Parse HTML into DOM tree
    println!("\n--- HTML DOM Tree ---");
    let mut parser = HtmlParser::new(html);
    let nodes = parser.parse();
    
    for node in &nodes {
        print_node(node, 0);
    }
    
    println!("\n=== CSS Parsing Example ===");
    
    let css = r#"
    .container {
        max-width: 1200px;
        margin: 0 auto;
        padding: 20px;
        background-color: #f5f5f5;
    }
    
    .container h1 {
        color: #333;
        font-size: 2rem;
        margin-bottom: 1rem;
    }
    
    .container > p {
        line-height: 1.6;
        color: #666;
    }
    
    #main {
        border: 1px solid #ddd;
        border-radius: 8px;
    }
    
    ul li {
        list-style-type: disc;
        margin-left: 20px;
    }
    "#;
    
    // Tokenize CSS
    println!("\n--- CSS Tokens ---");
    let tokenizer = CssTokenizer::new(css);
    for (i, token) in tokenizer.enumerate().take(20) { // Show first 20 tokens
        println!("{}: {:?}", i, token);
    }
    
    // Parse CSS rules
    println!("\n--- CSS Rules ---");
    let mut parser = CssParser::new(css);
    let rules = parser.parse();
    
    for (i, rule) in rules.iter().enumerate() {
        println!("\nRule {}:", i + 1);
        println!("  Selectors: {:?}", rule.selectors);
        println!("  Declarations:");
        for (property, value) in &rule.declarations {
            println!("    {}: {}", property, value);
        }
    }
}

fn print_node(node: &Node, indent: usize) {
    let indent_str = "  ".repeat(indent);
    
    match node {
        Node::Element(element) => {
            println!("{}Element: {}", indent_str, element.tag_name);
            if !element.attributes.is_empty() {
                println!("{}  Attributes: {:?}", indent_str, element.attributes);
            }
            for child in &element.children {
                print_node(child, indent + 1);
            }
        }
        Node::Text(text) => {
            let trimmed = text.trim();
            if !trimmed.is_empty() {
                println!("{}Text: {:?}", indent_str, trimmed);
            }
        }
        Node::Comment(comment) => {
            println!("{}Comment: {:?}", indent_str, comment);
        }
    }
}