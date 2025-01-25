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
        .or(clear_disaster_info(handle.clone()))
        .or(scroll(handle.clone()))
        .or(get_widgets())
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
            println!("{:?}", new_settings.clone());
            async move {
                handle
                    .clone()
                    .state::<Mutex<AppState>>()
                    .lock()
                    .unwrap()
                    .settings
                    .set(new_settings.clone())
                    .unwrap();
                handle.emit("settings_changed", ()).unwrap();
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
                handle
                    .emit("disaster_occurred", new_disaster_info.clone())
                    .unwrap();

                Ok::<warp::reply::Json, Infallible>(warp::reply::json(&new_disaster_info.clone()))
            }
        })
}

fn clear_disaster_info(
    handle: tauri::AppHandle,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("disaster_info" / "clear").and_then(move || {
        let handle = handle.clone();
        async move {
            handle.emit("disaster_clear", ()).unwrap();
            println!("clear disaster info");
            Ok::<warp::http::StatusCode, Infallible>(StatusCode::OK)
        }
    })
}

fn scroll(
    handle: tauri::AppHandle,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("scroll" / String).and_then(move |name: String| {
        let handle = handle.clone();
        async move {
            const VALID_NAMES: &[&str; 7] =
                constcat::concat_slices!([&str]: &crate::WIDGET_LIST, &["prev", "next"]);
            if VALID_NAMES.contains(&name.as_str()) {
                handle.emit("scroll", &name).unwrap();
                println!("scroll: {}", name);
                Ok::<warp::http::StatusCode, Infallible>(StatusCode::OK)
            } else {
                println!("scroll: invalid name");
                Ok(StatusCode::BAD_REQUEST)
            }
        }
    })
}

fn get_widgets() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("widgets").and(warp::get()).and_then(|| async {
        Ok::<warp::reply::Json, Infallible>(warp::reply::json(&crate::WIDGET_LIST))
    })
}
