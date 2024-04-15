pub mod echo;

use clap::ArgMatches;

pub fn handle(matches: ArgMatches) {
    match matches.subcommand() {
        Some(("echo", args)) => {
            echo::handle(args.clone());
        }
        _ => unreachable!("UNREACHABLE"),
    }
}
