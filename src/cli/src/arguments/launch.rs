use clap::Command;

pub fn cmd() -> Command {
    Command::new("launch")
        .alias("l")
        .about("Launch the GUI application")
        .long_about("Launches the Robocraft graphical user interface")
}
