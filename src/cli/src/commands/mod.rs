pub mod topic;

use clap::ArgMatches;

pub fn handle(matches: ArgMatches) {
    match matches.subcommand() {
        Some(("topic", submatch)) => {
            topic::handle(submatch.clone());
        }
        _ => unreachable!("UNREACHABLE"),
    };
}
