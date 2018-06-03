use std::net::*;
use std::io;
use std::time::{Duration, Instant};
use std::thread;

use util;
use message::*;

use serde_json;

pub struct ServeOptions {
    pub name: String,
    pub port: u16,
    pub ping_delay: Duration,
    pub serve_time: Duration
}

#[derive(Serialize)]
pub struct TentacleBroadcast {
    pub service: util::Service
}

/* ping network for devices */
pub fn serve(options: ServeOptions) -> io::Result<()> {
    let info = util::interfaces()
        .unwrap()
        .nth(0)
        .unwrap();

    let sock = UdpSocket::bind(("0.0.0.0:0"))?;
    sock.set_broadcast(true)?;

    let start_time = Instant::now();
    while Instant::now() - start_time < options.serve_time {
        let message = MessageBuilder::<String>::new()
            .build();
        let message_str = serde_json::to_string(&message)
            .unwrap();

        println!("{}", message_str);

        sock.send_to(message_str.as_bytes(), (info.broadcast, options.port))?;

        thread::sleep(options.ping_delay);
    }

    Ok(())
}
