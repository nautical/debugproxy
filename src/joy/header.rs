use sysinfo::Pid;

pub struct ConsoleJoy {
    pub id: Pid,
    pub ws_url: String,
    pub ws_uuid: String,
    pub remote_ws_url: String,
    pub remote_ws_uuid: String,
    pub remote_ws_token: String,
    pub ws_disconnect_count: i64,
    pub remote_ws_disconnect_count: i64,
}
