use alpm::{Alpm, SigLevel};
use log::{debug, info};

use crate::Database;

/// Get an Alpm handle
///
/// Seems wasteful to get a new handle each time this function is called but
/// alpm doesn't implement Sync so we can't use a static variable.
fn handle() -> Alpm {
    let handle = Alpm::new("/", "/var/lib/pacman").unwrap();
    handle
        .register_syncdb("core", SigLevel::USE_DEFAULT)
        .unwrap();
    handle
        .register_syncdb("extra", SigLevel::USE_DEFAULT)
        .unwrap();
    handle
        .register_syncdb("community", SigLevel::USE_DEFAULT)
        .unwrap();
    handle
}

fn get_databases<'h>(handle: &'h Alpm) -> Vec<Box<dyn Database + 'h>> {
    let raw_databases = handle.syncdbs();
    let mut dbs = vec![];
    dbs.extend(
        raw_databases
            .into_iter()
            .map(|db| Box::new(db) as Box<dyn Database>),
    );
    dbs.push(Box::new(handle.localdb()));
    dbs
}

pub fn summary(packages: Vec<String>) {
    todo!()
}
pub fn dependencies(packages: &Vec<String>) {
    let handle = handle();
    let dbs = get_databases(&handle);

    for db in &dbs {
        info!("Searching database {}", db.db_name());
        for pkg in packages {
            match db.get_package(pkg) {
                Ok(pkg) => {
                    for dep in pkg.depends {
                        println!("{}", dep);
                    }
                }
                Err(e) => debug!("Failed to get package {} from {}: {}", pkg, db.db_name(), e),
            }
        }
    }
}
pub fn search(keywords: Vec<String>) {
    todo!()
}
pub fn install(packages: Vec<String>) {
    todo!()
}
