pub mod topic;

use clap::{Command, builder::styling, arg};
use colored::Colorize;

pub fn letter_str(letter: &str) -> String {
    let mut wrapped = "[".bright_green().to_string();
    wrapped.push_str(&letter.bright_green().italic().to_string());
    wrapped.push_str(&"]".bright_green().to_string());
    wrapped.push_str(&"  ".to_string());
    wrapped
}

pub fn command_str(word: &str) -> String {
    word.bright_green().bold().to_string()
}

pub fn descriptin_str(word: &str) -> String {
    word.bright_white().to_string()
}

const ABOUT_STR: &str = "general-purpose robotics building blocks framework";

pub fn cli(logo: bool) -> Command {
    let _logo_1: String ="
                 █████                 
       ███     ██▓▓▓▓▓██     ███       
     ███▓███████▓▓▓▓▓▓▓███████▓███     
    ███▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓███    
     ██▓▓▓█▀▀█▓▓▓▓▓▓▓▓▓▓▓█▀▀█▓▓▓██      
      ███  ▄▄  ██▓▓▓▓▓██  ▄▄  ███      
  ▄█████  ████  █▓▓▓▓▓█  ████  █████▄  
 ██▓▓▓▓█▄ ▀██▀ ▄█▓▓▓▓▓█▄ ▀██▀ ▄█▓▓▓▓██ 
 █▓▓▓▓▓▓█▄ ▀▀ ▄█▓▓▓▓▓▓▓█▄ ▀▀ ▄█▓▓▓▓▓▓█ 
▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀ 
      ████                   ████       
       █▓▓██               ██▓▓█        
      ██▓▓▓▓███▄▄     ▄▄███▓▓▓▓██      
     ███▓▓███████████████████▓▓███     
       ███    ███▓▓▓▓▓███    ███       
                 ▀███▀    \n".bright_red().to_string();

    let logo_str: String = if logo {_logo_1 } else { String::new() };

    let help_str: String = " ".to_string().to_owned()+"
Usage:".bright_blue().bold().to_string().as_str()+"  rc".bright_green().bold().to_string().as_str()+" <COMMAND>".green().to_string().as_str()+"
      ".bright_blue().bold().to_string().as_str()+"  rc".bright_green().bold().to_string().as_str()+" <C>".green().to_string().as_str()+"

Utilities Commands:".bright_blue().bold().to_string().as_str()+"
  "+ &command_str("deamon") + "      "+&letter_str("d")+ &descriptin_str("Control the deamon process")  + "
  "+ &command_str("supervisor") + "  "+&letter_str("s")+ &descriptin_str("Run supervision tasks")  + "
  "+ &command_str("workspace") + "   "+&letter_str("w")+ &descriptin_str("Manage workspaces")  + "
  "+ &command_str("module") + "      "+&letter_str("m")+ &descriptin_str("Create modules")  + "
  ";

    let styles = styling::Styles::styled()
        .header(styling::AnsiColor::Blue.on_default() | styling::Effects::BOLD)
        .usage(styling::AnsiColor::Blue.on_default() | styling::Effects::BOLD)
        .literal(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .error(styling::AnsiColor::Red.on_default() | styling::Effects::BOLD)
        .placeholder(styling::AnsiColor::Green.on_default());

    Command::new("rc")
        .styles(styles)
        .about(&ABOUT_STR) 
        .author("bresilla <trim.bresilla@gmail.com>")
        .version("0.0.1")
        .long_about("RoboCraft is a cutting-edge robotics framework engineered to expedite the creation, deployment, and management of robotic systems")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(false)
        .disable_help_subcommand(true)
        .override_help(logo_str + &help_str)
        .subcommand(topic::cmd())
        .arg(arg!(--about "about"))
}
