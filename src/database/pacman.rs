use std::collections::HashSet;

use alpm::Db;
use log::error;

use crate::{package::alpm_package::AlpmPackage, Package};

use super::Database;

pub struct Pacman {
    db: Db<'static>,
}

impl Pacman {
    pub fn _new(db: Db<'static>) -> Self {
        Self { db }
    }
}

impl Database<AlpmPackage> for Pacman {
    fn get_package(&self, name: &str) -> Option<AlpmPackage> {
        self.db
            .pkg(name)
            .as_ref()
            .map(|pkg| AlpmPackage::from(*pkg))
            .ok()
    }

    fn get_packages(&self) -> Vec<AlpmPackage> {
        self.db
            .pkgs()
            .into_iter()
            .map(|pkg| AlpmPackage::from(pkg))
            .collect()
    }

    fn search(&self, queries: Vec<String>) -> Vec<AlpmPackage> {
        match self.db.search(queries.into_iter()) {
            Ok(results) => results
                .into_iter()
                .map(|result| AlpmPackage::from(result))
                .collect(),
            Err(e) => {
                error!("Search failed, attempting to continue: {}", e);
                vec![]
            }
        }
    }

    fn dependencies(&self, pkgs: &Vec<impl Package>) -> HashSet<String> {
        let mut deps = HashSet::new();
        for pkg in pkgs {
            deps.extend(
                pkg.depends()
                    .into_iter()
                    .map(|dep| String::from(dep.name())),
            );
        }
        deps
    }
}
