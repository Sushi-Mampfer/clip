use std::{
    io::{self, IsTerminal, Read, Write},
    process::exit,
};

use clap::{Parser, builder::Str};
use reqwest::blocking::{self, Client};

#[derive(Parser)]
#[command(
    version,
    about = "A very simple cli tool to use the clipboard from terminal, pipe into it to store and run without any args to get the value."
)]
struct Cli {
    #[arg(short, long, help = "If set a link to the clipboard will be created.")]
    share: bool,
    #[arg(help = "Value to set the clipboard to")]
    value: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let mut clipboard = clippers::Clipboard::get();
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    match cli.value {
        Some(data) => {
            clipboard.write_text(data.clone()).unwrap();
            if cli.share {
                create_link(data);
            }
            exit(0)
        }
        _ => (),
    }

    if stdin.is_terminal() {
        match clipboard.read() {
            Some(data) => match data.as_text() {
                Some(text) => {
                    if cli.share {
                        create_link(text.to_string());
                    }
                    stdout.write_all(text.as_bytes()).unwrap()
                }
                None => stderr.write_all(b"No string in clipboard").unwrap(),
            },
            None => (),
        }
        exit(0)
    }
    let mut data = String::new();
    stdin.read_to_string(&mut data).unwrap();
    clipboard.write_text(data.clone()).unwrap();
    if cli.share {
        create_link(data);
    }
}

fn create_link(data: String) {
    let params = [("content", data), ("privat", "false".to_string())];
    let client = Client::new();
    let res = match client
        .post("https://clip.tim.hackclub.app/create")
        .json(&params)
        .send()
    {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{}", e);
            exit(1)
        }
    };
    if res.status().is_success() {
        println!("https://clip.tim.hackclub.app/{}", res.text().unwrap());
        exit(0)
    } else {
        eprintln!("Request failed with status code {}.", res.status().as_str());
        exit(1)
    }
}
