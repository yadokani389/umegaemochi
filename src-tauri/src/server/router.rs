use crate::disaster_info::DisasterInfo;
use crate::settings::Settings;
use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
};
use std::sync::{Arc, Mutex};
use tauri::{Emitter, Manager};

pub fn new(handle: tauri::AppHandle) -> axum::Router {
    let handle = Arc::new(handle);
    axum::Router::new()
        .route("/settings", get(get_settings).post(post_settings))
        .route(
            "/disaster_info",
            get(get_disaster_info).post(post_disaster_info),
        )
        .route("/disaster_info/clear", get(clear_disaster_info))
        .route("/scroll/{name}", get(scroll))
        .route("/widgets", get(get_widgets))
        .route("/version", get(get_version))
        .with_state(handle)
}

async fn get_settings(State(handle): State<Arc<tauri::AppHandle>>) -> impl IntoResponse {
    Json(
        handle
            .state::<Mutex<AppState>>()
            .lock()
            .unwrap()
            .settings
            .data
            .clone(),
    )
}

async fn post_settings(
    State(handle): State<Arc<tauri::AppHandle>>,
    Json(new_settings): Json<Settings>,
) {
    handle
        .state::<Mutex<AppState>>()
        .lock()
        .unwrap()
        .settings
        .set(new_settings.clone())
        .unwrap();

    println!("{:?}", new_settings.clone());

    handle.emit("settings_changed", ()).unwrap();
}

async fn get_disaster_info(State(handle): State<Arc<tauri::AppHandle>>) -> impl IntoResponse {
    Json(
        handle
            .state::<Mutex<AppState>>()
            .lock()
            .unwrap()
            .disaster_info
            .clone(),
    )
}

async fn post_disaster_info(
    State(handle): State<Arc<tauri::AppHandle>>,
    Json(new_disaster_info): Json<DisasterInfo>,
) {
    handle
        .state::<Mutex<AppState>>()
        .lock()
        .unwrap()
        .disaster_info = Some(new_disaster_info.clone());

    println!("{:?}", new_disaster_info.clone());

    handle
        .emit("disaster_occurred", new_disaster_info.clone())
        .unwrap();
}

async fn clear_disaster_info(State(handle): State<Arc<tauri::AppHandle>>) {
    handle.emit("disaster_clear", ()).unwrap();
    println!("clear disaster info");
}

async fn scroll(
    State(handle): State<Arc<tauri::AppHandle>>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    const VALID_NAMES: &[&str; 7] =
        constcat::concat_slices!([&str]: &crate::WIDGET_LIST, &["prev", "next"]);

    if VALID_NAMES.contains(&name.as_str()) {
        handle.emit("scroll", &name).unwrap();
        println!("scroll: {}", name);
        StatusCode::OK
    } else {
        println!("scroll: invalid name");
        StatusCode::BAD_REQUEST
    }
}

async fn get_widgets() -> impl IntoResponse {
    Json(crate::WIDGET_LIST)
}

async fn get_version() -> impl IntoResponse {
    Json(crate::VERSION)
}
