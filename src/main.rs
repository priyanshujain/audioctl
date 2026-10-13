mod commands;

use clap::{Parser, Subcommand};
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(name = "audioctl")]
#[command(about = "Virtual audio devices and an echo-cancelling bridge for macOS")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    /// Increase verbosity (-v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    verbose: u8,
}

#[derive(Subcommand)]
enum Command {
    /// Install the virtual audio devices (asks for sudo, restarts coreaudiod)
    Install,

    /// Remove the virtual audio devices (asks for sudo, restarts coreaudiod)
    Uninstall,

    /// Show whether the devices are installed and loaded
    Status,

    /// List the real devices the bridge can use, by UID
    Devices,

    /// Run the echo-cancelling bridge until interrupted
    Bridge {
        /// UID of the real microphone
        #[arg(short, long)]
        input: String,

        /// UID of the real speaker
        #[arg(short, long)]
        output: String,

        /// Stop after this many seconds instead of running until interrupted
        #[arg(long)]
        seconds: Option<u64>,

        /// Suppress background noise on the microphone with RNNoise
        #[arg(long)]
        denoise: bool,

        /// Fade the microphone out below this voice probability, 0 to 1
        #[arg(long, default_value_t = 0.0)]
        voice_threshold: f32,
    },

    /// Measure the real round trip between a speaker and a microphone, out loud
    Calibrate {
        /// UID of the real microphone
        #[arg(short, long)]
        input: String,

        /// UID of the real speaker
        #[arg(short, long)]
        output: String,

        /// How many times to play the probe
        #[arg(long)]
        probes: Option<usize>,

        /// Measure and report without saving the correction
        #[arg(short = 'n', long)]
        dry_run: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let filter = match cli.verbose {
        0 => "warn",
        1 => "info",
        2 => "debug",
        _ => "trace",
    };
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(filter))
        .without_time()
        .init();

    match cli.command {
        Command::Install => commands::install(),
        Command::Uninstall => commands::uninstall(),
        Command::Status => commands::status(),
        Command::Devices => commands::devices(),
        Command::Bridge {
            input,
            output,
            seconds,
            denoise,
            voice_threshold,
        } => commands::bridge(&input, &output, seconds, denoise, voice_threshold),
        Command::Calibrate {
            input,
            output,
            probes,
            dry_run,
        } => commands::calibrate(&input, &output, probes, dry_run),
    }
}
