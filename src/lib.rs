use plugin::default::DefaultScraper;
use scraper::Html;
use url::Url;

mod plugin;

#[derive(Debug)]
pub struct SemanticURL {
    pub title: Option<String>,
    pub description: Option<String>,
    pub image_url: Option<Url>,
    pub url: Url,
}

/// Trait implemented by all scrapers
trait Scraper {
    fn scrape(url: Url, document: Html) -> Option<SemanticURL>;
}

pub fn scrape_webpage(url: Url, body: String) -> SemanticURL {
    let document = Html::parse_document(&body);

    DefaultScraper::scrape(url, document).unwrap()
    /*
    copied from go implememtation

    plugins.Amazon(&returnee, sourceURL, doc, c.Log, c.AmazonAdID)
    plugins.Imgurl(&returnee, sourceURL, c.HTTPClient, c.Log, c.ImgurClientID)
    plugins.Gfycat(&returnee, sourceURL, doc, c.Log)

    plugins.Fefe(&returnee, sourceURL, doc, c.Log)

    plugins.Youtube(&returnee, sourceURL, doc, c.Log)
    plugins.Vimeo(&returnee, sourceURL, doc, c.Log)

    plugins.Dilbert(&returnee, sourceURL, doc, c.Log)
    plugins.Garfield(&returnee, sourceURL, doc, c.Log)
    plugins.Xkcd(&returnee, sourceURL, doc, c.Log)
    plugins.Littlegamers(&returnee, sourceURL, doc, c.Log)

    plugins.IEEExplore(&returnee, sourceURL, doc, c.Log)

    plugins.Pastebin(&returnee, sourceURL, doc, c.Log)
     */
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn default_html() {
        let body = fs::read_to_string("tests/default_html.html").expect("reading file failed");
        let url = Url::parse("https://foo.bar/webpage").unwrap();
        let semantic = scrape_webpage(url.clone(), body);

        assert_eq!(semantic.url, url);
        assert_eq!(semantic.title.unwrap(), "OG Title");
        assert_eq!(semantic.description.unwrap(), "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Auctor eu augue ut lectus arcu bibendum. Rhoncus mattis rhoncus urna neque viverra justo nec ultrices dui. Vestibulum morbi blandit cursus risus at ultrices mi.");
        assert_eq!(
            semantic.image_url.unwrap().as_str(),
            "https://foo.bar/image.png"
        );
    }

    #[test]
    fn no_og_title() {
        let body = fs::read_to_string("tests/no_og_title.html").expect("reading file failed");
        let url = Url::parse("https://foo.bar").unwrap();
        let semantic = scrape_webpage(url.clone(), body);

        assert_eq!(semantic.url, url);
        assert_eq!(semantic.title.unwrap(), "HTML Title");
        assert_eq!(semantic.description, None);
        assert_eq!(semantic.image_url, None);
    }

    #[test]
    fn empty() {
        let body = fs::read_to_string("tests/empty.html").expect("reading file failed");
        let url = Url::parse("https://foo.bar").unwrap();
        let semantic = scrape_webpage(url.clone(), body);

        assert_eq!(semantic.url, url);
        assert_eq!(semantic.title, None);
        assert_eq!(semantic.description, None);
        assert_eq!(semantic.image_url, None);
    }
}
