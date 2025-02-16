use chrono::{Duration, Local, NaiveTime};
use tauri::{Emitter, Manager};

pub async fn start_daily_reload(handle: tauri::AppHandle) {
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

        tokio::time::sleep(sleep_duration.to_std().unwrap()).await;

        handle.emit("daily_reload", ()).unwrap();
    }
}

pub async fn start_control_nightmode(handle: tauri::AppHandle) {
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    loop {
        let nightmode_range = handle
            .state::<std::sync::Mutex<crate::state::AppState>>()
            .lock()
            .unwrap()
            .settings
            .data
            .nightmode_range;
        let now = Local::now();
        let night_start = Local::now().with_time(nightmode_range.start).unwrap();
        let day_start = Local::now().with_time(nightmode_range.end).unwrap();

        if now < day_start || night_start <= now {
            handle.emit("set_nightmode", ()).unwrap();
            let target_time = if night_start <= now {
                day_start + Duration::days(1)
            } else {
                day_start
            };
            let sleep_duration = target_time - now;
            tokio::time::sleep(sleep_duration.to_std().unwrap()).await;
            handle.emit("set_daymode", ()).unwrap();
        } else {
            let sleep_duration = night_start - now;
            tokio::time::sleep(sleep_duration.to_std().unwrap()).await;
        }
    }
}
