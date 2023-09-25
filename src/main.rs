use anyhow::Result;
use dialoguer::console::Term;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use std::process::{self, Command};

mod bulk_download;
use crate::bulk_download::bulk_download;

mod single_download;
use crate::single_download::single_download;

fn main() -> Result<()> {
    let choices = vec!["Single Download", "Multi Download", "Get versions", "Exit"];

    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What do you want to do ?")
            .items(&choices)
            .default(0)
            .interact_on_opt(&Term::stderr())?
            .expect("Choose something");

        match selection {
            0 => {
                let dl_type = ask_dl_type().unwrap();
                single_download(dl_type).unwrap()
            }
            1 => {
                let dl_type = ask_dl_type().unwrap();
                bulk_download(dl_type).unwrap()
            }
            2 => show_information(),
            _ => break,
        };
    }

    Ok(())
}

#[derive(PartialEq, Clone, Copy)]
pub enum DownloadType {
    Video,
    Audio,
}

pub fn ask_dl_type() -> Result<DownloadType> {
    let type_choices = vec!["Audio", "Video"];
    let dl_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which type do you want ?")
        .items(&type_choices)
        .default(0)
        .interact_on_opt(&Term::stderr())?
        .expect("Choose something");

    match dl_type {
        0 => Ok(DownloadType::Audio),
        _ => Ok(DownloadType::Video),
    }
}

fn get_version() -> String {
    let command = Command::new("yt-dlp")
        .args(["--version"])
        .output()
        .expect("Failed to exec process.");
    let out = String::from_utf8_lossy(&command.stdout);

    return out.trim().to_owned();
}

fn show_information() {
    let ytdlp = format!("Ytdlp version: {}", get_version());
    let version = format!("Program version: {}", env!("CARGO_PKG_VERSION"));
    println!("{}\n{}\n", version, ytdlp);

    process::exit(0);
}
