// Tools for communicating with the FIST server
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
            Ok(c) => self.conn = Some(c),
            Err(_) => panic!(),
        }
    }

    // Return the address of the FIST server
    pub fn get_addr(self) -> String {
        self.addr
    }
}
