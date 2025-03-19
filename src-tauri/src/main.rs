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
    System, Disks
};

fn togb(value: u64) -> String {
    if value >= 1024 * 1024 * 1024 * 1024 {
        // TB 단위로 변환
        let tb_value = (value as f64) / (1024_f64.powf(4.0));
        format!("{:.2}TB", tb_value)
    } else {
        // GB 단위로 변환
        let gb_value = (value as f64) / 1024_f64.powf(3.0);
        format!("{:.2}GB", gb_value)
    }
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

    let cpu_name = sys.cpus()[0].brand().to_string(); //CPU명을 잡음
    let name_to_array: Vec<&str> = cpu_name.split("with").collect(); //with같은 길게 나오는 지랄을 자르기 위해준비
    let sorted_cpu_name: String = name_to_array[0].trim().to_string(); //자르고 string으로 만듬


    let disks = Disks::new_with_refreshed_list();

    let mut disk_used: u64 = 0;
    let mut disk_total: u64 = 0;

    for disk in disks.list() {
        disk_used += disk.total_space() - disk.available_space();
        disk_total += disk.total_space();
    }
    let info = json!({
        "OS": system,
        "kernal": System::kernel_version(),
        "cpu_name": sorted_cpu_name,
        "cpu_len": sys.cpus().len(),
        "disk_used": togb(disk_used),
        "disk_total": togb(disk_total),
        "mem_used": togb(sys.used_memory()),
        "mem_total": togb(sys.total_memory()),
        "vmem_total": togb(sys.total_swap()),
    });
    info.into()
}
