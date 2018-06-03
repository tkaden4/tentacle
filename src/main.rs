extern crate ansi_term;
extern crate pnet;
extern crate semver;
extern crate nix;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate clap;
#[macro_use] extern crate matches;

mod util;
mod message;
mod serve;
mod find;

use std::io;
use std::time::Duration;

use util::*;
use message::*;

use serve::*;
use find::*;

use serde::*;
use semver::Version;

fn tentacle(args: clap::ArgMatches) -> Result<(), String> {
    let port = args.value_of("port")
        .ok_or("port not supplied".to_owned())?;
    let port = port.parse::<u16>().unwrap();

    if let Some(serve_args) = args.subcommand_matches("serve") {
        let ping_delay = serve_args.value_of("ping_delay")
            .ok_or("ping-delay not supplied".to_owned())?;
        let ping_delay = ping_delay.parse::<u64>()
            .map_err(|_| "ping-delay is not a valid number".to_owned())
            .map(|x| Duration::from_secs(x))?;

        let serve_time = serve_args.value_of("serve_time")
            .ok_or("serve_time not supplied".to_owned())?;
        let serve_time = serve_time.parse::<u64>()
            .map_err(|_| "serve_time is not a valid number".to_owned())
            .map(|x| Duration::from_secs(x))?;

        let service_name = serve_args.value_of("NAME")
            .ok_or("NAME not supplied".to_owned())?;

        let options = ServeOptions {
            name: "service".into(),
            port: port,
            ping_delay: ping_delay,
            serve_time: serve_time
        };

        serve(options)
            .map(|_| ())
            .map_err(|err| format!("{}", err))?
    } else if let Some(find_args) = args.subcommand_matches("find") {
        let timeout = find_args.value_of("timeout")
            .ok_or("timeout not supplied".to_owned())?;
        let timeout = timeout.parse::<u64>()
            .map_err(|_| "timeout is not a valid number")
            .map(|x| Duration::from_secs(x))?;
        let options = FindOptions {
            port: port,
            timeout: timeout,
            json: find_args.is_present("json")
        };
        find(options)
            .map(|_| ())
            .map_err(|err| format!("{}", err))?
    } else {
        return Err("subcommand not supplied".to_owned());
    }
    Ok(())
}

fn main() {
    let args = clap_app!(tentacle =>
        (version: "0.1.0")
        (about: "Local service discovery")
        (@arg v: -v --verbosity "Verbose output")
        (@arg port: --port -p +takes_value +required "Port to use")
        (@subcommand serve =>
         (about: "Advertise a service")
         (@arg NAME: +required +takes_value "Name of the service")
         (@arg ping_delay: --pingdelay +takes_value +required "Time between pings")
         (@arg serve_time: --servetime +takes_value +required "Time to serve"))
        (@subcommand find =>
         (about: "Find a service")
         (@arg json: --json "Output in json")
         (@arg timeout: --timeout -t +takes_value +required "Amount of time to look for services (in seconds)"))
    ).get_matches();
    if let Err(err) = tentacle(args) {
        eprintln!("tentacle: {}", err);
    }
}
