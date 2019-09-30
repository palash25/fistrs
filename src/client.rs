/// Tools for communicating with the FIST server
use std::io::{Error, Read, Write};
use std::net::TcpStream;

/// The client for the FIST server.
pub struct FistClient {
    addr: String,
    conn: Option<TcpStream>,
}

impl FistClient {
    /// Create a new FistClient for the fist server running on the address provided
    /// ## Arguments
    ///
    /// * `ip` - The IP of the FIST server
    /// * `port` - The port the FIST server is bound to
    ///
    /// ## Example
    ///
    /// ```
    /// let client = fistrs::FistClient::new("localhost", "5575");
    /// assert_eq!(client.get_addr, "localhost:5575");
    /// ```
    pub fn new(ip: &str, port: &str) -> Self {
        FistClient {
            addr: String::from(ip) + ":" + &String::from(port),
            conn: None,
        }
    }

    /// Create a TCP connection to the FIST server
    pub fn connect(&mut self) {
        let res = TcpStream::connect(&self.addr);
        match res {
            Ok(c) => {
                self.conn = Some(c);
            },
            Err(_) => panic!(),
        }
    }

    /// Sends the `VERSION` command to the FIST server
    /// which returns the current version of FIST
    pub fn version(&self) -> Result<Vec<u8>, Error> {
        let mut ver: [u8; 6] = [0;6];
        const VERSION_COMMAND: &[u8] = b"VERSION\r\n";
        let mut conn = self.conn.as_ref().unwrap();
        conn.write(VERSION_COMMAND)?;
        conn.read(&mut ver[..])?;
        Ok(ver.iter().cloned().collect())
    }

    /// Sends the `INDEX` command to index some text in a document
    pub fn index(&self, doc: &str, text: &str) -> Result<Vec<u8>, Error> {
        let mut ver: [u8; 22] = [0;22];
        let index_str = format!("{} {} {}\r\n", "INDEX", doc, text);
        let index_command: &[u8] = index_str.as_bytes();
        let mut conn = self.conn.as_ref().unwrap();
        conn.write(index_command)?;
        conn.read(&mut ver[..])?;
        Ok(ver.iter().cloned().collect())
    }

    /// Sends the `SEARCH` command to look for the provided text.
    /// If successful returns a list of documents in which the text
    /// was found otherwise an empty list (`[]`) is returned by the server
    pub fn search(&self, text: &str) -> Result<Vec<u8>, Error> {
        let mut ver: [u8; 9] = [0;9];
        let search_str = format!("{} {}\r\n", "SEARCH", text);
        let search_command: &[u8] = search_str.as_bytes();
        let mut conn = self.conn.as_ref().unwrap();
        conn.write(search_command)?;
        conn.read(&mut ver[..])?;
        Ok(ver.iter().cloned().collect())
    }

    /// Sends the `DELETE` command to delete all the documents that
    /// contain the provided text string
    pub fn delete(&self, text: &str) -> Result<Vec<u8>, Error> {
        let mut ver: [u8; 8] = [0;8];
        let delete_str = format!("{} {}\r\n", "DELETE", text);
        let delete_command: &[u8] = delete_str.as_bytes();
        let mut conn = self.conn.as_ref().unwrap();
        conn.write(delete_command)?;
        conn.read(&mut ver[..])?;
        Ok(ver.iter().cloned().collect())
    }

    /// Sends the `EXIT` command to the FIST server
    pub fn exit(&self) -> Result<usize, Error> {
        const EXIT_COMMAND: &[u8] = b"EXIT\r\n";
        let mut conn = self.conn.as_ref().unwrap();
        let res = conn.write(EXIT_COMMAND)?;
        Ok(res)
    }

    /// Return the address of the FIST server
    pub fn get_addr(self) -> String {
        self.addr
    }

    /// Return the connection to the FIST server
    pub fn get_conn(self) -> Option<TcpStream> {
        self.conn
    }
}
