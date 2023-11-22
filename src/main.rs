use scraper::Scraper;

mod scraper;

#[tokio::main]
async fn main() {
    let scraper = Scraper::new();
    scraper.initialize().await;
}
