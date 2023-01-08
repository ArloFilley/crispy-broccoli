use crate::{disk::disk_info, disk::to_disk_units, process::process_info, process::run_time};
use serde::Serialize;
use sysinfo::{
    System,
    SystemExt
};
use reqwest::{self, Response, StatusCode};
use tokio::time::{sleep, Duration};

use std::{fs, io::Write};

mod disk;
mod process;

#[derive(Serialize)]
struct Systems {
    key: String,
    host_name: String,
    uptime: String,
    os: String,
    total_ram: String,
    used_ram: String,
    available_ram: String,
    ram_usage: String,
    total_swap: String,
    used_swap: String,
    available_swap: String,
    swap_usage: String,
    disks: Vec<disk::Disk>,
    processes: Vec<process::Process>,
}

#[tokio::main]
async fn main() {
    let mut system = System::new_all();
    
    let mut file = fs::File::create("foo.json").unwrap();

    loop {
        system.refresh_all();
        file.write_all(info(&mut system, true).as_bytes()).unwrap();
        post(info(&mut system, false)).await;
        sleep(Duration::from_secs(5)).await;
    }
}

async fn post(info: String) {
    let request = reqwest::Client::new().post("http://arlofilley.com/api/servers")
        .body(info)
        .send()
        .await;

    match request {
        Err(why) => println!("Error {why}"),
        Ok(response) => {
            match response.status() {
                StatusCode::OK => println!("Sent Data From Arch Server"),
                _ => println!("Unsucessful")
            }
        }
    }
}

fn info(system: &mut System, pretty: bool) -> String {
    let host_name = system.host_name().unwrap();

    let disks = disk_info(&system);    
    let processes = process_info(&system);
    let uptime = run_time(system.uptime());
    let os = system.long_os_version().unwrap();
    let total_ram = to_disk_units(system.total_memory());
    let used_ram = to_disk_units(system.used_memory());
    let available_ram = to_disk_units(system.available_memory());
    let ram_usage = (system.used_memory() / system.total_memory() * 100).to_string();
    let total_swap = to_disk_units(system.total_swap());
    let used_swap = to_disk_units(system.used_swap());
    let free_swap = to_disk_units(system.free_swap());
    let swap_usage = (system.used_swap() / system.total_swap() * 100).to_string();

    let systems = Systems {
        key: "0etnmXPSr95@FNy6A3U9Bw*ZupNIR85zI!hRFGIdj6SW$Tu0q%".to_string(),
        host_name: host_name,
        uptime: uptime,
        os: os,
        total_ram: total_ram,
        used_ram: used_ram,
        available_ram: available_ram,
        ram_usage: ram_usage,
        total_swap: total_swap,
        used_swap: used_swap,
        available_swap: free_swap,
        swap_usage: swap_usage,
        disks: disks,
        processes: processes,
    };

    if pretty {
        return serde_json::to_string_pretty(&systems).unwrap();
    } else {
        return serde_json::to_string(&systems).unwrap();
    }
    
}