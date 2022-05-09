mod display;
mod package_ops;

use clap::{Arg, ArgMatches, Command};
use fern::colors::{Color, ColoredLevelConfig};
use package_ops::PackageOps;

fn build_command() -> ArgMatches {
    Command::new("opman")
        .version("0.0.1")
        .author("Ben Mitchell")
        .about("Opinionated Archlinux Package Manager")
        .subcommand(
            Command::new("summary")
                .about("Summarize the given packages")
                .arg(Arg::new("packages").required(false).multiple_values(true)),
        )
        .subcommand(
            Command::new("dependencies")
                .about("Get the given packages' dependencies")
                .alias("deps")
                .arg(Arg::new("packages").required(true).multiple_values(true)),
        )
        .subcommand(
            Command::new("search")
                .about("Search for packages")
                .arg(Arg::new("query").required(true).multiple_values(true)),
        )
        .subcommand(
            Command::new("install")
                .about("Install a package")
                .arg(Arg::new("packages").required(true).multiple_values(true)),
        )
        .get_matches()
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

    let ops = PackageOps::new();

    match build_command().subcommand() {
        Some(("summary", summary_matches)) => {
            let pkgs: Vec<&str> = summary_matches
                .values_of("packages")
                .unwrap_or_default()
                .collect();
            ops.summary(&pkgs);
        }
        Some(("dependencies", dependencies_matches)) => {
            let pkgs: Vec<&str> = dependencies_matches
                .values_of("packages")
                .unwrap_or_default()
                .collect();
            ops.dependencies(&pkgs);
        }
        Some(("search", search_matches)) => {
            let queries: Vec<&str> = search_matches
                .values_of("query")
                .unwrap_or_default()
                .collect();
            ops.search(&queries);
        }
        Some(("install", install_matches)) => {
            let pkgs: Vec<&str> = install_matches
                .values_of("packages")
                .unwrap_or_default()
                .collect();
            ops.install(&pkgs);
        }
        _ => {
            println!("No subcommand was used");
        }
    }
}
