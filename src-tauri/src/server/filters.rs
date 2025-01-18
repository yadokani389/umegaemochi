use crate::commands::settings::{set_atcoder_id, set_weather_city_id};
use crate::disaster_info::DisasterInfo;
use crate::settings::Settings;
use crate::state::AppState;
use std::convert::Infallible;
use std::sync::Mutex;
use tauri::{Emitter, Manager};
use warp::{http::StatusCode, Filter};

pub fn api(
    handle: tauri::AppHandle,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_settings(handle.clone())
        .or(post_settings(handle.clone()))
        .or(get_disaster_info(handle.clone()))
        .or(post_disaster_info(handle.clone()))
        .or(scroll(handle.clone()))
}

fn get_settings(
    handle: tauri::AppHandle,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    println!(
        "{:?}",
        handle.state::<Mutex<AppState>>().lock().unwrap().settings
    );

    warp::path!("settings").and(warp::get()).and_then(move || {
        let handle = handle.clone();
        async move {
            Ok::<warp::reply::Json, Infallible>(warp::reply::json(
                &handle
                    .state::<Mutex<AppState>>()
                    .lock()
                    .unwrap()
                    .settings
                    .data
                    .clone(),
            ))
        }
    })
}

fn post_settings(
    handle: tauri::AppHandle,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("settings")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |new_settings: Settings| {
            let handle = handle.clone();
            async move {
                set_weather_city_id(
                    handle.clone(),
                    handle.clone().state(),
                    new_settings.weather_city_id.clone(),
                )
                .unwrap();
                set_atcoder_id(
                    handle.clone(),
                    handle.clone().state(),
                    new_settings.atcoder_id.clone(),
                )
                .unwrap();
                Ok::<warp::reply::Json, Infallible>(warp::reply::json(&new_settings.clone()))
            }
        })
}

fn get_disaster_info(
    handle: tauri::AppHandle,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("disaster_info")
        .and(warp::get())
        .and_then(move || {
            let handle = handle.clone();
            async move {
                Ok::<warp::reply::Json, Infallible>(warp::reply::json(
                    &handle
                        .state::<Mutex<AppState>>()
                        .lock()
                        .unwrap()
                        .disaster_info
                        .clone(),
                ))
            }
        })
}

fn post_disaster_info(
    handle: tauri::AppHandle,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("disaster_info")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |new_disaster_info: DisasterInfo| {
            let handle = handle.clone();
            async move {
                handle
                    .state::<Mutex<AppState>>()
                    .lock()
                    .unwrap()
                    .disaster_info = Some(new_disaster_info.clone());
                println!("{:?}", new_disaster_info.clone());
                let _ = handle.emit("disaster_occurred", new_disaster_info.clone());

                Ok::<warp::reply::Json, Infallible>(warp::reply::json(&new_disaster_info.clone()))
            }
        })
}

fn scroll(
    handle: tauri::AppHandle,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("scroll" / String).and_then(move |name: String| {
        let handle = handle.clone();
        async move {
            let valid_names = [
                "WidgetWeather",
                "WidgetNews",
                "WidgetAtCoder",
                "WidgetCalendar",
                "WidgetClock",
                "prev",
                "next",
            ];
            if valid_names.contains(&name.as_str()) {
                let _ = handle.emit("scroll", &name);
                println!("scroll: {}", name);
                Ok::<warp::http::StatusCode, Infallible>(StatusCode::OK)
            } else {
                println!("scroll: invalid name");
                Ok(StatusCode::BAD_REQUEST)
            }
        }
    })
}
