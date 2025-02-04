mod router;

pub async fn start_server(handle: tauri::AppHandle) {
    let port_number = 33117;

    let app = router::new(handle);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port_number}"))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
