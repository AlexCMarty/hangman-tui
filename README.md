# Hangman 🦀

This is a simple hangman game I've written in Rust. It runs in the terminal. I taught myself some Rust in making this program. I've built it for Linux, Windows, and MacOS.

## Installation

First, go to the "Releases" section on the right side to download the file for your OS.

### 🐧 Linux x64

1. Download .AppImage
2. `chmod +x hangman-x86_64.AppImage`
3. `./hangman-x86_64.AppImage`

### 🪟 Windows x64 

1. Run the file. SmartScreen will block you.
2. More info > Run anyway.

### 🍎 MacOS

1. Try running it. You will get an error.
2. Go to System Settings > Privacy & Security.  
3. ​Scroll down to the Security section.
4. ​You should see a message about your app being blocked. Click Open Anyway.
5. Run again.

### Build from source (easy)
1. Check the Rust documentation to [install cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html). For Linux and MacOS, run `curl https://sh.rustup.rs -sSf | sh`. For Windows run `rustup-init.exe`.
2. Run `cargo install --git https://github.com/AlexCMarty/hangman-tui --branch main`. This will install it to your PATH.
3. Now run `hangman`. You might have to reopen the terminal to refresh your PATH.
4. 🦀`
