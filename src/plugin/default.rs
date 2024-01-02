use itertools::Itertools;
use scraper::Html;
use url::Url;

use crate::{Scraper, SemanticURL};

pub(crate) struct DefaultScraper {}

impl Scraper for DefaultScraper {
    fn scrape(url: Url, document: Html) -> Option<SemanticURL> {
        let queries = [
            "meta[property='og:title'",
            "meta[property='og:description'",
            "meta[property='og:image'",
            "meta[property='og:url'",
        ];

        let (og_title, description, image_url, parsed_url) = queries
            .iter()
            .map(|q| {
                let selector = scraper::Selector::parse(q).unwrap();
                if let Some(v) = document.select(&selector).next() {
                    v.attr("content")
                } else {
                    None
                }
            })
            .next_tuple()
            .unwrap();

        // get <title> if og:title is not set
        let html_title = if og_title.is_none() {
            let selector = scraper::Selector::parse("title").unwrap();
            document
                .select(&selector)
                .next()
                .map(|temp| temp.inner_html())
        } else {
            None
        };

        // select og title if available
        let title = if og_title.is_some() {
            og_title.map(str::to_string)
        } else {
            html_title
        };

        // parse urls
        let image_url = if let Some(url) = image_url {
            Url::parse(url).ok()
        } else {
            None
        };

        let parsed_url = if let Some(url) = parsed_url {
            Url::parse(url).ok()
        } else {
            Some(url)
        };

        Some(SemanticURL {
            title,
            description: description.map(str::to_string),
            image_url,
            url: parsed_url.unwrap(),
        })
    }
}
