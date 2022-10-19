use alpm::Db;
use anyhow::Result;
use log::error;

use crate::Package;

use super::Database;

impl<'h> Database<'h> for Db<'h> {
    fn db_name(&self) -> String {
        self.name().to_string()
    }

    fn get_package(&self, name: &String) -> Result<Package> {
        Ok(self.pkg(name.as_str())?.into())
    }

    fn get_packages(&self) -> Vec<Package> {
        self.pkgs().into_iter().map(|p| p.into()).collect()
    }

    fn search(&self, queries: Vec<String>) -> Result<Vec<Package>> {
        self.search(queries.into_iter().map(|s| s))?
            .into_iter()
            .map(|p| Ok(p.into()))
            .collect()
    }

    fn dependencies(&self, pkgs: &Vec<Package>) -> Vec<String> {
        let mut deps = vec![];
        for pkg in pkgs {
            match self.pkg(pkg.name.as_str()) {
                Ok(alpm_pkg) => {
                    deps.extend(alpm_pkg.depends().into_iter().map(|d| d.name().to_string()));
                }
                Err(e) => {
                    error!("Failed to get package {}: {}", pkg.name, e);
                }
            }
        }
        deps
    }
}
