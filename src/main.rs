mod database;
mod display;
mod package;
mod package_ops;

pub use database::Database;
use database::{handle, Pacman};
use package::AlpmPackage;
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
        .level(log::LevelFilter::Info)
        .chain(std::io::stderr())
        .apply()?;
    Ok(())
}

fn main() {
    // Initialize logger
    build_logger().unwrap();

    let handle = handle();
    let db = &handle.syncdbs().into_iter().next().unwrap();
    let sync = Pacman::new(db);

    let cli = Cli::parse();

    match cli.command {
        Action::Summary { packages: _ } => todo!(),
        Action::Dependencies { packages } => {
            let pkgs = packages
                .into_iter()
                .filter_map(|pkg| sync.get_package(pkg))
                .collect::<Vec<AlpmPackage>>();
            sync.dependencies(&pkgs);
        }
        Action::Search { keywords } => {
            sync.search(keywords);
        }
        Action::Install { packages: _ } => todo!(),
    }
}
