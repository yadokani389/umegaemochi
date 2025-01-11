use crate::commands::utils::stringify;
use chrono::{DateTime, Local};
use std::sync::Mutex;
use tauri::{Emitter, Manager};
use unicode_normalization::UnicodeNormalization;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DisasterInfo {
    title: String,
    description: String,
    warning: String,
    occurred: DateTime<Local>,
}

#[derive(Debug, serde::Deserialize, PartialEq, Eq, Hash)]
struct Entry {
    title: String,
    id: String,
    updated: String,
    content: String,
}

#[derive(Debug, serde::Deserialize, PartialEq)]
struct Feed {
    updated: String,
    #[serde(rename = "entry", default)]
    entries: std::collections::HashSet<Entry>,
}

nestify::nest! {
    #[derive(Debug, serde::Deserialize)]*
    #[serde(rename_all = "PascalCase")]*
    struct Report {
        head: struct Head {
            headline: struct Headline {
                text: String,
            },
        },
        body: struct Body {
            earthquake: struct Earthquake {
                origin_time: DateTime<Local>,
                hypocenter: struct Hypocenter {
                    area: struct Area {
                        name: String,
                        coordinate: struct Coordinate {
                            #[serde(rename = "@description")]
                            description: String,
                        },
                    },
                },
                magnitude: String,
            },
            intensity: struct Intensity {
                observation: struct Observation {
                    max_int: u8,
                },
            },
            comments: struct Comment {
                forecast_comment: struct ForecastComment {
                    text: String,
                },
            }
        }
    }
}

pub async fn check_disaster_updates(handle: tauri::AppHandle) -> Result<(), String> {
    loop {
        let feed = fetch_feed().await.map_err(stringify)?;
        for entry in feed.entries.iter() {
            if entry.title.contains("震源") {
                let report = fetch_report(&entry.id).await.map_err(stringify)?;
                println!("{:?}", report);
                update_state_if_needed(&handle, report)
                    .await
                    .map_err(stringify)?;
            } 
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}

async fn fetch_feed() -> Result<Feed, Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://www.data.jma.go.jp/developer/xml/feed/eqvol.xml")
        .await?
        .text()
        .await?;
    let feed: Feed = quick_xml::de::from_str(&resp)?;
    Ok(feed)
}

async fn fetch_report(url: &str) -> Result<Report, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url).await?.text().await?;
    let report: Report = quick_xml::de::from_str(&resp)?;
    Ok(report)
}

async fn update_state_if_needed(
    handle: &tauri::AppHandle,
    report: Report,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(mut state) = handle.state::<Mutex<crate::state::AppState>>().lock() {
        if let Some(disaster_info) = state.disaster_info.as_ref() {
            if disaster_info.occurred == report.body.earthquake.origin_time {
                return Ok(());
            }
        }

        state.disaster_info = Some(DisasterInfo {
            title: format!(
                "{}で震度{},マグニチュード{}の地震が発生しました。",
                report.body.earthquake.hypocenter.area.name,
                report.body.intensity.observation.max_int,
                report.body.earthquake.magnitude
            ),
            description: format!(
                "{}で{}",
                report
                    .body
                    .earthquake
                    .hypocenter
                    .area
                    .coordinate
                    .description
                    .nfkc()
                    .collect::<String>(),
                report.head.headline.text.nfkc().collect::<String>()
            ),
            warning: report.body.comments.forecast_comment.text,
            occurred: report.body.earthquake.origin_time,
        });
        println!("{:?}", state.disaster_info);
        handle.emit(
            "disaster_occurred",
            state.disaster_info.as_ref().unwrap().clone(),
        )?;
    }
    Ok(())
}
