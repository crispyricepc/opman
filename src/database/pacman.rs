use alpm::Db;
use anyhow::Result;
use log::error;

use crate::{package::Dependency, Package};

use super::Database;

impl Database for Db<'_> {
    fn db_name(&self) -> String {
        self.name().to_string()
    }

    fn get_package(&self, name: &String) -> Result<Package> {
        Ok(self.pkg(name.as_str())?.into())
    }

    fn all_packages(&self) -> Vec<Package> {
        self.pkgs().into_iter().map(|p| p.into()).collect()
    }

    fn search(&self, queries: Vec<String>) -> Result<Vec<Package>> {
        self.search(queries.into_iter().map(|s| s))?
            .into_iter()
            .map(|p| Ok(p.into()))
            .collect()
    }

    fn dependencies(&self, pkgs: &Vec<Package>) -> Vec<Dependency> {
        let mut deps = vec![];
        for pkg in pkgs {
            match self.pkg(pkg.name.as_str()) {
                Ok(alpm_pkg) => {
                    let deps_to_add: Vec<Dependency> =
                        alpm_pkg.depends().iter().map(|d| d.into()).collect();
                    deps.extend(deps_to_add);
                }
                Err(e) => {
                    error!("Failed to get package {}: {}", pkg.name, e);
                }
            }
        }
        deps
    }
}
