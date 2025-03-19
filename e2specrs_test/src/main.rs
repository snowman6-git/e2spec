use sysinfo::{
    // System, RefreshKind, CpuRefreshKind, 
    Disks
};

fn togb(value: u64) -> String {
    format!("{:.2}GB", (value as f64) / 1024_f64.powf(3.0))
}

fn main() {

    let disks = Disks::new_with_refreshed_list();
    for disk in disks.list() {
        println!("{:?}: {:?} {} {}", disk.name(), disk.kind(), togb(disk.total_space() - disk.available_space()), togb(disk.total_space()));
    } 
    //HDD or SDD

    // let disks = Disks::new_with_refreshed_list();

    // let mut disk_used: u64 = 0;
    // let mut disk_total: u64 = 0;

    // for disk in disks.list() {
    //     disk_used += disk.available_space();
    //     disk_total += disk.total_space();
    // }
    // println!("{} / {}", togb(disk_used), togb(disk_total))
}
