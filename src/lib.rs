use std::net::TcpStream;

pub struct FistClient {
    addr: String,
    //conn: TcpStream,
}

impl FistClient {
    pub fn new(ip: &str, port: &str) -> FistClient {
        FistClient {
            addr: String::from(ip) + ":" + &String::from(port),
            //conn: TcpStream{},
        }
    }

    pub fn connect(&mut self) {
        let res = TcpStream::connect(&self.addr);
        match res {
            Ok(_) => print!("init conn"),
            Err(_) => panic!(),
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let client = super::FistClient::new("127.0.0.1", "5575");
        assert_eq!(client.addr, "127.0.0.1:5575");
    }
}
