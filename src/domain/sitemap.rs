use sitemap::reader::{SiteMapEntity, SiteMapReader};
use std::fs::File;

pub fn get_sitemap() {
    let mut urls = Vec::new();
    let mut sitemaps = Vec::new();
    let mut errors = Vec::new();
    let file = File::open("sitemap.xml").expect("Unable to open file.");
    let parser = SiteMapReader::new(file);
    for entity in parser {
        match entity {
            SiteMapEntity::Url(url_entry) => {
                urls.push(url_entry);
            }
            SiteMapEntity::SiteMap(sitemap_entry) => {
                sitemaps.push(sitemap_entry);
            }
            SiteMapEntity::Err(error) => {
                errors.push(error);
            }
        }
    }
    println!("urls = {:?}", urls);
    println!("sitemaps = {:?}", sitemaps);
    println!("errors = {:?}", errors);
}
