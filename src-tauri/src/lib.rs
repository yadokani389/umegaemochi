// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn get_yahoo_news(url: String) -> Result<Vec<String>, String> {
    let client = reqwest::Client::new();
    let Ok(response) = client.get(&url).send().await else {
        return Err("Failed to fetch the URL".to_string());
    };

    let Ok(text) = response.text().await else {
        return Err("Failed to read the response".to_string());
    };
    let mut reader = quick_xml::Reader::from_str(&text);

    let target_elm_name = String::from("title");
    let mut output_flg = false;
    let mut buf = Vec::new();
    let mut ret = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(quick_xml::events::Event::Eof) => break,
            Ok(quick_xml::events::Event::Start(e)) => {
                let elm_name = String::from_utf8(e.name().as_ref().to_vec()).unwrap();
                if elm_name == target_elm_name {
                    output_flg = true
                }
            }
            Ok(quick_xml::events::Event::Text(e)) => {
                if output_flg {
                    ret.push(e.unescape().unwrap().into_owned());
                    output_flg = false
                }
            }
            _ => (),
        }
        buf.clear();
    }

    Ok(ret)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_yahoo_news])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
