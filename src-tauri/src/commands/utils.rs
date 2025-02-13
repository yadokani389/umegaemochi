#[tauri::command]
pub async fn get_yahoo_news(url: String) -> Result<Vec<String>, String> {
    nestify::nest! {
        #[derive(Debug, serde::Deserialize)]*
        struct Rss {
            channel: struct Channel {
                #[serde(rename = "item")]
                items: Vec<struct Item {
                    title: String,
                }>,
            },
        }
    }

    let resp = reqwest::get(url)
        .await
        .map_err(stringify)?
        .text()
        .await
        .map_err(stringify)?;

    let rss: Rss = quick_xml::de::from_str(&resp).map_err(stringify)?;
    let titles = rss
        .channel
        .items
        .into_iter()
        .map(|item| item.title)
        .collect();

    Ok(titles)
}

#[tauri::command]
pub fn get_server_address() -> Result<String, String> {
    let port_number = 33117;

    let local_addr = format!(
        "{}:{}",
        local_ip_addr::get_local_ip_address().map_err(stringify)?,
        port_number
    );

    println!("Listening on {}", local_addr);

    Ok(local_addr)
}

pub fn stringify(e: impl ToString) -> String {
    e.to_string()
}

#[tauri::command]
pub fn get_settings(
    state: tauri::State<std::sync::Mutex<crate::state::AppState>>,
) -> Result<crate::state::settings::Settings, String> {
    Ok(state.lock().unwrap().settings.data.clone())
}

#[tauri::command]
pub fn get_version() -> String {
    crate::VERSION.to_string()
}

#[tauri::command]
pub fn get_todos(
    state: tauri::State<std::sync::Mutex<crate::state::AppState>>,
) -> Result<Vec<crate::state::todo::Todo>, String> {
    Ok(state.lock().unwrap().todo.data.values().cloned().collect())
}

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
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ExchangeRate {
    pub result: String,
    pub basecode: String,
    pub update: String,
    pub source: String,
    #[serde(rename = "API_URL")]
    pub api_url: String,
    #[serde(flatten)]
    pub rates: std::collections::HashMap<String, String>,
    #[serde(rename = "JPY")]
    pub jpy: f32,
}

#[tauri::command]
pub async fn get_exchange_rate() -> Result<std::collections::HashMap<String, f32>, String> {
    let url = "https://api.aoikujira.com/kawase/json/jpy";

    let response = reqwest::get(url)
        .await
        .map_err(stringify)?
        .json::<ExchangeRate>()
        .await
        .map_err(stringify)?;

    let mut rates = std::collections::HashMap::new();

    for (key, value) in response.rates {
        if let Ok(rate) = value.parse::<f32>() {
            rates.insert(key, rate);
        }
    }
    Ok(rates)
}
