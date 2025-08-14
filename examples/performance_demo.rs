use html_css_parser::{HtmlParser, CssParser};
use std::time::Instant;

fn main() {
    println!("=== Performance Demo ===");
    
    // Large HTML document for testing
    let large_html = generate_large_html();
    let large_css = generate_large_css();
    
    println!("HTML document size: {} bytes", large_html.len());
    println!("CSS document size: {} bytes", large_css.len());
    
    // Benchmark HTML parsing
    println!("\n--- HTML Parsing Performance ---");
    let start = Instant::now();
    let mut html_parser = HtmlParser::new(&large_html);
    let nodes = html_parser.parse();
    let html_duration = start.elapsed();
    
    println!("Parsed {} nodes in {:?}", count_nodes(&nodes), html_duration);
    println!("HTML parsing rate: {:.2} MB/s", 
             (large_html.len() as f64 / 1_000_000.0) / html_duration.as_secs_f64());
    
    // Benchmark CSS parsing
    println!("\n--- CSS Parsing Performance ---");
    let start = Instant::now();
    let mut css_parser = CssParser::new(&large_css);
    let rules = css_parser.parse();
    let css_duration = start.elapsed();
    
    println!("Parsed {} rules in {:?}", rules.len(), css_duration);
    println!("CSS parsing rate: {:.2} MB/s", 
             (large_css.len() as f64 / 1_000_000.0) / css_duration.as_secs_f64());
    
    // Memory usage demonstration
    println!("\n--- Memory Efficiency ---");
    println!("HTML nodes use string slices from original input where possible");
    println!("CSS tokens reference original input without copying");
    println!("Only final tree structure requires new allocations");
}

fn generate_large_html() -> String {
    let mut html = String::with_capacity(100_000);
    html.push_str("<!DOCTYPE html><html><head><title>Performance Test</title></head><body>");
    
    for i in 0..1000 {
        html.push_str(&format!(
            r#"<div class="item-{}" id="item-{}">
                <h2>Item {}</h2>
                <p>This is item number {} with some content.</p>
                <ul>
                    <li>Feature A</li>
                    <li>Feature B</li>
                    <li>Feature C</li>
                </ul>
                <img src="image-{}.jpg" alt="Image {}">
            </div>"#,
            i, i, i, i, i, i
        ));
    }
    
    html.push_str("</body></html>");
    html
}

fn generate_large_css() -> String {
    let mut css = String::with_capacity(50_000);
    
    css.push_str(r#"
    * { margin: 0; padding: 0; box-sizing: border-box; }
    body { font-family: Arial, sans-serif; line-height: 1.6; }
    "#);
    
    for i in 0..500 {
        css.push_str(&format!(
            r#"
            .item-{} {{
                background: #f{:x}f{:x}f{:x};
                padding: {}px;
                margin: {}px;
                border: {}px solid #ddd;
                border-radius: {}px;
            }}
            
            .item-{} h2 {{
                color: #{:06x};
                font-size: {}rem;
                margin-bottom: {}px;
            }}
            
            .item-{} p {{
                color: #{:06x};
                font-size: {}px;
                line-height: {};
            }}
            "#,
            i, i % 16, (i * 2) % 16, (i * 3) % 16,
            10 + (i % 20), 5 + (i % 15), 1 + (i % 3), 4 + (i % 8),
            i, (i * 123456) % 0xFFFFFF, 1.2 + (i % 10) as f32 * 0.1, 10 + (i % 20),
            i, (i * 654321) % 0xFFFFFF, 14 + (i % 6), 1.4 + (i % 5) as f32 * 0.1
        ));
    }
    
    css
}

fn count_nodes(nodes: &[html_css_parser::Node]) -> usize {
    let mut count = 0;
    for node in nodes {
        count += 1;
        if let html_css_parser::Node::Element(element) = node {
            count += count_nodes(&element.children);
        }
    }
    count
}