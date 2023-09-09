use std::{
    fs::File,
    io::{self, BufRead, Write},
    path::Path,
    process::Command,
};

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

    /// Force writing mode
    #[arg(short, long, conflicts_with = "read")]
    write: bool,

    /// Force reading mode
    #[arg(short, long)]
    read: bool,

    /// If reading, read one line and exit (useful for event loops)
    #[arg(short, long)]
    one_line: bool,

    /// If reading, don't exit when all writers disconnect
    #[arg(short, long)]
    ignore_disconnects: bool,

    /// Name of the channel to use, defaults to the id of the parent process
    channel: Option<String>,
}

fn main() -> Result<()> {
    // variable declaration ////////
    let dirs = BaseDirs::new().context("couldn't find valid home directory")?;
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
    //////// variable declaration //

    if args.write || !args.read && atty::is(Stdout) {
        // portal writer ////////
        if args.verbose {
            eprintln!("portal [writer]: using channel {channel}");
        }

        //? if the channel fifo doesn't exist yet, create it
        if !channel_path.exists() {
            unix_named_pipe::create(&channel_path, None).context(format!(
                "failed to create channel {}",
                channel_path.display()
            ))?;
        }

        //? loop until a reader has connected to the same channel
        //NOTE: this allows you to spawn a writer first and start buffering
        //      input without it crashing
        let mut channel;
        loop {
            match unix_named_pipe::open_write(&channel_path) {
                Err(e) => {
                    if e.raw_os_error() != Some(6) {
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

            //? poll at <=1000Hz to avoid wasting too many resources
            std::thread::sleep(std::time::Duration::from_millis(1));
        }

        //? write data from stdin into the channel
        for line in std::io::stdin().lines().map(|l| l.unwrap()) {
            channel
                .write_all((line + "\n").as_bytes())
                .context("broken pipe")?;
        }

        std::fs::remove_file(&channel_path).context("failed to remove old channel")?;
        //////// portal writer //
    } else {
        // portal reader ////////
        if args.verbose {
            eprintln!("portal [reader]: using channel {channel}");
        }

        loop {
            //? loop until a writer creates the channel
            while !channel_path.exists() {
                //? poll at <=1000Hz to avoid wasting too many resources
                std::thread::sleep(std::time::Duration::from_millis(1));
            }

            let channel = File::open(&channel_path).context("failed to open file for reading")?;
            let mut lines = io::BufReader::new(channel).lines().flatten();

            if args.one_line {
                //? read one line from the channel
                if let Some(l) = lines.next() {
                    println!("{l}");
                    break;
                }
            } else {
                //? read from the channel until all writers exit
                for line in lines {
                    println!("{line}");
                }
            }

            if !args.ignore_disconnects {
                break;
            }
        }
        //////// portal reader //
    }

    Ok(())
}
