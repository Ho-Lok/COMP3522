mod add_columns;
mod change_once;
mod combine;
mod no_change;

use clap::{arg, Parser};

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    module: String,
}

fn main() {
    let args = Args::parse();
    match args.module.as_str() {
        "no_change" => no_change::main(),
        "add_columns" => add_columns::main(),
        "change_once" => change_once::main(),
        "combine" => combine::main(),
        _ => {}
    }
}
