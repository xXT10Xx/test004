use html_css_parser::{HtmlParser, CssParser};

fn main() {
    println!("HTML & CSS Parser Demo");
    println!("======================");
    
    // HTML parsing example
    let html = r#"<div class="container"><h1>Hello</h1><p>World!</p></div>"#;
    let mut html_parser = HtmlParser::new(html);
    let nodes = html_parser.parse();
    println!("HTML: Parsed {} nodes from: {}", nodes.len(), html);
    
    // CSS parsing example  
    let css = r#".container { width: 100%; color: red; }"#;
    let mut css_parser = CssParser::new(css);
    let rules = css_parser.parse();
    println!("CSS: Parsed {} rules from: {}", rules.len(), css);
    
    println!("\nRun 'cargo run --example basic_usage' for detailed examples!");
}
