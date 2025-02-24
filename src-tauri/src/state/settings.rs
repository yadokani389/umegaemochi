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
    pub nightmode_range: NightmodeRange,
    pub use_sound_when_disaster: bool,
    pub server_port: u16,
}

#[derive(Debug, Copy, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NightmodeRange {
    pub start: chrono::NaiveTime,
    pub end: chrono::NaiveTime,
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
            nightmode_range: NightmodeRange {
                start: chrono::NaiveTime::from_hms_opt(22, 0, 0).unwrap(),
                end: chrono::NaiveTime::from_hms_opt(6, 30, 0).unwrap(),
            },
            use_sound_when_disaster: false,
            server_port: 33117,
        }
    }
}
