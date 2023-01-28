use std::env;
use std::path::PathBuf;
use std::process::exit;
use structopt::StructOpt;

mod config;
mod convert;
mod make;

static VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(StructOpt, Debug)]
#[structopt(name = "icon", version = VERSION)]
struct Opt {
    #[structopt(subcommand)]
    sub_command: SubCommand,

    #[structopt(short = "V", long = "version")]
    version: bool,
}

#[derive(StructOpt, Debug)]
enum SubCommand {
    #[structopt(name = "make")]
    Make {
        #[structopt(name = "icon-name")]
        icon_name: String,
        #[structopt(name = "url | path")]
        url_or_path: String,
    },
    #[structopt(name = "conv")]
    Conv {
        #[structopt(name = "icon-name")]
        icon_name: String,
        #[structopt(name = "path")]
        path: String,
    },
    #[structopt(name = "conv-all")]
    ConvAll {
        #[structopt(name = "directory-path")]
        directory_path: String,
    },
    #[structopt(name = "help")]
    Help,
    #[structopt(name = "init")]
    Init,
    #[structopt(name = "update")]
    Update,
    #[structopt(name = "component")]
    Component {
        #[structopt(name = "icon-name")]
        icon_name: String,
        #[structopt(name = "type")]
        type_: String,
        #[structopt(short = "o", long = "output", name = "output-path")]
        output_path: Option<String>,
    },
}

fn main() {
    let opt = Opt::from_args();
    match opt.sub_command {
        SubCommand::Make {
            icon_name,
            url_or_path,
        } => {
            let result = make::make(&icon_name, &url_or_path);
            if let Err(e) = result {
                eprintln!("Error: {}", e);
                exit(1);
            }
        }
        SubCommand::Conv { icon_name, path } => {
            let result = convert::convert(&icon_name, &PathBuf::from(&path));
            if let Err(e) = result {
                eprintln!("Error: {}", e);
                exit(1);
            }
        }
        SubCommand::ConvAll { directory_path } => {
            convert::convert_all(&PathBuf::from(&directory_path));
        }
        SubCommand::Help => println!("Displaying help message"),
        SubCommand::Init => config::setup(),
        SubCommand::Update => config::update(),
        SubCommand::Component {
            icon_name,
            type_,
            output_path,
        } => {
            println!("Making component of {} icon with type {}", icon_name, type_);
            if let Some(path) = output_path {
                println!("Output path: {}", path);
            }
        }
    }
    if opt.version {
        println!("{}", VERSION);
        exit(0);
    }
}
