use anyhow::Result;
use dialoguer::console::Term;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Select};
use home::home_dir;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    dbg!(Path::new(&home_dir().unwrap()).join("Downloads"));

    let choices = vec!["Video", "Audio", "Exit"];

    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What do you want to do ?")
            .items(&choices)
            .default(0)
            .interact_on_opt(&Term::stderr())?
            .expect("Choose something");

        match selection {
            0 => video().unwrap(),
            1 => audio().unwrap(),
            _ => break,
        };
    }

    Ok(())
}

fn audio() -> anyhow::Result<()> {
    let url: String = Input::new()
        .with_prompt("Enter the url of the video")
        .interact_text()?;

    let default_location = Path::new(&home_dir().unwrap()).join("Downloads");
    let location: String = Input::new()
        .with_prompt("Enter the location where you want to save the video")
        .default(default_location.to_str().unwrap().to_string())
        .interact_text()?;

    let mut command = Command::new("yt-dlp");

    command
        .current_dir(location)
        .arg("-x")
        .arg("--audio-format")
        .arg("mp3")
        .arg(url);

    download(&mut command);

    Ok(())
}

fn video() -> anyhow::Result<()> {
    let choices = vec!["360p", "480p", "720p", "1080p"];

    let quality = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which quality do you want ?")
        .items(&choices)
        .default(0)
        .interact_on_opt(&Term::stderr())?
        .expect("Choose something");
    let format = format!("best[height<={}]", choices[quality].replace("p", ""));

    let url: String = Input::new()
        .with_prompt("Enter the url of the video")
        .interact_text()?;

    let default_location = Path::new(&home_dir().unwrap()).join("Downloads");
    let location: String = Input::new()
        .with_prompt("Enter the location where you want to save the video")
        .default(default_location.to_str().unwrap().to_string())
        .interact_text()?;

    let mut command = Command::new("yt-dlp");

    command
        .current_dir(location)
        .arg("-i")
        .arg("-f")
        .arg(format)
        .arg(url);

    download(&mut command);

    Ok(())
}

fn download(command: &mut Command) {
    let output = command.output();

    match output {
        Ok(_) => println!(
            "\n {}",
            console::style("Downloaded successfully").green().bold()
        ),
        Err(e) => println!(
            "\n {}\n{}",
            console::style("Failed to download").red().bold(),
            e
        ),
    }

    thread::sleep(Duration::from_millis(2000));
    std::process::Command::new("clear").status().unwrap();
}

#[allow(dead_code)]
fn get_version() -> String {
    let command = Command::new("yt-dlp")
        .args(["--version"])
        .output()
        .expect("Failed to exec process.");
    let out = String::from_utf8_lossy(&command.stdout);

    return out.trim().to_owned();
}
