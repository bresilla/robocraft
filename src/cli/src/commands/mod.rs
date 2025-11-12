pub mod topic;
pub mod launch;

use clap::ArgMatches;

pub fn handle(matches: ArgMatches) {
    match matches.subcommand() {
        Some(("launch", _)) => {
            launch::handle();
        }
        Some(("topic", submatch)) => {
            topic::handle(submatch.clone());
        }
        _ => unreachable!("UNREACHABLE"),
    };
}
