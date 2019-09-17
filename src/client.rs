// Tools for communicating with the FIST server
use std::io::{Error, Write};
use std::net::TcpStream;

// The client for the FIST server.
pub struct FistClient {
    addr: String,
    conn: Option<TcpStream>,
}

impl FistClient {
    // Create a new FistClient
    pub fn new(ip: &str, port: &str) -> Self {
        FistClient {
            addr: String::from(ip) + ":" + &String::from(port),
            conn: None,
        }
    }

    // Create a TCP connection to the FIST server
    pub fn connect(&mut self) {
        let res = TcpStream::connect(&self.addr);
        match res {
            Ok(c) => {
                self.conn = Some(c);
                println!("done\n");
            },
            Err(_) => panic!(),
        }
    }

    // Sends the `VERSION` command to the FIST server
    // which returns the current version of FIST
    pub fn version(self) -> Result<usize, Error> {
        let command_string = b"VERSION\r\n";
        self.conn.unwrap().write(&command_string[0..])
    }

    // Return the address of the FIST server
    pub fn get_addr(self) -> String {
        self.addr
    }

    // Return the connection to the FIST server
    pub fn get_conn(self) -> Option<TcpStream> {
        self.conn
    }
}
