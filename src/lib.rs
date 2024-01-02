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
