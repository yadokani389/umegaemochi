mod filters;

pub async fn start_server(handle: tauri::AppHandle) {
    let port_number = 33117;

    let api = filters::api(handle);

    warp::serve(api).run(([0, 0, 0, 0], port_number)).await;
}
