use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about="Convert spreadsheets register maps to IP-XACT XML files.", long_about = None)]
pub struct Args {
    /// Path to the input excel file.
    #[arg(short, long)]
    pub input: String,

    /// Path for the output XML file.
    #[arg(short, long)]
    pub output: Option<String>,

    #[clap(long)]
    pub regvue: bool,
}
