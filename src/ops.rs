use std::{
    collections::{HashMap, HashSet},
    iter::once,
};

use alpm::{Alpm, Db, SigLevel};
use anyhow::Result;
use log::{debug, info, warn};

use crate::{database::Aur, package::Dependency, Database, Package};

pub struct Opman {
    handle: Alpm,
    aur_db: Aur,
}

pub struct PackageSummary {
    pub count: u32,
    pub total_size: usize,
}

impl Opman {
    pub fn new() -> Self {
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

        Self {
            handle,
            aur_db: Aur::new(),
        }
    }

    pub fn alpm_databases(&self) -> impl Iterator<Item = Db> {
        once(self.handle.localdb()).chain(self.handle.syncdbs())
    }

    pub fn summary(&self, packages: Vec<String>) -> Result<PackageSummary> {
        let packages = self.get_packages(&packages)?;

        let ret = PackageSummary {
            count: packages.len() as u32,
            total_size: packages.iter().map(|pkg| pkg.installed_size).sum(),
        };

        Ok(ret)
    }

    pub fn get_package(&self, package_name: &String) -> Result<Package> {
        // Search alpm
        for found_pkg in self
            .alpm_databases()
            .find_map(|db| db.get_package(&package_name).ok())
        {
            return Ok(found_pkg);
        }

        // Search AUR
        self.aur_db.get_package(&package_name)
    }

    pub fn get_packages(&self, package_names: &Vec<String>) -> Result<HashSet<Package>> {
        let mut packages = HashSet::new();

        for package_name in package_names {
            packages.insert(self.get_package(&package_name.to_string())?);
        }

        Ok(packages)
    }

    pub fn dependencies(
        &self,
        package_names: &Vec<String>,
    ) -> Result<HashMap<Dependency, Option<Package>>> {
        let mut ret = HashMap::new();

        for package in self.get_packages(package_names)? {
            ret.extend(package.depends.iter().map(|dep| {
                (
                    dep.clone(),
                    match self.get_package(&dep.name) {
                        Ok(pkg) => Some(pkg),
                        Err(e) => {
                            warn!(
                                "Couldn't find dependency '{}' for package '{}' in any database, {}",
                                dep.name, package.name, e
                            );
                            None
                        }
                    },
                )
            }))
        }

        Ok(ret)
    }

    pub fn search(&self, keywords: Vec<String>) {
        todo!()
    }

    pub fn install(&self, packages: Vec<String>) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn dependencies() {}
}
