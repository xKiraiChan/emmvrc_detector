use reqwest::Client;

pub async fn check_id(client: &Client, id: String) -> u16 {
    let body = format!(r#"{{"username":"{}","name":"tupper","password":"{}","loginKey":"1"}}"#, id, id);
    
    match client
    .post("https://thetrueyoshifan.com:3000/api/authentication/login")
    .header("User-Agent", "emmVRC/1.0 (Client; emmVRCClient/2.3.0)")
    .header("Content-Type", "application/json; charset=utf-8")
    .header("Content-Length", body.len())
    .body(body)
    .send()
    .await {
        Ok(val) => {
            match val.status().as_u16() {
                401 => 0,
                200 => 1,
                v => v,
            }
        },
        Err(v) => { println!("{}", v); 2 },
    }
}
