mod header;
use crate::{debugger::get_ws_url, node};
use core::time;
use futures_util::{future, StreamExt};
use header::*;
use http::{header::AUTHORIZATION, HeaderValue};
use std::{thread, time::Duration, ptr::null};
use sysinfo::Pid;
use tokio::sync::oneshot;
use tokio_tungstenite::{connect_async, tungstenite::client::IntoClientRequest, WebSocketStream};

impl ConsoleJoy {
    pub fn new(url: String, uuid: String, token: String) -> Self {
        ConsoleJoy {
            id: Pid::from(0),
            ws_url: "ws://127.0.0.1:9229".to_string(),
            ws_uuid: "".to_string(),
            ws_disconnect_count: 0,
            remote_ws_url: if url.len() > 0 {
                url
            } else {
                "ws://127.0.0.1:12345".to_string()
            },
            remote_ws_uuid: uuid,
            remote_ws_token: token,
            remote_ws_disconnect_count: 0,
        }
    }

    pub fn set_id(&mut self, id: Pid) {
        self.id = id;
    }

    pub fn set_ws_uuid(&mut self, uuid: String) {
        self.ws_uuid = uuid;
    }

    fn increment_ws_disconnect_count(&mut self) {
        self.ws_disconnect_count += 1;
    }

    fn reset_ws_disconnect_count(&mut self) {
        self.ws_disconnect_count = 0;
    }

    fn increment_remote_ws_disconnect_count(&mut self) {
        self.remote_ws_disconnect_count += 1;
    }

    fn reset_remote_ws_disconnect_count(&mut self) {
        self.remote_ws_disconnect_count = 0;
    }

    async fn connect(
        mut shutdown: oneshot::Receiver<()>,
        url: String,
        uuid: String,
        key: String,
    ) -> Option<(
        WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
        tokio_tungstenite::tungstenite::http::Response<()>,
    )> {
        loop {
            let mut socket_request = format!("{}/{}", url, uuid).into_client_request().unwrap();
            if key.len() > 0 {
                socket_request
                    .headers_mut()
                    .insert(AUTHORIZATION, HeaderValue::from_str(&key).unwrap());
            }

            tokio::select! {
              biased;
              _ = &mut shutdown => return None,
              r = connect_async(socket_request) => {
                match r {
                  Ok(stream) => return Some(stream),
                  Err(err) => {
                    error!("{} {:?}", &format!("{}/{}", url, uuid),  err.to_string());
                    tokio::time::sleep(Duration::from_secs(5)).await;
                  }
                }
              }
            }
        }
    }

    #[tokio::main]
    pub async fn exec(&mut self) {
        let delay = time::Duration::from_secs(30);
        loop {
            let (remote_src, remote_dst) = oneshot::channel();
            let (local_src, local_dst) = oneshot::channel();
            tokio::spawn(async {
                tokio::select! {
                    _ = tokio::signal::ctrl_c() => (),
                    _ = tokio::time::sleep(Duration::from_secs(30)) => ()
                }
                remote_src.send(()).unwrap_or_default();
            });
            tokio::spawn(async {
                tokio::select! {
                    _ = tokio::signal::ctrl_c() => (),
                    _ = tokio::time::sleep(Duration::from_secs(30)) => ()
                }
                local_src.send(()).unwrap_or_default();
            });
            let stream = ConsoleJoy::connect(
                remote_dst,
                self.remote_ws_url.clone(),
                self.remote_ws_uuid.clone(),
                self.remote_ws_token.clone(),
            )
            .await;
            match stream {
                Some((remote_ws_stream, _response2)) => {
                    self.reset_remote_ws_disconnect_count();
                    let (remote_writer, remote_reader) =
                        remote_ws_stream.filter_map(|i| async { Some(i) }).split();
                    let stream = ConsoleJoy::connect(
                        local_dst,
                        self.ws_url.clone(),
                        self.ws_uuid.clone(),
                        "".to_string(),
                    )
                    .await;
                    match stream {
                        Some((debug_ws_stream, _response)) => {
                            self.reset_ws_disconnect_count();
                            let (debug_writer, debug_reader) =
                                debug_ws_stream.filter_map(|i| async { Some(i) }).split();
                            let remote_to_debug = remote_reader.filter_map(|x| async move {
                                // println!("{:?}", x);
                                if x.is_ok() {
                                    Some(x)
                                } else {
                                    None
                                }
                            }).boxed().forward(debug_writer);
                            let debug_to_remote = debug_reader.forward(remote_writer);
                            info!("Bridge success");
                            future::select(remote_to_debug, debug_to_remote).await;
                        }
                        _err => {
                            error!("Connecting to debugger ws");
                            self.increment_ws_disconnect_count();
                            if self.ws_disconnect_count >= 2 {
                                info!("Local WS UUID not found, resetting");
                                let processes = node::get_process("node");
                                for process in processes {
                                    if process.cmd.len() > 1 && process.cmd[1].contains("www") {
                                        self.set_id(Pid::from(process.pid));
                                    }
                                }
                                node::start_debugger(self.id);
                                let jsonobj = get_ws_url().await;
                                match jsonobj {
                                    Ok(json_object) => {
                                        self.set_ws_uuid(json_object[0].id.clone());
                                    }
                                    Err(e) => {
                                        error!("{:?}", e.to_string());
                                    }
                                }
                            }
                            thread::sleep(delay);
                        }
                    }
                }
                _err => {
                    error!("Connecting to remote ws");
                    self.increment_remote_ws_disconnect_count();
                    thread::sleep(delay);
                }
            }
        }
    }
}

pub fn init(url: String, uuid: String, token: String) -> ConsoleJoy {
    return ConsoleJoy::new(url, uuid, token);
}
