use std::sync::Arc;

use handler::SiteInfo;

mod error;
mod handler;
mod mailer;

#[tokio::main]
async fn main() {
    let nv_3070_card = handler::Handler::new(
        "3070",
        vec![SiteInfo::new(
            "Rp-Tech",
            "https://rptechindia.in/nvidia-geforce-rtx-3070.html",
            "strike",
        )],
        "Out of stock",
    )
    .await;
    let nv_3060_ti_card = handler::Handler::new(
        "3060 ti",
        vec![SiteInfo::new(
            "Rp-Tech",
            "https://rptechindia.in/nvidia-geforce-rtx-3060-ti.html",
            "strike",
        )],
        "Out of stock",
    )
    .await;
    let cards = vec![Arc::new(nv_3060_ti_card), Arc::new(nv_3070_card)];
    let mut workers = vec![];
    for card in cards {
        let worker = tokio::spawn(async move {
            match card.scrape().await {
                Err(e) => println!("{}", e),
                _ => (),
            };
        });
        workers.push(worker)
    }
    for worker in workers {
        match worker.await {
            Err(e) => println!("{}", e),
            _ => (),
        };
    }
}
