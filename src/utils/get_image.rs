use reqwest::Client;

pub async fn get_image_from_url(url: String) -> Option<Vec<u8>> {
    let client = Client::new();
    let request = client.get(&url)
        .send()
        .await;

    match request {
        Ok(response) => {
            let body = response.bytes().await.unwrap();
            Some(body.to_vec())
        },
        Err(_) => None
        
    }
}