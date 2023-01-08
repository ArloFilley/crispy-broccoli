use serde::Serialize;
use sysinfo::{
    System,
    SystemExt,
    DiskExt,
    DiskType
};

#[derive(Serialize)]
pub struct Disk {
    name: String,
    disk_type: String,
    total_space: String,
    available_space: String,
    usage: String,
    file_system: String
}

/// Gathers relevant info about disks and returns an vector of disks
pub fn disk_info(system: &System) -> Vec<Disk> {
    let mut disks: Vec<Disk> = vec![];

    for disk in system.disks() {
        // Get's the partition name of the disk
        let disk_name = disk.name().to_str().unwrap().to_string();

        // Gets whether the disk is an SSD, HDD, or Network Drive
        let disk_type_raw = disk.type_();
        let disk_type: String;
        match disk_type_raw {
            DiskType::HDD => disk_type = String::from("HDD"),
            DiskType::SSD => disk_type = String::from("SSD"),
            DiskType::Unknown(_) => disk_type = String::from("Network")
        }

        // Calculates the Disk usage;
        let disk_space = disk.total_space();
        let disk_available_space = disk.available_space();
        let disk_used_space = disk_space - disk_available_space;
        let disk_usage = format!("{:.2}%",(disk_used_space as f64 / disk_space as f64 * 100.0));

        let file_system_raw = disk.file_system();
        let mut file_system = String::new();

        for letter in file_system_raw {
            file_system.push(*letter as char);
        }

        disks.push( Disk {
            name: disk_name,
            disk_type: disk_type,
            total_space: to_disk_units(disk_space),
            available_space: to_disk_units(disk_available_space),
            usage: disk_usage,
            file_system: file_system,
        });
    }

    disks
}

pub fn to_disk_units(space: u64) -> String {
    let len = space.to_string().len();
    let space: f64 = space as f64;
    if len > 12 {
        return format!("{:.2}TB", space / 1_000_000_000_000.0);
    } else if len > 9 {
        return format!("{:.2}GB", space / 1_000_000_000.0);
    } else if len > 6 {
        return format!("{:.2}MB", space / 1_000_000.0);
    } else if len > 3 {
        return format!("{:.2}KB", space / 1_000.0);
    } else {
        return format!("{:.2}BY", space);
    }
}