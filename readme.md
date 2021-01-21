# emmvrc_detector

**This project will probably break in the future**

Simple project written in [Rust](https://www.rust-lang.org/) to detect if a user has ever used [emmVRC](https://thetrueyoshifan.com/emmvrc.php).

This project works by sending a request to the emmVRC servers. 
Based on the status code we can determine if the user has a password (pin) set.

The username field seems to be ignored except for when it is empty, and I haven't figured out what "loginKey" is used for.

# Crates used

For getting user input: [text_io](https://crates.io/crates/text_io)

For sending HTTP requests: [reqwest](https://crates.io/crates/reqwest)

For using async: [tokio](https://crates.io/crates/tokio)
