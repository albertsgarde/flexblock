use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Args {
    #[clap(short, long, default_value = "15926")]
    pub port: String,

    #[clap(long)]
    pub ip: Option<String>,
}
