pub mod client;

#[cfg(test)]
mod tests {
    #[test]

    fn it_works() {
        let mut fist_client = super::client::FistClient::new("127.0.0.1", "5575");
        fist_client.connect();
        match fist_client.version() {
            Ok(v) => {
                assert_eq!(String::from_utf8(v).unwrap(), "0.0.2\n")
            }
            Err(_) => panic!(),
        };

        match fist_client.index("doc1", "A string to index") {
            Ok(v) => {
                assert_eq!(String::from_utf8(v).unwrap(), "Text has been indexed\n");
            }
            Err(_) => panic!(),
        };

        match fist_client.search("string to index") {
            Ok(v) => {
                assert_eq!(String::from_utf8(v).unwrap(), "[\"doc1\"]\n");
            }
            Err(_) => panic!(),
        };

        match fist_client.exit() {
            Ok(v) => {
                assert_eq!(v, 6);
            }
            Err(_) => panic!(),
        };
    }
}
