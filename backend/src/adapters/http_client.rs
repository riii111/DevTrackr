use reqwest::Client;

pub struct HttpClientAdapter {
    client: Client,
}

impl HttpClientAdapter {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn get(&self, url: &str) -> Result<String, reqwest::Error> {
        self.client.get(url).send().await?.text().await
    }

    // TODO: POST, PUT, DELETE などの他のHTTPメソッドも同様に実装
}
