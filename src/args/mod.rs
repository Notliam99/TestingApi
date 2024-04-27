use std::{env, net::TcpListener};

fn port_is_available(port: u16) -> bool {
    match TcpListener::bind(("127.0.0.1", port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// # Arg_Port
/// This function checks for the -p or --port flags and returns a unsigned 16 int aka the port
/// number. It also verifys that the port is avalible.
///
/// # Examples
///
/// ```
/// # #[cfg(test)]
/// # mod test {
/// # #[test]
/// # fn test() {
/// use testing_api::args::arg_port;
///
/// panic_unless!(arg_port() == 8000u16)
/// # }}
/// ```
pub fn arg_port() -> u16 {
    let default_port: u16 = if cfg!(test) {
        8000u16
    } else {
        (8000..9000)
            .find(|port| port_is_available(*port))
            .expect("Could Not Find A Open Port")
    }; // Finds A Available Port On the System

    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            // no args are set
            return default_port;
        }
        2 => {
            // one pram is set with no value
            if args[1] == "-p" || args[1] == "--port" {
                println!("Parameter Must Have VALUE eg. `--port {default_port}`\n");
            } else {
                println!(
                    "Parameter {} Not Recognised Only Accept `-p` And `--port`\n",
                    args[1]
                )
            }
            return default_port;
        }
        _ => {
            // a pram and a value is set
            for (index, item) in args.iter().enumerate() {
                if item == "-p" || item == "--port" {
                    let port = args
                        .get(index + 1)
                        .unwrap()
                        .parse::<u16>()
                        .unwrap_or(default_port);

                    if !port_is_available(port) || cfg!(test) {
                        println!("Port Was Not Avalible Using Port: ( {default_port} )\n");
                        return default_port;
                    }

                    return port;
                }
            }
            return default_port;
        }
    }
}
