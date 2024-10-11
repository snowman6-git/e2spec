// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
fn main() {
    // tauri_app_lib::run();
    tauri::Builder::default()
    // .invoke_handler(tauri::generate_handler![link])
    .invoke_handler(tauri::generate_handler![sysinfo, ramfresh])
    .run(tauri::generate_context!())
    .expect("failed to run app");

}

// use reqwest::Client;
use serde_json::{json};
use sysinfo::{
    System,
};
fn togb(value: u64) -> String {
    format!("{:.2}GB", (value as f64) / 1024_f64.powf(3.0))
}


#[tauri::command]
fn ramfresh() -> serde_json::Value{ //리턴값을 적어야함, 패키지 밸류는 이런식
    let mut sys = System::new_all();
    sys.refresh_all();
    let info = json!({
        "mem_used": togb(sys.used_memory()),
        "mem_total": togb(sys.total_memory()),
        "vmem_total": togb(sys.total_swap()),
    });
    info.into()
}

#[tauri::command]
fn sysinfo() -> serde_json::Value{ //리턴값을 적어야함, 패키지 밸류는 이런식
    let mut sys = System::new_all();
    sys.refresh_all();
    let system = match System::name() {
        Some(value) => value,
        _ => String::from("Lost"),
    };
    let info = json!({
        "OS": system,
        "CPU_len": sys.cpus().len(),
        "mem_used": togb(sys.used_memory()),
        "mem_total": togb(sys.total_memory()),
        "vmem_total": togb(sys.total_swap()),
    });
    info.into()
}
