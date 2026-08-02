use anyhow::{Context, Result};
use capstone_08_solution::{KvCommand, KvResponse, KvStore};
use clap::{Parser, Subcommand};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Parser)]
#[command(name = "kv", about = "Distributed Key-Value Store")]
struct Cli {
    #[arg(long, default_value = "follower")]
    role: String,

    #[arg(long, default_value_t = 9000)]
    port: u16,

    #[arg(long)]
    leader_addr: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Set { key: String, value: String },
    Get { key: String },
    Delete { key: String },
    Keys,
    Info,
}

fn send_command(addr: &str, cmd: KvCommand) -> Result<KvResponse> {
    let mut stream = TcpStream::connect(addr).context("connect")?;
    let json = KvStore::serialize_command(&cmd);
    stream.write_all(json.as_bytes()).context("write")?;
    stream.flush().context("flush")?;

    let mut reader = BufReader::new(&mut stream);
    let mut line = String::new();
    reader.read_line(&mut line).context("read")?;
    KvStore::deserialize_response(&line).map_err(|e| anyhow::anyhow!("deserialize response: {}", e))
}

fn handle_client(store: Arc<Mutex<KvStore>>, mut stream: TcpStream) {
    let stream_clone = stream.try_clone().expect("clone tcp stream");
    let mut reader = BufReader::new(stream_clone);
    let mut line = String::new();

    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {}
            Err(_) => break,
        }

        let cmd = match KvStore::deserialize_command(&line) {
            Ok(c) => c,
            Err(e) => {
                let resp = KvResponse::Error {
                    message: format!("parse error: {}", e),
                };
                let json = KvStore::serialize_response(&resp);
                let _ = stream.write_all(json.as_bytes());
                let _ = stream.flush();
                continue;
            }
        };

        let response = {
            let mut s = store.lock().unwrap();
            s.handle_command(cmd)
        };

        let json = KvStore::serialize_response(&response);
        let _ = stream.write_all(json.as_bytes());
        let _ = stream.flush();
    }
}

fn start_server(store: Arc<Mutex<KvStore>>, port: u16) -> Result<()> {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))?;
    println!("KV server listening on port {}", port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let store = Arc::clone(&store);
                thread::spawn(move || handle_client(store, stream));
            }
            Err(e) => {
                eprintln!("connection error: {}", e);
            }
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let node_id = format!("node-{}", cli.port);
    let store = Arc::new(Mutex::new(KvStore::new(node_id.clone())));

    if cli.role == "leader" {
        store.lock().unwrap().become_leader();
    }

    if let Some(cmd) = cli.command {
        match cmd {
            Commands::Set { key, value } => {
                let cmd = KvCommand::Set { key, value };
                if let Some(ref addr) = cli.leader_addr {
                    let resp = send_command(addr, cmd)?;
                    println!("{:?}", resp);
                } else {
                    let resp = store.lock().unwrap().handle_command(cmd);
                    println!("{:?}", resp);
                }
            }
            Commands::Get { key } => {
                let cmd = KvCommand::Get { key };
                if let Some(ref addr) = cli.leader_addr {
                    let resp = send_command(addr, cmd)?;
                    println!("{:?}", resp);
                } else {
                    let resp = store.lock().unwrap().handle_command(cmd);
                    println!("{:?}", resp);
                }
            }
            Commands::Delete { key } => {
                let cmd = KvCommand::Delete { key };
                if let Some(ref addr) = cli.leader_addr {
                    let resp = send_command(addr, cmd)?;
                    println!("{:?}", resp);
                } else {
                    let resp = store.lock().unwrap().handle_command(cmd);
                    println!("{:?}", resp);
                }
            }
            Commands::Keys => {
                let cmd = KvCommand::Keys;
                if let Some(ref addr) = cli.leader_addr {
                    let resp = send_command(addr, cmd)?;
                    println!("{:?}", resp);
                } else {
                    let resp = store.lock().unwrap().handle_command(cmd);
                    println!("{:?}", resp);
                }
            }
            Commands::Info => {
                let s = store.lock().unwrap();
                println!("Node ID: {}", s.node_id());
                println!("Role: {:?}", s.role());
                println!("Term: {}", s.term());
                println!("Leader: {}", s.leader_id().unwrap_or("<unknown>"));
            }
        }
    } else {
        start_server(store, cli.port)?;
    }

    Ok(())
}
