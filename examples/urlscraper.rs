use clap::Parser;
use semantic_scraper::scrape_webpage;
use url::Url;

/// Program to extract data from an URL
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Url to download and parse
    #[arg(short, long)]
    url: Url,
}

fn main() {
    let args = Args::parse();

    let body = reqwest::blocking::get(args.url.clone())
        .unwrap()
        .text()
        .unwrap();

    let webpage_infos = scrape_webpage(args.url, body);

    println!("{:#?}", webpage_infos);
}
