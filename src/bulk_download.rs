use anyhow::Result;
use dialoguer::console::Term;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Select};
use home::home_dir;
use std::path::Path;
use std::process::Command;
use std::thread;

use crate::DownloadType;

pub fn bulk_download(dl_type: DownloadType) -> Result<()> {
    let mut format = String::new();
    if dl_type == DownloadType::Video {
        let quality_choices = vec!["360p", "480p", "720p", "1080p"];
        let quality = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Which quality do you want ? (This format will be used for each video)")
            .items(&quality_choices)
            .default(0)
            .interact_on_opt(&Term::stderr())?
            .expect("Choose something");
        format = format!(
            "best[height<={}]",
            quality_choices[quality].replace("p", "")
        );
    }

    let default_location = Path::new(&home_dir().unwrap()).join("Downloads");
    let location: String = Input::new()
        .with_prompt("Enter the location where you want to save your downloads")
        .default(default_location.to_str().unwrap().to_string())
        .interact_text()?;

    let urls_bulk: String = Input::new()
        .with_prompt("Enter the urls of the videos (separated by a comma)")
        .interact_text()?;

    let urls: Vec<String> = urls_bulk.split(",").map(|e| e.trim().to_string()).collect();

    let starting_time = std::time::Instant::now();
    let mut handles = Vec::new();
    for url in urls {
        let loc = location.clone();
        let fmt = format.clone();

        let handle = thread::spawn(move || {
            let mut command = Command::new("yt-dlp");
            command
                .current_dir(loc)
                .arg("-O")
                .arg("title")
                .arg("--no-simulate");

            match dl_type {
                DownloadType::Video => {
                    command.arg("-f").arg(fmt).arg(&url);
                }
                DownloadType::Audio => {
                    command.arg("-x").arg("--audio-format").arg("mp3").arg(&url);
                }
            }

            let out = command.output().expect("Failed to exec process.");

            (out, url)
        });
        handles.push(handle);
    }

    println!();
    let mut success = 0;
    for handle in handles {
        let val = handle.join().expect("Thread panicked.");
        let out = val.0;
        let url = val.1;

        match out.status {
            code if code.success() => {
                success += 1;
                println!(
                    "{}: '{}'",
                    console::style("Downloaded successfully").green().bold(),
                    String::from_utf8_lossy(&out.stdout).trim()
                )
            }
            _ => {
                println!(
                    "{}: '{}'\n{}",
                    console::style("Failed to download").red().bold(),
                    url,
                    String::from_utf8_lossy(&out.stderr)
                );
            }
        }
    }
    println!(
        "\n🚀 {} vids downloaded in {}s\n",
        success,
        starting_time.elapsed().as_secs_f32()
    );

    Ok(())
}
