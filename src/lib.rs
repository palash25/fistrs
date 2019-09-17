pub mod client;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let mut fist_client = super::client::FistClient::new("127.0.0.1", "5575");
        //assert_eq!(fist_client.get_addr(), "127.0.0.1:5575");
        fist_client.connect();
        match fist_client.version() {
            Ok(v) => println!("{}", v),
            Err(_) => panic!(),
        };
    }
}
