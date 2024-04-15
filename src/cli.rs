use clap::Parser;

use crate::utils::version;

#[derive(Parser, Debug)]
#[command(author, version = version(), about)]
pub struct Cli {
    #[arg(
        short,
        long,
        value_name = "FLOAT",
        help = "Tick rate, i.e. number of ticks per second",
        default_value_t = 1.0
    )]
    pub tick_rate: f64,

    #[arg(
        short,
        long,
        value_name = "FLOAT",
        help = "Frame rate, i.e. number of frames per second",
        default_value_t = 4.0
    )]
    pub frame_rate: f64,
}

// use clap::{Parser, Subcommand};

// #[derive(Parser, Debug)]
// #[command(author, version, about = "Utility to manage installations and updates")]
// pub struct Cli {
//     #[command(subcommand)]
//     pub command: Commands,
// }
//
// #[derive(Subcommand, Debug)]
// pub enum Commands {
//     /// Updates the Neovim installation to the latest version
//     UpdateNeovim {
//         #[arg(short, long, help = "Specify a version to update to, if not the latest")]
//         version: Option<String>,
//     },
//
//     /// Updates the NEVIRAIDE installation to the latest version
//     UpdateNeviraide {
//         #[arg(short, long, help = "Specify a version to update to, if not the latest")]
//         version: Option<String>,
//     },
//
//     /// Checks for missing dependencies required by the system
//     CheckDependencies,
// }
