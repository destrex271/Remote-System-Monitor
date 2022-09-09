use sysinfo::{System, SystemExt, Component, ProcessExt, DiskUsage, Pid};
use std::collections::HashMap;

#[derive(Debug)]
struct Proc{
    pid: Pid,
    name: String,
    disk: DiskUsage
}

fn main(){
    let mut sys = System::new_all();
    let mut i = 0;
    loop{
        println!("{:?}", get_temp_info(&mut sys));
        println!("{:?}",get_process(&mut sys)); 
        println!("{:?}", get_ram_info(&mut sys));
        i += 1;
        if i == 2{
            println!("OK!");
            break;
        }
    }
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
