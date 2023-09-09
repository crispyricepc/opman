mod database;
mod display;
mod ops;
mod package;

pub use database::Database;
use display::{print_packages, print_summary};
use ops::Opman;
pub use package::Package;

use clap::{command, Parser, Subcommand};
use fern::colors::{Color, ColoredLevelConfig};

/// Opman - Opinionated Package Manager for ArchLinux
///
/// todo: long about
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Action,
}

#[derive(Subcommand)]
enum Action {
    /// Summarize the given packages
    Summary { packages: Vec<String> },
    /// Get the given packages' dependencies
    Dependencies { packages: Vec<String> },
    /// Search for packages
    Search { keywords: Vec<String> },
    /// Install a package
    Install { packages: Vec<String> },
}

fn build_logger() -> Result<(), fern::InitError> {
    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .trace(Color::Yellow)
        .debug(Color::Blue)
        .warn(Color::Yellow);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}] [{}] {}",
                chrono::Local::now().format("%H:%M:%S"),
                colors.color(record.level()),
                message
            ))
        })
        .chain(std::io::stderr())
        .apply()?;
    Ok(())
}

fn main() {
    let opman = Opman::new();

    // Initialize logger
    build_logger().unwrap();

    let cli = Cli::parse();

    match cli.command {
        Action::Summary { packages } => {
            print_summary(opman.summary(packages).unwrap());
        }
        Action::Dependencies { packages } => {
            print_packages(
                opman
                    .dependencies(&packages)
                    .unwrap()
                    .values()
                    .filter_map(|pkg| pkg.as_ref()),
                false,
            );
        }
        Action::Search { keywords } => opman.search(keywords),
        Action::Install { packages } => opman.install(packages),
    }
}
