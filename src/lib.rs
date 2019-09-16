pub mod client;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let fist_client = super::client::FistClient::new("127.0.0.1", "5575");
        assert_eq!(fist_client.get_addr(), "127.0.0.1:5575");
    }
}
