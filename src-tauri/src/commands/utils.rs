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
        localip::get_local_ip().map_err(stringify)?,
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
) -> Result<crate::settings::Settings, String> {
    Ok(state.lock().unwrap().settings.data.clone())
}

#[tauri::command]
pub fn get_version() -> String {
    crate::VERSION.to_string()
}
