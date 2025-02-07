use reqwest;

#[tauri::command]
pub async fn get_sports_news(topic: String) -> Result<Vec<String>, String> {
    let url = format!("https://www.nikkansports.com/{}/atom.xml", topic);
    let response = reqwest::get(url).await.unwrap();
    let response_str = response.text().await.unwrap();

    let article_titles: Vec<String> = response_str
        .split("<title>")
        .skip(1)
        .map(|s| s.split("</title>").next().unwrap().to_string())
        .collect();
      
    Ok(article_titles)
}