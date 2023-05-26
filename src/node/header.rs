use sysinfo::Pid;

pub struct Process {
    pub pid: Pid,
    pub name: String,
    pub cwd: String,
    pub cmd: Vec<String>,
}
