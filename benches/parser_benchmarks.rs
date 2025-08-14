use criterion::{black_box, criterion_group, criterion_main, Criterion};
use html_css_parser::{HtmlParser, HtmlTokenizer, CssParser, CssTokenizer};

const SMALL_HTML: &str = r#"
<div class="container">
    <h1>Hello World</h1>
    <p>This is a test paragraph.</p>
    <ul>
        <li>Item 1</li>
        <li>Item 2</li>
        <li>Item 3</li>
    </ul>
</div>
"#;

const LARGE_HTML: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Test Document</title>
</head>
<body>
    <header class="main-header">
        <nav class="navigation">
            <ul class="nav-list">
                <li><a href="#home">Home</a></li>
                <li><a href="#about">About</a></li>
                <li><a href="#services">Services</a></li>
                <li><a href="#contact">Contact</a></li>
            </ul>
        </nav>
    </header>
    <main class="content">
        <section class="hero">
            <h1>Welcome to Our Website</h1>
            <p>This is a comprehensive test document with various HTML elements.</p>
            <button class="cta-button">Get Started</button>
        </section>
        <section class="features">
            <div class="feature-grid">
                <div class="feature-item">
                    <h3>Feature 1</h3>
                    <p>Description of feature 1 with some detailed text content.</p>
                    <img src="feature1.jpg" alt="Feature 1 Image">
                </div>
                <div class="feature-item">
                    <h3>Feature 2</h3>
                    <p>Description of feature 2 with some detailed text content.</p>
                    <img src="feature2.jpg" alt="Feature 2 Image">
                </div>
                <div class="feature-item">
                    <h3>Feature 3</h3>
                    <p>Description of feature 3 with some detailed text content.</p>
                    <img src="feature3.jpg" alt="Feature 3 Image">
                </div>
            </div>
        </section>
        <section class="testimonials">
            <h2>What Our Customers Say</h2>
            <div class="testimonial-list">
                <blockquote class="testimonial">
                    <p>"This service is amazing! Highly recommended."</p>
                    <cite>- John Doe</cite>
                </blockquote>
                <blockquote class="testimonial">
                    <p>"Great experience, will use again."</p>
                    <cite>- Jane Smith</cite>
                </blockquote>
            </div>
        </section>
    </main>
    <footer class="main-footer">
        <div class="footer-content">
            <p>&copy; 2024 Test Company. All rights reserved.</p>
            <div class="social-links">
                <a href="#facebook">Facebook</a>
                <a href="#twitter">Twitter</a>
                <a href="#linkedin">LinkedIn</a>
            </div>
        </div>
    </footer>
</body>
</html>
"#;

const SMALL_CSS: &str = r#"
.container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 20px;
}

h1 {
    color: #333;
    font-size: 2rem;
}

p {
    line-height: 1.6;
    color: #666;
}
"#;

const LARGE_CSS: &str = r#"
/* Reset and base styles */
* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

body {
    font-family: 'Arial', sans-serif;
    line-height: 1.6;
    color: #333;
    background-color: #f4f4f4;
}

/* Header styles */
.main-header {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    padding: 1rem 0;
    position: fixed;
    top: 0;
    width: 100%;
    z-index: 1000;
    box-shadow: 0 2px 10px rgba(0,0,0,0.1);
}

.navigation {
    max-width: 1200px;
    margin: 0 auto;
    padding: 0 2rem;
}

.nav-list {
    display: flex;
    list-style: none;
    justify-content: center;
    gap: 2rem;
}

.nav-list li a {
    color: white;
    text-decoration: none;
    font-weight: 500;
    transition: color 0.3s ease;
    padding: 0.5rem 1rem;
    border-radius: 4px;
}

.nav-list li a:hover {
    background-color: rgba(255,255,255,0.1);
    color: #f0f0f0;
}

/* Main content */
.content {
    margin-top: 80px;
    min-height: calc(100vh - 160px);
}

.hero {
    background: linear-gradient(rgba(0,0,0,0.4), rgba(0,0,0,0.4)), url('hero-bg.jpg');
    background-size: cover;
    background-position: center;
    color: white;
    text-align: center;
    padding: 8rem 2rem;
}

.hero h1 {
    font-size: 3.5rem;
    margin-bottom: 1rem;
    text-shadow: 2px 2px 4px rgba(0,0,0,0.5);
}

.hero p {
    font-size: 1.2rem;
    margin-bottom: 2rem;
    max-width: 600px;
    margin-left: auto;
    margin-right: auto;
}

.cta-button {
    background: #ff6b6b;
    color: white;
    border: none;
    padding: 1rem 2rem;
    font-size: 1.1rem;
    border-radius: 50px;
    cursor: pointer;
    transition: all 0.3s ease;
    text-transform: uppercase;
    font-weight: bold;
    letter-spacing: 1px;
}

.cta-button:hover {
    background: #ff5252;
    transform: translateY(-2px);
    box-shadow: 0 4px 15px rgba(255,107,107,0.4);
}

/* Features section */
.features {
    padding: 6rem 2rem;
    background: white;
}

