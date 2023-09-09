use std::{io::Write, process::Command};

use anyhow::{Context, Result};
use atty::Stream::Stdout;
use clap::Parser;
use directories::BaseDirs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Increase verbosity
    #[arg(short, long)]
    verbose: bool,

    /// Name of the channel to use, defaults to the id of the parent process
    channel: Option<String>,
}

fn main() -> Result<()> {
    let dirs = BaseDirs::new().context("couldn't find valid runtime directory")?;
    let runtime_dir = dirs
        .runtime_dir()
        .context("couldn't find valid runtime directory")?;

    let channels_dir = runtime_dir.join("pl-portal");

    if !channels_dir.exists() {
        std::fs::create_dir(channels_dir.clone()).context("failed to create channels directory")?;
    }

    let parent_id = std::os::unix::process::parent_id();

    let args = Args::parse();
    let channel = args.channel.unwrap_or(parent_id.to_string());
    let channel_path = channels_dir.join(&channel);

    if atty::is(Stdout) {
        if args.verbose {
            eprintln!("portal [writer]: using channel {channel}");
        }

        if !channel_path.exists() {
            unix_named_pipe::create(&channel_path, None).context(format!(
                "failed to create channel {}",
                channel_path.display()
            ))?;
        }

        let mut channel;
        loop {
            match unix_named_pipe::open_write(&channel_path) {
                Err(e) => {
                    if e.raw_os_error() == Some(6) {
                        continue;
                    } else {
                        Result::Err(e).context(format!(
                            "failed to open {} for writing",
                            channel_path.display()
                        ))?;
                    }
                }
                Ok(v) => {
                    channel = v;
                    break;
                }
            }
        }

        for line in std::io::stdin().lines().map(|l| l.unwrap()) {
            channel
                .write_all((line + "\n").as_bytes())
                .context("broken pipe")?;
        }

        std::fs::remove_file(&channel_path).context("failed to remove old channel")?;
    } else {
        if args.verbose {
            eprintln!("portal [reader]: using channel {channel}");
        }

        loop {
            if channel_path.exists() {
                break;
            }
        }

        // i'm too high to know how to do this properly but this is fine i swear
        //
        // update: i'm sober and still don't know how to do this properly lmao
        let mut process = Command::new("cat");
        process
            .arg(&channel_path)
            .spawn()
            .context("subprocess exited unsuccessfully")?;
    }

    Ok(())
}
