// とりあえずrssを読み込んでみよう
pub async fn get_sports_news(topic: String) -> Vec<String> {
    let url = "https://www.nikkansports.com/" + &topic + "/atom.xml";
    let response = reqwest::get(url).await.unwrap();
    let response_str = response.text().await.unwrap();

    // <feed> <entry> <title>を全て取得したい

    let article_titles: Vec<String> = response_str
        .split("<title>")
        .skip(1)
        .map(|s| s.split("</title>").next().unwrap().to_string())
        .collect();
    Ok(article_titles)
}

// テスト用の関数

// #[tokio::main]
// async fn main() {
//     let titles = get_sports_news().await;
//     for title in titles.iter() {
//         println!("{}", title);
//     }
// }
