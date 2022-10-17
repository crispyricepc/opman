use std::collections::HashSet;

use alpm::Db;
use log::error;

use crate::{package::alpm_package::AlpmPackage, Package};

use super::Database;

pub struct Pacman<'h> {
    db: &'h Db<'h>,
}

impl<'h> Pacman<'h> {
    pub fn new(db: &'h Db) -> Self {
        Self { db }
    }
}

impl<'h> Database<AlpmPackage<'h>> for Pacman<'h> {
    fn get_package(&self, name: String) -> Option<AlpmPackage<'h>> {
        self.db.pkg(name).map(|pkg| AlpmPackage::from(pkg)).ok()
    }

    fn get_packages(&self) -> Vec<AlpmPackage<'h>> {
        self.db
            .pkgs()
            .into_iter()
            .map(|pkg| AlpmPackage::from(pkg))
            .collect()
    }

    fn search(&self, queries: Vec<String>) -> Vec<AlpmPackage<'h>> {
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

    fn dependencies(&self, pkgs: &Vec<AlpmPackage>) -> HashSet<String> {
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
