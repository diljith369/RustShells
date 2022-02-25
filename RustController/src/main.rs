use std::io::{self,  BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let controller = TcpListener::bind("127.0.0.1:443").expect("Could not bind to the port");

    for mut in_stream in controller.incoming() {
        match in_stream {
            Err(e) => {
                eprintln!("Failed {}", e)
            }
            Ok(in_stream) => {
                println!(
                    "Received connection from {}",
                    in_stream.peer_addr().unwrap()
                );

                handle_agent(in_stream);
            }
        }
    }
}

fn handle_agent(mut incomingstream: TcpStream) {
    loop {
        let mut command = String::new();
        print!(">> ");
        io::stdout().flush().expect("failed to get it");
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read from stdin");
        //let finalcommand = command.strip_suffix("\r\n").or(command.strip_suffix("\n")).unwrap_or(command);

        incomingstream
            .write(command.as_bytes())
            .expect("Failed to write to server");
        let mut cmd_result = [0 as u8; 5096];

        let mut incomingstreamreader = BufReader::new(&incomingstream);

        incomingstreamreader.read(&mut cmd_result).unwrap();
        println!("{}", String::from_utf8_lossy(&cmd_result).to_string());
    }
}
