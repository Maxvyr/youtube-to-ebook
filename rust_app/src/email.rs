use crate::models::Article;
use anyhow::{Context, Result};
use lettre::{
    message::{header::ContentType, Attachment, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use pulldown_cmark::{html, Parser};
use std::fs;
use std::path::Path;

pub struct EmailSender {
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    from_email: String,
}

impl EmailSender {
    pub fn new(email: String, password: String) -> Self {
        let creds = Credentials::new(email.clone(), password);

        // Open connection to Gmail
        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();

        Self {
            mailer,
            from_email: email,
        }
    }

    pub async fn send_newsletter(
        &self,
        recipient: &str,
        articles: &[Article],
        epub_path: &str,
    ) -> Result<()> {
        let today = chrono::Local::now().format("%B %d, %Y").to_string();
        let subject = format!("Your YouTube Digest - {}", today);

        // Read EPUB file
        let epub_content = fs::read(epub_path).context("Failed to read EPUB file")?;
        let filename = Path::new(epub_path)
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("newsletter.epub");

        // Create HTML content
        let html_body = self.create_html_body(articles, &today);

        // Build the email
        let email = Message::builder()
            .from(self.from_email.parse().unwrap())
            .to(recipient.parse().unwrap())
            .subject(subject)
            .multipart(
                MultiPart::mixed()
                    .singlepart(
                        SinglePart::builder()
                            .header(ContentType::TEXT_HTML)
                            .body(html_body),
                    )
                    .singlepart(
                        Attachment::new(String::from(filename))
                            .body(epub_content, ContentType::parse("application/epub+zip").unwrap()),
                    ),
            )?;

        // Send
        self.mailer.send(email).await.context("Failed to send email")?;

        Ok(())
    }

    fn create_html_body(&self, articles: &[Article], date: &str) -> String {
        let mut articles_html = String::new();

        for article in articles {
            let mut options = pulldown_cmark::Options::empty();
            let parser = Parser::new_ext(&article.content, options);
            let mut html_output = String::new();
            html::push_html(&mut html_output, parser);

            articles_html.push_str(&format!(
                r#"<div class="article">
                    <div class="article-intro">
                        <em>This article is based on the video "<strong>{}</strong>" from the YouTube channel <strong>{}</strong>.</em>
                    </div>
                    <div class="article-content">{}</div>
                    <a href="{}" class="watch-link">Watch the original video</a>
                </div>"#,
                article.title, article.channel, html_output, article.url
            ));
        }

        format!(
            r#"<!DOCTYPE html>
            <html>
            <head>
                <style>
                    body {{ font-family: Georgia, serif; font-size: 18px; max-width: 700px; margin: 0 auto; padding: 20px; background-color: #f9f9f9; color: #333; }}
                    .header {{ text-align: center; padding: 30px 0; border-bottom: 3px solid #333; margin-bottom: 30px; }}
                    .header h1 {{ margin: 0; font-size: 32px; letter-spacing: 2px; }}
                    .header p {{ color: #666; font-size: 18px; margin: 10px 0 0 0; }}
                    .article {{ background: white; padding: 30px; margin-bottom: 30px; border-radius: 5px; box-shadow: 0 2px 5px rgba(0,0,0,0.1); }}
                    .article-intro {{ background: #f8f8f8; padding: 15px 20px; border-left: 4px solid #666; margin-bottom: 25px; font-size: 16px; color: #555; line-height: 1.6; }}
                    .article-content {{ font-size: 18px; line-height: 1.9; }}
                    .article-content h1 {{ color: #222; font-size: 26px; margin-top: 25px; }}
                    .article-content h2 {{ color: #222; font-size: 22px; margin-top: 25px; }}
                    .watch-link {{ display: inline-block; margin-top: 20px; padding: 12px 24px; background: #ff0000; color: white !important; text-decoration: none; border-radius: 5px; font-size: 16px; }}
                    .footer {{ text-align: center; color: #999; font-size: 14px; padding: 20px; }}
                    .epub-note {{ text-align: center; background: #e8f4e8; padding: 15px; border-radius: 5px; margin-bottom: 30px; font-size: 16px; }}
                </style>
            </head>
            <body>
                <div class="header">
                    <h1>YOUR YOUTUBE DIGEST</h1>
                    <p>{}</p>
                </div>
                <div class="epub-note">
                    📚 EPUB ebook attached - open on your phone's ebook reader!
                </div>
                {}
                <div class="footer">Generated by YouTube Newsletter Bot</div>
            </body>
            </html>"#,
            date, articles_html
        )
    }
}
