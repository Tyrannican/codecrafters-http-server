use clap::Parser;

#[derive(Debug, Parser)]
pub(crate) struct Cli {
    #[arg(short, long)]
    pub(crate) directory: Option<String>,
}
