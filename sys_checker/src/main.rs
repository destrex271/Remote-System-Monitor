use sysinfo::{System, SystemExt, Component, ProcessExt, DiskUsage, Pid};
use std::collections::HashMap;

struct Proc{
    pid: Pid,
    name: String,
    disk: DiskUsage
}

fn main(){
    let mut sys = System::new_all();
    let mut i = 0;
    loop{
        // println!("{:?}", getTempInfo(&mut sys));
        getProcess(&mut sys);
        i += 1;
        if i == 2{
            println!("OK!");
            break;
        }
    }
}

fn getRamInfo(sys: &mut System) -> HashMap<&str, f32>{
    sys.refresh_all();
    let mut ramInfo = HashMap::new();
    ramInfo.insert("used_ram", sys.used_memory() as f32/(1024.0*1024.0));
    ramInfo.insert("total_ram", sys.total_memory() as f32/(1024.0*1024.0));
    ramInfo.insert("used_swap", sys.used_swap() as f32/(1024.0*1024.0));
    ramInfo.insert("total_swap", sys.total_swap() as f32/(1024.0*1024.0));
    ramInfo
}

fn getTempInfo(sys: &mut System) -> Vec<&sysinfo::Component>{
    sys.refresh_all();
    let mut tempInfo = Vec::new();
    for component in sys.components(){
        tempInfo.push(component);
    }
    tempInfo
}

fn getProcess(sys: &mut System){
    sys.refresh_all();
    //let mut processes = HashMap::new();
    for (pid, process) in sys.processes(){
        /*let mut vc = Vec::new();
        vc.push(process.name());
        vc.push(process.disk_usage());
        processes.insert(pid, vc);*/
    }
    
    /*for x in processes.iter(){
        println!("{:?}", x);
    }*/

}
