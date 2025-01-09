use crate::commands::settings::{set_atcoder_id, set_weather_city_id};
use crate::settings::Settings;
use crate::state::AppState;
use std::convert::Infallible;
use std::sync::Mutex;
use tauri::Manager;
use warp::Filter;

pub fn api(
    handle: tauri::AppHandle,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_settings(handle.clone()).or(post_settings(handle))
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
