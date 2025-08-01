use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about="Convert spreadsheets register maps to IP-XACT XML files.", long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub input: String,

    #[arg(short, long)]
    pub output: String,
}
