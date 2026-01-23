use crate::models::Article;
use anyhow::Result;
use epub_builder::{EpubBuilder, EpubContent, ReferenceType, ZipLibrary};
use pulldown_cmark::{html, Parser, Options};
use std::fs::File;
use std::io::Cursor;

pub struct EpubGenerator;

impl EpubGenerator {
    pub fn create_epub(articles: &[Article], output_path: &str) -> Result<()> {
        let mut builder = EpubBuilder::new(ZipLibrary::new()?)?;

        builder.metadata("author", "YouTube to Ebook")?;
        builder.metadata("title", "Weekly Newsletter")?;
        
        // Add CSS
        builder.stylesheet(Cursor::new(Self::get_css()))?;

        // Process each article
        for (i, article) in articles.iter().enumerate() {
            let html_content = Self::markdown_to_html(&article.content);
            let chapter_title = &article.title;
            
            // Create proper HTML wrapper for the chapter
            let chapter_html = format!(
                r#"<?xml version="1.0" encoding="UTF-8"?>
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
  <title>{}</title>
  <link rel="stylesheet" type="text/css" href="stylesheet.css" />
</head>
<body>
  <h1>{}</h1>
  <p><i>Channel: {} | <a href="{}">Watch Video</a></i></p>
  {}
</body>
</html>"#,
                chapter_title, chapter_title, article.channel, article.url, html_content
            );

            let content_name = format!("article_{}.xhtml", i);
            
            builder.add_content(
                EpubContent::new(&content_name, Cursor::new(chapter_html))
                    .title(chapter_title)
                    .reftype(ReferenceType::Text)
            )?;
        }

        // Generate the file
        let mut file = File::create(output_path)?;
        builder.generate(&mut file)?;

        Ok(())
    }

    fn markdown_to_html(markdown: &str) -> String {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        
        let parser = Parser::new_ext(markdown, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        
        html_output
    }

    fn get_css() -> &'static str {
        r#"
body { 
    font-family: serif; 
    line-height: 1.5; 
    margin: 0;
    padding: 1em;
}
h1 { 
    text-align: center; 
    margin-bottom: 0.5em; 
}
img { 
    max-width: 100%; 
}
pre { 
    background: #f4f4f4; 
    padding: 1em; 
    white-space: pre-wrap; 
}
"#
    }
}
