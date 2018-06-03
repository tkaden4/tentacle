use std::net::*;
use std::io;
use std::time::{Duration, Instant};
use std::thread;

use std::str;
use std::u16;

use util;

pub struct FindOptions {
    pub port: u16,
    pub timeout: Duration,
    pub json: bool
}

pub fn find(options: FindOptions) -> io::Result<()> {
    let socket = UdpSocket::bind(("0.0.0.0", options.port))?;
    socket.set_nonblocking(true)?;

    let start_time = Instant::now();
    let mut buf = [0; u16::MAX as usize];
    while Instant::now() - start_time < options.timeout {
        match socket.recv_from(&mut buf[..]) {
            Ok(_) => {
                let data = str::from_utf8(&buf).unwrap();
                /* TODO handle options.json */
                println!("{}", data);
            },
            _ => {}
        }
    }

    Ok(())
}
