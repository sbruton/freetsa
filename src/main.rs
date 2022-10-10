use clap::{Parser, Subcommand};

mod timestamp;

#[derive(Subcommand)]
enum Command {
    Timestamp(timestamp::Args),
}

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let args = Args::parse();
    match args.command {
        Command::Timestamp(timestamp_args) => timestamp::exec(timestamp_args).await,
    }
}
