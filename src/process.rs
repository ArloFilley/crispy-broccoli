use crate::disk::to_disk_units;
use serde::Serialize;
use sysinfo::{
    System,
    SystemExt,
    ProcessExt
};

#[derive(Serialize)]
pub struct Process {
    name: String,
    memory: String,
    run_time: String,
    id: String,
    user_id: String,
    virtual_memory: String
}

pub fn process_info(system: &System) -> Vec<Process> {
    let mut processes: Vec<Process> = vec![];

    for (process_id, process) in system.processes() {
        processes.push( Process {
            name: process.name().to_string(),
            memory: to_disk_units(process.memory()),
            run_time: run_time(process.run_time()),
            id: process_id.to_string(),
            user_id: process.user_id().unwrap().to_string(),
            virtual_memory: to_disk_units(process.virtual_memory()),
        })
    }
    processes
}

pub fn run_time(time: u64) -> String {
    let time = time as f64;
    if time > 3600.0 {
        return format!("{:.2}H", time / 3600.0);
    } else if time > 60.0 {
        return format!("{:.2}M", time / 60.0);
    } else {
        return format!("{:.2}S", time);
    }
}