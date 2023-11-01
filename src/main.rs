use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;

use documentation_as_code_gxp::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
}

fn main() -> ExitCode {
    let args = Args::parse();
    let project = PathBuf::from(args.path);

    let documents = match get_documents(project) {
        Ok(documents) => documents,
        Err(Error(errors)) => {
            for error in errors {
                eprintln!("ERROR: {error}");
            }
            return ExitCode::FAILURE;
        }
    };

    let result = serde_json::to_string_pretty(&documents).unwrap();

    println!("{result}");

    ExitCode::SUCCESS
}
