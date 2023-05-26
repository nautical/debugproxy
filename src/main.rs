mod debugger;
mod joy;
mod node;
use clap::Parser;
use std::fs::File;
use std::process;
use sysinfo::Pid;

#[macro_use]
extern crate log;

extern crate daemonize;
use daemonize::Daemonize;

/// ConsoleJoy : One click debugger for NodeJS Applications
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// Process name to attach debugger to
    #[clap(short = 'p', long, env = "CJ_PROCESS")]
    process: String,

    /// Remote Dashboard URL
    #[clap(short = 'u', long, env = "CJ_URL")]
    url: String,

    /// Client ID ( can be found in settings )
    #[clap(short = 'c', long, env = "CJ_CLIENT")]
    client: String,

    /// Client Secret ( can be found in settings )
    #[clap(short = 's', long, env = "CJ_SECRET")]
    secret: String,

    /// Daemonize the application
    #[clap(short = 'd', long)]
    daemon: bool,
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    if args.daemon {
        let stderr = File::create("/tmp/consolejoy.log").unwrap();

        let daemonize = Daemonize::new()
            .pid_file("/tmp/consolejoy.pid")
            .chown_pid_file(true)
            .working_directory("/tmp")
            .user("nobody")
            .group("daemon")
            .group(2)
            .umask(0o777)
            .stderr(stderr)
            .privileged_action(|| "Executed before drop privileges");

        match daemonize.start() {
            Ok(_) => info!("Success, daemonized"),
            Err(e) => error!("Error, {}", e),
        }
    }
    ctrlc::set_handler(move || {
        info!("Exiting");
        process::exit(1);
    })
    .expect("Error setting Ctrl-C handler");

    let processes = node::get_process("node");
    if processes.len() > 0 {
        let mut console_joy = joy::init(args.url, args.client, args.secret);
        let mut pid: Pid = Pid::from(0);
        for process in processes {
            if process.cmd.len() > 1 && process.cmd[1].contains(&args.process) {
                pid = Pid::from(process.pid);
                console_joy.set_id(Pid::from(pid));
            }
        }
        node::start_debugger(pid);
        let json = debugger::get_ws_url_tokio();
        match json {
            Ok(json_object) => {
                if json_object.len() > 0 {
                    console_joy.set_ws_uuid(json_object[0].id.clone());
                    console_joy.exec();
                }
            }
            Err(e) => {
                warn!("{:?}", e.to_string());
            }
        }
    } else {
        error!("Node process not found");
    }
}
