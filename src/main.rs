use reqwest::Client;
use serde_json::{json};

//use std::collections::HashMap;

use sysinfo::{
    System,
};

fn togb(value: u64) -> f64 {(value as f64) / 1024_f64.powf(3.0)}
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
    return info
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let info = sysinfo();
    println!("sysinfo {}", info);
    let client = Client::new();
    //let response = client.post("https://aa2.e2spec.p-e.kr/myinfo/")        
    let response = client.post("http://localhost:8000/myinfo/")        
        .json(&info) // JSON 데이터 전송
        .send()//보내고
        .await?//기다리고
        .text()//받기를
        .await?;//기다리기
    println!("링크가 생성되었어요: {:?}", response);
    let mut input = String::new();
    std::io::stdin();
    .read_line(&mut input)
    Ok(())
}

