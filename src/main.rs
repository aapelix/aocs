use pulldown_cmark::{Parser, html};
use std::fs;

fn main() -> std::io::Result<()> {
    let input_dir = "docs";

    let template = fs::read_to_string("docs/template.html")?;

    let entries = fs::read_dir(input_dir)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            let markdown_input = fs::read_to_string(&path)?;

            let parser = Parser::new(&markdown_input);

            let mut html_output = String::new();
            html::push_html(&mut html_output, parser);

            let html_path = path.with_extension("html");

            let title = path.file_stem()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default();

            let html_with_content = template
                .replace("{{title}}", title)
                .replace("{{content}}", &html_output);

            fs::write(html_path, html_with_content)?;
            println!("Generated .html file")
        }
    }

    Ok(())
}
