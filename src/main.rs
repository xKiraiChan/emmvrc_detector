use futures::{stream, StreamExt};
use reqwest::Client;
use std::io::Write;
use text_io::read;

#[tokio::main]
async fn main() {
    eprintln!("emmVRC Detecter by xKiraiChan");

    if std::env::args().count() > 1 {
        let client = Client::new();
        
        stream::iter(std::env::args().skip(1))
            .map(|str| {
                let cli = &client;

                async move {
                    let res = emmvrc_detector::check_id(&cli, str.clone()).await;
                
                    match res {
                        0 => println!("\x1b[32m[+] {}\x1b[0m", &str),
                        1 => println!("\x1b[31m[-] {}\x1b[0m", &str),
                        2 => println!("\x1b[36mInternal error\x1b[0m"),
                        v => println!("\x1b[36mFailed with code {}\x1b[0m", v),
                    };
                }
            })
            .buffer_unordered(24)
            .for_each(|_| async {

            }).await;
    } else {
        eprint!("User ID to check: ");
        std::io::stdout().flush().expect("Failed to flush stdout");
        let id: String = read!();
            
        match emmvrc_detector::check_id(&Client::new(), id.to_string()).await {
            0 => println!("\n\x1b[32m{} has used emmVRC before\x1b[0m\n", id),
            1 => println!("\n\x1b[31m{} has probably not used emmVRC before\x1b[0m\n", id),
            2 => println!("\n\x1b[36mInternal error\x1b[0m\n"),
            v => println!("\n\x1b[36mFailed with code {}\x1b[0m\n", v),
        };
    }

    print!("Press enter or CTRL-C to close... ");
    std::io::stdout().flush().expect("Failed to flush stdout");

    let _: String = read!();

    std::process::exit(0);
}
