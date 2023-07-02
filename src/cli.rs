use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long = "conf")]
    conf_file_path: String,
}

pub fn handle() -> String {
    let args = Args::parse();
    println!("Loading configuration file : {}", args.conf_file_path);
    args.conf_file_path
}
