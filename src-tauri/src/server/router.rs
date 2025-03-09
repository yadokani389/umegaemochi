use crate::state::{
    config::ConfigTrait, disaster_info::DisasterInfo, settings::Settings, todo::Todo, AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
};
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use tauri::{Emitter, Manager};
use uuid::Uuid;

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
        .route("/hostname", get(get_hostname))
        .route("/todos", get(get_todos).post(create_todo))
        .route(
            "/todos/{id}",
            get(get_todo).patch(update_todo).delete(delete_todo),
        )
        .route("/sports_news", get(get_sports_news))
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
    const VALID_NAMES: &[&str; crate::WIDGET_LIST.len() + 2] =
        constcat::concat_slices!([&str]: &crate::WIDGET_LIST, &["prev", "next"]);

    if VALID_NAMES.contains(&name.as_str()) {
        handle.emit("scroll", &name).unwrap();
        println!("scroll: {name}");
        StatusCode::OK
    } else {
        println!("scroll: invalid name");
        StatusCode::BAD_REQUEST
    }
}

async fn get_widgets() -> impl IntoResponse {
    Json(crate::WIDGET_LIST)
}

async fn get_sports_news() -> impl IntoResponse {
    Json([
        "プロ野球",
        "高校野球",
        "サッカー",
        "スポーツ",
        "ゴルフ",
        "ラグビー",
        "テニス",
        "バスケ",
        "バレー",
        "水泳",
    ])
}

async fn get_version() -> impl IntoResponse {
    crate::VERSION
}

async fn get_hostname() -> impl IntoResponse {
    tauri_plugin_os::hostname()
}

async fn get_todos(State(handle): State<Arc<tauri::AppHandle>>) -> impl IntoResponse {
    Json(
        handle
            .state::<Mutex<AppState>>()
            .lock()
            .unwrap()
            .todo
            .data
            .clone(),
    )
}

async fn get_todo(
    State(handle): State<Arc<tauri::AppHandle>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Todo>, StatusCode> {
    let todo = handle
        .state::<Mutex<AppState>>()
        .lock()
        .unwrap()
        .todo
        .data
        .get(&id)
        .cloned();

    match todo {
        Some(todo) => Ok(Json(todo)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

#[derive(Debug, Deserialize)]
struct CreateTodo {
    text: String,
}

async fn create_todo(
    State(handle): State<Arc<tauri::AppHandle>>,
    Json(input): Json<CreateTodo>,
) -> impl IntoResponse {
    let todo = Todo {
        id: Uuid::new_v4(),
        text: input.text,
        completed: false,
    };

    let state = handle.state::<Mutex<AppState>>();
    let mut state = state.lock().unwrap();
    state.todo.data.insert(todo.id, todo.clone());
    state.todo.write_file().unwrap();

    handle.emit("todo_changed", ()).unwrap();

    (StatusCode::CREATED, Json(todo))
}

#[derive(Debug, Deserialize)]
struct UpdateTodo {
    text: Option<String>,
    completed: Option<bool>,
}

async fn update_todo(
    State(handle): State<Arc<tauri::AppHandle>>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateTodo>,
) -> Result<Json<Todo>, StatusCode> {
    let state = handle.state::<Mutex<AppState>>();
    let todo = {
        let mut state = state.lock().unwrap();

        let todo = state.todo.data.get_mut(&id).ok_or(StatusCode::NOT_FOUND)?;
        todo.text = input.text.unwrap_or_else(|| todo.text.clone());
        todo.completed = input.completed.unwrap_or(todo.completed);
        todo.clone()
    };
    state.lock().unwrap().todo.write_file().unwrap();

    handle.emit("todo_changed", ()).unwrap();

    Ok(Json(todo))
}

async fn delete_todo(
    State(handle): State<Arc<tauri::AppHandle>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Todo>, StatusCode> {
    let state = handle.state::<Mutex<AppState>>();
    let mut state = state.lock().unwrap();

    let todo = state.todo.data.remove(&id).ok_or(StatusCode::NOT_FOUND)?;
    state.todo.write_file().unwrap();

    handle.emit("todo_changed", ()).unwrap();

    Ok(Json(todo))
}
