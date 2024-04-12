// this function checks for the -p or --port flags and returns a unsigned 16 int aka the arg
pub fn check_port() -> u16 {
    let default_port: u16 = 8080;
    let args: Vec<String> = std::env::args().collect();
    // iterate through arguements checking for port arguement
    for (index, arg) in args.iter().enumerate() {
        if arg == "-p" || arg == "--port" {
            // check if a arguement exists after -p
            if let Some(port_str) = args.get(index + 1) {
                /* if string converts to unsigned 16 int then return the port
                otherwise error etc. default to default_port */
                if let Ok(port) = port_str.parse::<u16>() {
                    println!("Port set to {}", port);
                    return port;
                }
            }
        }
    }
    // if there are no arguments return default port
    println!("No valid port specified defaulting to {}", default_port);
    return default_port;
}
