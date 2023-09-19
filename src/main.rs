use anyhow::Result;
use dialoguer::console::Term;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Select};
use home::home_dir;
use std::path::Path;
use std::process::{self, Command};

fn main() -> Result<()> {
    let choices = vec!["Video", "Audio", "Get versions", "Exit"];

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
            2 => show_information(),
            _ => break,
        };
    }

    Ok(())
}

fn audio() -> Result<()> {
    let url: String = Input::new()
        .with_prompt("Enter the url of the video")
        .interact_text()?;

    let default_location = Path::new(&home_dir().unwrap()).join("Downloads");
    let location: String = Input::new()
        .with_prompt("Enter the location where you want to save the file")
        .default(default_location.to_str().unwrap().to_string())
        .interact_text()?;

    let mut command = Command::new("yt-dlp");

    command
        .current_dir(&location)
        .arg("-x")
        .arg("--audio-format")
        .arg("mp3")
        .arg("-O")
        .arg("id,title,filesize,filesize_approx,duration_string")
        .arg("--no-simulate")
        .arg(url);

    download(&mut command, &location, None);

    Ok(())
}

fn video() -> Result<()> {
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
        .current_dir(&location)
        .arg("-f")
        .arg(format)
        .arg("-O")
        .arg("id,title,filesize,filesize_approx,duration_string")
        .arg("--no-simulate")
        .arg(url);

    download(&mut command, &location, Some(choices[quality]));

    Ok(())
}

fn download(command: &mut Command, location: &str, quality: Option<&str>) {
    let output = command.output();

    match output {
        Ok(output) => {
            let out = String::from_utf8_lossy(&output.stdout);
            let informations: Vec<&str> = out.split("\n").collect();
            println!(
                "\n{}\n\n{}\nYoutube ID: {}\nName: '{}'\nFile size: {}MB\nLocation: {}\n{}Length: {}\n",
                console::style("Downloaded successfully").green().bold(),
                console::style("Informations").bold(),
                informations[0],
                informations[1],
                format!(
                    "{:.1$}",
                    informations[2].parse::<f64>().unwrap_or(informations[3].parse::<f64>().unwrap_or(0.0)) / 1000000.0,
                    1
                ),
                location,
                match quality {
                    Some(quality) => format!("Definition: {}\n", quality),
                    None => String::new(),
                },
                informations[4]
            )
        }
        Err(e) => println!(
            "\n {}\n{}",
            console::style("Failed to download").red().bold(),
            e
        ),
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
