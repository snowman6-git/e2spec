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
    //println!("{info:?}"); //타입 출력
}

//#[derive(Serialize, Debug)] // Debug 트레이트 추가
//struct Item {
  //  str: String,
    //int: i32,
//}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let info = sysinfo();
    //let json = serde_json::to_string(&info).unwrap();
    
    println!("sysinfo {}", info);

    let client = Client::new();
    //let item = Item {
        //str: String::from("str value"),
        //int: 32,
    //};
    //println!("{:?}", item); // Debug 형식으로 출력

    let response = client.post("http://localhost:8000/myinfo/")        
        .json(&info) // JSON 데이터 전송
        .send()
        .await?;
    // 응답을 HashMap으로 파싱
    //let response_body: HashMap<String, serde_json::Value> = response.json().await?;
    
    println!("{:?}", response);

    Ok(())
}

