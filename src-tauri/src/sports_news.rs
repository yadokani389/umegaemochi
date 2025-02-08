use reqwest;
use crate::commands::utils::stringify;

#[tauri::command]
pub async fn get_sports_news(topic: String) -> Result<Vec<String>, String> {
    nestify::nest! {
        #[derive(Debug, serde::Deserialize)]*
        #[serde(rename = "feed")]
        struct Feed {
            #[serde(rename = "entry")]
            entries: Vec<struct Entry {
                title: String,
            }>,
        }
    }

    let url = format!("https://www.nikkansports.com/{}/atom.xml", topic);

    let resp = reqwest::get(url)
        .await
        .map_err(stringify)?
        .text()
        .await
        .map_err(stringify)?;

    let feed: Feed = quick_xml::de::from_str(&resp).map_err(stringify)?;
    let titles = feed.entries.into_iter().map(|entry| entry.title).collect();

    Ok(titles)
}
