use reqwest::Client;
use scraper::{Html, Selector};

use crate::error::{ErrorKind, Errors};

#[derive(Debug)]
pub struct SiteInfo<'a> {
    name: &'a str,
    address: &'a str,
    css_selector: &'a str,
}

impl<'a> SiteInfo<'a> {
    pub fn new(name: &'a str, address: &'a str, css_selector: &'a str) -> SiteInfo<'a> {
        SiteInfo {
            name,
            address,
            css_selector,
        }
    }

    async fn parse_selector(&self) -> Result<Selector, Errors<'a>> {
        Selector::parse(&self.css_selector)
            .map_err(|_| Errors::new(ErrorKind::ParsingError, self.css_selector))
    }

    // pub fn name(&self) -> &'a str {
    //     self.name
    // }

    // pub fn address(&self) -> &'a str {
    //     self.address
    // }

    // pub fn keyword(&self) -> &'a str {
    //     self.keyword
    // }
}

pub struct Handler<'a> {
    product_name: &'a str,
    client: Client,
    links: Vec<SiteInfo<'a>>,
    keyword: &'a str,
}

impl<'a> Handler<'a> {
    pub async fn new(
        product_name: &'a str,
        links: Vec<SiteInfo<'a>>,
        keyword: &'a str,
    ) -> Handler<'a> {
        Handler {
            product_name,
            links,
            keyword,
            client: Client::new(),
        }
    }

    pub async fn scrape(&self) -> Result<(), Errors<'a>> {
        self.parse_html().await?;
        Ok(())
    }

    async fn get_html(&self, link: &'a str) -> Result<Html, Errors<'a>> {
        match self.client.get(link).send().await {
            Ok(res) => {
                let html_unparsed = res
                    .text()
                    .await
                    .map_err(|_| Errors::new(ErrorKind::HtmlError, "Cannot get Html String"))?;
                Ok(Html::parse_fragment(&html_unparsed))
            }
            Err(_) => Err(Errors::new(ErrorKind::HtmlError, "Cannot Process Request")),
        }
    }

    async fn parse_html(&self) -> Result<(), Errors<'a>> {
        for link in self.links.iter() {
            let selector = link.parse_selector().await?;
            let html = self.get_html(link.address).await?;
            let mut result = vec![];
            for elem in html.select(&selector) {
                let res = elem.text().collect::<Vec<_>>();
                result.push(res.concat());
            }

            if result.concat().trim().trim_end_matches(".").to_string() == (self.keyword) {
                println!(
                    "[{}]{:^20}{:^20}",
                    link.name, self.product_name, "Out of Stock"
                )
            } else {
                println!("[{}]{:^20}{:^20}", link.name, self.product_name, "In Stock")
            }
        }
        Ok(())
    }
}
