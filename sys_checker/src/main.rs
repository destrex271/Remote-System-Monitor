use sysinfo::{System, SystemExt, Component, ProcessExt, DiskUsage, Pid};
use std::collections::HashMap;
use std::thread;

#[derive(Debug)]
struct Proc{
    pid: Pid,
    name: String,
    disk: DiskUsage
}

fn main(){
    // let mut sys = System::new_all();
    let mut i = 0;
    let ram_handler = thread::spawn(move || {
        for i in 1..2{
            println!("\n");
            let mut sys = System::new_all();
            println!("{:?}", get_ram_info(&mut sys));
        }
    });

    let temp_handler = thread::spawn(move || {
        for i in 1..2{
            println!("\n");
            let mut sys = System::new_all();
            println!("Temp");
            println!("{:?}", get_temp_info(&mut sys));
        }
    });

    let proc_handler = thread::spawn(move || {
        for i in 1..2{
            println!("\n");
            let mut sys = System::new_all();
            println!("Process!");
            println!("{:?}", get_process(&mut sys));
        }
    });

    ram_handler.join().unwrap();
    temp_handler.join().unwrap();
    proc_handler.join().unwrap();
}

fn get_ram_info(sys: &mut System) -> HashMap<&str, f32>{
    sys.refresh_all();
    let mut ramInfo = HashMap::new();
    ramInfo.insert("used_ram", sys.used_memory() as f32/(1024.0*1024.0));
    ramInfo.insert("total_ram", sys.total_memory() as f32/(1024.0*1024.0));
    ramInfo.insert("used_swap", sys.used_swap() as f32/(1024.0*1024.0));
    ramInfo.insert("total_swap", sys.total_swap() as f32/(1024.0*1024.0));
    ramInfo
}

fn get_temp_info(sys: &mut System) -> Vec<&sysinfo::Component>{
    sys.refresh_all();
    let mut tempInfo = Vec::new();
    for component in sys.components(){
        tempInfo.push(component);
    }
    tempInfo
}

fn get_process(sys: &mut System) -> HashMap<&Pid, Vec<Proc>>{
    sys.refresh_all();
    let mut processes = HashMap::new();
    for (pid, process) in sys.processes(){
        let mut vc = Vec::new();
        let p : Proc = Proc{
            pid: process.pid(),
            name: String::from(process.name()),
            disk: process.disk_usage()
        };
        vc.push(p);
        processes.insert(pid, vc);
    }
    processes
}