.feature-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 3rem;
    max-width: 1200px;
    margin: 0 auto;
}

.feature-item {
    text-align: center;
    padding: 2rem;
    border-radius: 10px;
    box-shadow: 0 5px 15px rgba(0,0,0,0.1);
    transition: transform 0.3s ease;
}

.feature-item:hover {
    transform: translateY(-5px);
}

.feature-item h3 {
    color: #667eea;
    margin-bottom: 1rem;
    font-size: 1.5rem;
}

.feature-item img {
    width: 100%;
    max-width: 200px;
    height: auto;
    border-radius: 8px;
    margin-top: 1rem;
}

/* Testimonials */
.testimonials {
    background: #f8f9fa;
    padding: 6rem 2rem;
    text-align: center;
}

.testimonials h2 {
    color: #333;
    margin-bottom: 3rem;
    font-size: 2.5rem;
}

.testimonial-list {
    display: flex;
    gap: 2rem;
    max-width: 800px;
    margin: 0 auto;
    flex-wrap: wrap;
}

.testimonial {
    flex: 1;
    background: white;
    padding: 2rem;
    border-radius: 10px;
    box-shadow: 0 3px 10px rgba(0,0,0,0.1);
    border-left: 4px solid #667eea;
    min-width: 300px;
}

.testimonial p {
    font-style: italic;
    margin-bottom: 1rem;
    font-size: 1.1rem;
}

.testimonial cite {
    color: #667eea;
    font-weight: bold;
}

/* Footer */
.main-footer {
    background: #333;
    color: white;
    padding: 3rem 2rem 1rem;
    text-align: center;
}

.footer-content {
    max-width: 1200px;
    margin: 0 auto;
}

.social-links {
    margin-top: 1rem;
    display: flex;
    justify-content: center;
    gap: 1rem;
}

.social-links a {
    color: white;
    text-decoration: none;
    padding: 0.5rem 1rem;
    border: 1px solid #555;
    border-radius: 4px;
    transition: all 0.3s ease;
}

.social-links a:hover {
    background: #555;
    border-color: #777;
}

/* Responsive design */
@media (max-width: 768px) {
    .hero h1 {
        font-size: 2.5rem;
    }
    
    .nav-list {
        flex-direction: column;
        gap: 1rem;
    }
    
    .testimonial-list {
        flex-direction: column;
    }
    
    .feature-grid {
        grid-template-columns: 1fr;
    }
}
"#;

fn html_tokenizer_small(c: &mut Criterion) {
    c.bench_function("html_tokenizer_small", |b| {
        b.iter(|| {
            let tokenizer = HtmlTokenizer::new(black_box(SMALL_HTML));
            let tokens: Vec<_> = tokenizer.collect();
            black_box(tokens);
        })
    });
}

fn html_tokenizer_large(c: &mut Criterion) {
    c.bench_function("html_tokenizer_large", |b| {
        b.iter(|| {
            let tokenizer = HtmlTokenizer::new(black_box(LARGE_HTML));
            let tokens: Vec<_> = tokenizer.collect();
            black_box(tokens);
        })
    });
}

fn html_parser_small(c: &mut Criterion) {
    c.bench_function("html_parser_small", |b| {
        b.iter(|| {
            let mut parser = HtmlParser::new(black_box(SMALL_HTML));
            let nodes = parser.parse();
            black_box(nodes);
        })
    });
}

fn html_parser_large(c: &mut Criterion) {
    c.bench_function("html_parser_large", |b| {
        b.iter(|| {
            let mut parser = HtmlParser::new(black_box(LARGE_HTML));
            let nodes = parser.parse();
            black_box(nodes);
        })
    });
}

fn css_tokenizer_small(c: &mut Criterion) {
    c.bench_function("css_tokenizer_small", |b| {
        b.iter(|| {
            let tokenizer = CssTokenizer::new(black_box(SMALL_CSS));
            let tokens: Vec<_> = tokenizer.collect();
            black_box(tokens);
        })
    });
}

fn css_tokenizer_large(c: &mut Criterion) {
    c.bench_function("css_tokenizer_large", |b| {
        b.iter(|| {
            let tokenizer = CssTokenizer::new(black_box(LARGE_CSS));
            let tokens: Vec<_> = tokenizer.collect();
            black_box(tokens);
        })
    });
}

fn css_parser_small(c: &mut Criterion) {
    c.bench_function("css_parser_small", |b| {
        b.iter(|| {
            let mut parser = CssParser::new(black_box(SMALL_CSS));
            let rules = parser.parse();
            black_box(rules);
        })
    });
}

fn css_parser_large(c: &mut Criterion) {
    c.bench_function("css_parser_large", |b| {
        b.iter(|| {
            let mut parser = CssParser::new(black_box(LARGE_CSS));
            let rules = parser.parse();
            black_box(rules);
        })
    });
}

criterion_group!(
    benches,
    html_tokenizer_small,
    html_tokenizer_large,
    html_parser_small,
    html_parser_large,
    css_tokenizer_small,
    css_tokenizer_large,
    css_parser_small,
    css_parser_large
);
criterion_main!(benches);