use html_css_parser::css::CssParser;

fn main() {
    let mut parser = CssParser::new(".container { width: 100%; }");
    let rules = parser.parse();
    
    println!("Parsed {} rules", rules.len());
    for rule in &rules {
        println!("Rule: {:?}", rule);
    }
}
