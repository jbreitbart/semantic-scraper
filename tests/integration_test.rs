use std::fs;

use semantic_scraper::scrape_webpage;
use url::Url;

#[test]
fn default_html() {
    let body = fs::read_to_string("tests/input/default_html.html").expect("reading file failed");
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
    let body = fs::read_to_string("tests/input/no_og_title.html").expect("reading file failed");
    let url = Url::parse("https://foo.bar").unwrap();
    let semantic = scrape_webpage(url.clone(), body);

    assert_eq!(semantic.url, url);
    assert_eq!(semantic.title.unwrap(), "HTML Title");
    assert_eq!(semantic.description, None);
    assert_eq!(semantic.image_url, None);
}

#[test]
fn empty() {
    let body = fs::read_to_string("tests/input/empty.html").expect("reading file failed");
    let url = Url::parse("https://foo.bar").unwrap();
    let semantic = scrape_webpage(url.clone(), body);

    assert_eq!(semantic.url, url);
    assert_eq!(semantic.title, None);
    assert_eq!(semantic.description, None);
    assert_eq!(semantic.image_url, None);
}
