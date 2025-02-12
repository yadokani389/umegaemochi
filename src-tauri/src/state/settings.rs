#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Settings {
    pub weather_city_id: String,
    pub atcoder_id: String,
    pub widget_interval: u64,
    pub using_widgets: Vec<String>,
    pub auto_fullscreen: bool,
    pub auto_hide_cursor: bool,
    pub using_sports_news: Vec<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            weather_city_id: "130010".into(),
            atcoder_id: "1step621".into(),
            widget_interval: 10000,
            using_widgets: crate::WIDGET_LIST.iter().map(|x| x.to_string()).collect(),
            auto_fullscreen: false,
            auto_hide_cursor: false,
            using_sports_news: Vec::new(),
        }
    }
}
