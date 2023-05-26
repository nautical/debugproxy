mod header;
use header::*;
use sysinfo::{Pid, ProcessExt, Signal, System, SystemExt};

pub fn start_debugger(pid: Pid) -> bool {
    let mut status = false;
    let mut s = System::new();
    s.refresh_all();
    if let Some(process) = s.process(pid) {
        if process.kill_with(Signal::User1).is_none() {
            error!("Signal User1 is not supported");
        } else {
            status = true;
        }
    } else {
        warn!("No such process : {}", pid);
    }
    return status;
}

pub fn get_process(process_name: &str) -> Vec<Process> {
    let mut s = System::new();
    s.refresh_all();
    let mut processes = Vec::<Process>::new();
    for process in s.processes_by_name(process_name) {
        let p = Process {
            pid: process.pid(),
            name: String::from(process.name()),
            cmd: process.cmd().to_owned(),
            cwd: process.cwd().display().to_string(),
        };
        processes.push(p)
    }
    return processes;
}
