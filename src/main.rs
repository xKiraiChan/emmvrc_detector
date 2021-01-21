use std::io::Write;
use text_io::read;

#[tokio::main]
async fn main() {
    println!("emmVRC Detecter by xKiraiChan");

    print!("User ID to check: ");
    std::io::stdout().flush().expect("Failed to flush stdout");
    let uid: String = read!();

    if uid.len() == 10 {
        println!("Checking Classic ID...")
    } else if uid.len() == 40 {
        println!("Checking GUID...")
    } else {
        println!("Invalid UserID");
        quit();
    }

    let client = reqwest::Client::new();

    let body = format!(r#"{{"username":"{}","name":"tupper","password":"{}","loginKey":"1"}}"#, uid, uid);

    match client
    .post("https://thetrueyoshifan.com:3000/api/authentication/login")
    .header("User-Agent", "emmVRC/1.0 (Client; emmVRCClient/2.3.0)")
    .header("Content-Type", "application/json; charset=utf-8")
    .header("Content-Length", body.len())
    .body(body)
    .send()
    .await {
        Ok(val) => {
            let code: u16 = val.status().as_u16();

            if code == 401 {
                println!("\n\x1b[32m{} has used emmVRC before\x1b[0m\n", uid);
            } else if code == 200 {
                println!("\n\x1b[31m{} has probably not used emmVRC before\x1b[0m\n", uid);
            } else {
                println!("\n\x1b[36mFailed to detect user due to {}\x1b[0m\n", val.status());
            }
        },
        Err(val) => println!("Failed due to {}", val),
    };

    quit();
}

fn quit() {
    print!("Press enter or CTRL-C to close... ");
    std::io::stdout().flush().expect("Failed to flush stdout");

    let _: String = read!();

    std::process::exit(0);
}
