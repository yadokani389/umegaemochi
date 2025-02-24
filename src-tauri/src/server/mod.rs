use tauri::Manager;

mod router;

pub async fn start_server(handle: tauri::AppHandle) {
    let port_number = handle
        .state::<std::sync::Mutex<crate::state::AppState>>()
        .lock()
        .unwrap()
        .settings
        .data
        .server_port;

    let app = router::new(handle);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port_number}"))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
