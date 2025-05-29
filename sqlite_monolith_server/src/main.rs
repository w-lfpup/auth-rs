use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::conn::auto::Builder;
use std::env;
use std::path::PathBuf;
use tokio::net::TcpListener;

use config::Config;
use sqlite_interface;

#[tokio::main]
async fn main() -> Result<(), String> {
    let conf = match get_config().await {
        Ok(c) => c,
        Err(e) => return Err(e),
    };

    let listener = match TcpListener::bind(&conf.host_and_port).await {
        Ok(lstnr) => lstnr,
        Err(e) => return Err(e.to_string()),
    };

    println!("auth_server: {}", conf.host_and_port);

    // arc sqlite connections
    // return mutex of connection
    // fn (&mut Connection) {}

    loop {
        let (stream, _remote_address) = match listener.accept().await {
            Ok(strm) => strm,
            Err(e) => return Err(e.to_string()),
        };

        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            // log service errors here
            // Builder::new(TokioExecutor::new())
            //     .serve_connection(io, svc)
            //     .await
        });
    }
}

async fn get_config() -> Result<Config, String> {
    match env::args().nth(1) {
        Some(conf_path) => {
            let conf_path_buf = PathBuf::from(conf_path);
            return Config::try_from(&conf_path_buf).await;
        }
        _ => Err("no config found".to_string()),
    }
}
