# No longer possible
The emmVRC API always returns "invalid combination", regardless of whether or not an account exists.

# emmvrc_detector

***.bunny. did not contribute to this project in any way. Abyss client ($100) sells a public GitHub feature.***

**This project will probably break in the future**

Simple project written in [Rust](https://www.rust-lang.org/) to detect if a user has ever used [emmVRC](https://thetrueyoshifan.com/emmvrc.php).

This project works by sending a request to the emmVRC servers. 
Based on the status code we can determine if the user has a password (pin) set.

The username field seems to be ignored except for when it is empty, and "loginKey" is effectively ignored due to this being a single sign on attempt.

# Crates used

For getting user input: [text_io](https://crates.io/crates/text_io)

For sending HTTP requests: [reqwest](https://crates.io/crates/reqwest)

For using async: [tokio](https://crates.io/crates/tokio)
