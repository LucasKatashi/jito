<p align="center">
<h1 align="center"><b>Project Jito</b></h1>
<p align="center"><b>A Rust-based Message Redirection Tool for Discord and Slack Webhooks</b></p>

---
### Summary

Project Jito is a tool crafted in Rust, designed to facilitate the redirection of messages from standard input (stdin) to webhooks on platforms like Discord and Slack.

This tool serves as a versatile intermediary, seamlessly transferring messages from various sources to designated channels on Discord or Slack, enhancing communication and integration within teams and communities.

Whether used for real-time notifications, automated alerts, or collaborative messaging, Project Jito offers a flexible and scalable solution for enhancing communication dynamics across Discord and Slack platforms.
---
### Installation
- You need to have `cargo` installed: https://rustup.rs
- Go to the `jito` directory and run:
    ```sh
    cargo run
    ```
- The `jito` binary will be generated in `target/debug`

---
### Usage
```sh
echo "Hello World!" | ./jito --output discord
```

If the `webhooks.yaml` file is not in the same directory from which you are running the script, you can specify the file with the `--file` argument.

---
### License

This work is licensed under [MIT License.](/LICENSE.md)
