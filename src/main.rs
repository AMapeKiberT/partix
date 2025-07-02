mod cli;

use std::env;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        cli::repl_loop(Some(args[1].clone()))
    } else {
        cli::repl_loop(None)
    }
}

