use redis::Client;
use std::io::Result;

pub fn create_redis_client(redis_url: &str) -> Result<Client> {
    Client::open(redis_url).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}
