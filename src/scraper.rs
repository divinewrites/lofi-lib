use csv::Writer;
use lofi_lib::Album;
use log::{error, info, Level};
use reqwest::{Client, StatusCode};
use scraper::{Html, Selector};
use std::{env, time::Duration};
use tokio::{join, time::sleep};

#[derive(Debug)]
pub struct Scraper {
    pub albums: Vec<Album>,
    client: Client,
}

const BASE_URL: &str = "https://vinyl.lofirecords.com";
const MAX_RETRIES: i32 = 10;
const USER_AGENT: &str = "Mozilla/5.0 (Linux x86_64; rv:115.0) Gecko/20100101 Firefox/115.0";

impl Scraper {
    pub fn new() -> Self {
        Scraper {
            albums: Vec::new(),
            client: Client::builder().user_agent(USER_AGENT).build().unwrap(),
        }
    }

    pub async fn initialize(&self) {
        simple_logger::init_with_level(Level::Info).unwrap();
        self.run().await;
    }

    async fn run(&self) {
        let args: Vec<String> = env::args().collect();

        let urls_to_scrape: Vec<String> = if args.len() > 1 {
            let valid_urls: Vec<String> = args[1..]
                .iter()
                .filter(|&url| url.starts_with("https://vinyl.lofirecords.com/collections/lofi"))
                .map(|url| url.to_owned())
                .collect();

            if valid_urls.is_empty() {
                error!("No valid URLs found. Aborting.");
                return;
            }

            valid_urls
        } else {
            vec![format!("{}/collections/lofi", BASE_URL)]
        };

        join!(self.fetch_content(urls_to_scrape));
    }

    async fn sleeper(time_in_secs: u64) {
        sleep(Duration::from_secs(time_in_secs)).await;
    }

    async fn fetch_content(&self, urls: Vec<String>) {
        let mut all_album_data = Vec::new();

        for url in urls {
            let mut retry_attempts = 0;

            while retry_attempts < MAX_RETRIES {
                info!(
                    "Fetching data from URL: {} (Attempt {}/{})",
                    url,
                    retry_attempts + 1,
                    MAX_RETRIES
                );

                match self.fetch_and_process_data(&url).await {
                    Ok(album_data) => {
                        all_album_data.extend(album_data);
                        break;
                    }
                    Err(err) => {
                        error!("Error fetching data: {}. \nRetrying...", err);
                        retry_attempts += 1;
                        Self::sleeper(15).await;
                    }
                }
            }

            if retry_attempts >= MAX_RETRIES {
                error!("Maximum retry attempts reached for URL: {}. Aborting.", url);
            }
        }

        if let Ok(mut writer) = Writer::from_path("output.csv") {
            for album_data in all_album_data {
                if let Err(err) = writer.write_record(&album_data) {
                    eprintln!("Error writing to CSV: {}", err);
                }
            }
        } else {
            eprintln!("Error opening or creating CSV file");
        }
    }

    async fn fetch_and_process_data(&self, url: &str) -> Result<Vec<Vec<String>>, reqwest::Error> {
        let response = self.client.get(url).send().await?;
    
        if response.status() == StatusCode::SERVICE_UNAVAILABLE {
            error!("Service unavailable for URL: {}. Retrying...", url);
            Self::sleeper(15).await;
            return Ok(Vec::new());
        }
    
        let body = response.text().await?;
        Ok(self.process_data(&body, url).await)
    }

    async fn process_data(&self, body: &str, url: &str) -> Vec<Vec<String>> {
        let document = Html::parse_document(body);
        let release_selector = Selector::parse("a.album-title").unwrap();
        // 😭
        let mut releases = document.select(&release_selector).peekable();

        let releases_count = releases.peek().is_some();
    
        let mut all_album_data = Vec::new();
    
        if !releases_count {
            info!("No albums found on the page: {}", url);
        } else {
            for release in releases {
                let album_name = release.text().collect::<String>();
                let album_url =
                    format!("{}{}", BASE_URL, release.value().attr("href").unwrap_or(""));
        
                let tracks = self
                    .fetch_data_by_selector(&album_url, "span.track-name")
                    .await;
        
                let cleaned_tracks: Vec<String> = tracks
                    .iter()
                    .map(|track| track.trim().replace("\n", ""))
                    .collect();
        
                all_album_data.push(vec![album_name, album_url, cleaned_tracks.join(", ")]);
            }
        
            info!("Successfully fetched data from URL: {}", url);
        }
    
        all_album_data
    }

    async fn fetch_data_by_selector(&self, url: &str, selector: &str) -> Vec<String> {
        let response = self.client.get(url).send().await;

        match response {
            Ok(response) => {
                if response.status() == StatusCode::SERVICE_UNAVAILABLE {
                    error!(
                        "Service unavailable while fetching tracks for URL: {}. Retrying...",
                        url
                    );
                    Self::sleeper(15).await;
                    return Vec::new();
                }

                let body = response.text().await.unwrap();
                let document = Html::parse_document(&body);
                let track_selector = Selector::parse(selector).unwrap();
                let tracks = document.select(&track_selector);

                let mut track_names = Vec::new();
                for track in tracks {
                    track_names.push(track.text().collect::<String>());
                }

                track_names
            }
            Err(_) => Vec::new(),
        }
    }
}
