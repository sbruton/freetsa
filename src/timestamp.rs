use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use freetsa::TimestampResponse;
use tokio::{
    fs::{create_dir_all, OpenOptions},
    io::AsyncWriteExt,
};

#[derive(Parser)]
pub(super) struct FileArgs {
    #[clap(long = "data")]
    data_path: PathBuf,
    #[clap(long = "query-out")]
    query_path: PathBuf,
    #[clap(long = "reply-out")]
    reply_path: PathBuf,
}

#[derive(Subcommand)]
pub(super) enum Command {
    File(FileArgs),
}

#[derive(Parser)]
pub(super) struct Args {
    #[clap(subcommand)]
    command: Command,
}

pub(super) async fn exec(args: Args) {
    match args.command {
        Command::File(file_args) => exec_file(file_args).await,
    }
}

async fn exec_file(args: FileArgs) {
    let TimestampResponse { query, reply } =
        freetsa::timestamp_file(&args.data_path).await.unwrap();
    save(&args.query_path, &query).await.unwrap();
    save(&args.reply_path, &reply).await.unwrap();
}

async fn save(path: &Path, data: &[u8]) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        if parent.is_dir() {
            create_dir_all(parent).await?;
        }
    }
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .await?;
    file.write_all(data).await?;
    file.flush().await?;
    Ok(())
}
