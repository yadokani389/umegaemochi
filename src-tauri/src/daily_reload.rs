use chrono::{Duration, Local, NaiveTime};
use tauri::Emitter;
use tokio::time::{sleep_until, Instant};

pub async fn start_job(handle: tauri::AppHandle) {
    loop {
        let now = Local::now();
        let target_time = {
            let mut res = Local::now()
                .with_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
                .unwrap();
            while res < now {
                res += Duration::hours(6);
            }
            res
        };
        let sleep_duration = target_time - now;

        println!("Now: {}", now);
        println!("Next run: {}", target_time);
        println!("Sleeping for {} seconds", sleep_duration.num_seconds());

        sleep_until(Instant::now() + sleep_duration.to_std().unwrap()).await;

        let _ = handle.emit("daily_reload", ());
    }
}
