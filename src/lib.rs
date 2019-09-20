pub mod client;


#[cfg(test)]
mod tests {
    #[test]

    fn it_works() {
        let mut fist_client = super::client::FistClient::new("127.0.0.1", "5575");
        //assert_eq!(fist_client.get_addr(), "127.0.0.1:5575");
        fist_client.connect();
        match fist_client.version() {
            // println!("{} {} {} {} {} {} {} {}", v[0] as char, v[1], v[2], v[3], v[4], v[5], v[6], v[7])
            Ok(v) => {
                let response = String::from_utf8(v).unwrap();
                println!("{}", response);
            }
            Err(_) => panic!(),
        };
    }
}
