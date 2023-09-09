use std::{
    collections::{HashMap, HashSet},
    iter::once,
};

use alpm::{Alpm, Db, SigLevel};
use log::warn;

use crate::{database::Aur, error::Result, package::Dependency, Database, Package};

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

    pub fn search(&self, keywords: Vec<String>) -> Result<Vec<Package>> {
        let mut results = Vec::<Package>::new();
        for db in self.alpm_databases() {
            results.extend(db.search_packages(keywords.clone())?);
        }

        results.extend(self.aur_db.search_packages(keywords.clone())?);

        Ok(results)
    }

    pub fn install(&self, packages: Vec<String>) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_package() {
        let opman = Opman::new();

        let git = opman.get_package(&"git".to_string()).unwrap();
        assert_eq!(git.name, "git");
    }

    #[test]
    fn get_package_should_fail() {
        let opman = Opman::new();

        let git = opman.get_package(&"this-package-does-not-exist".to_string());
        assert!(git.is_err());
        assert_eq!(
            git.unwrap_err().kind,
            crate::error::ErrorKind::PackageNotFound
        );
    }

    #[test]
    fn get_packages() {
        let opman = Opman::new();

        let packages = opman
            .get_packages(&vec!["git".to_string(), "bash".to_string()])
            .unwrap();
        assert_eq!(packages.len(), 2);
        assert!(packages.iter().any(|p| p.name == "git"));
        assert!(packages.iter().any(|p| p.name == "bash"));
    }

    #[test]
    fn get_packages_should_fail() {
        let opman = Opman::new();

        let packages = opman.get_packages(&vec!["this-package-does-not-exist".to_string()]);
        assert!(packages.is_err());
        assert_eq!(
            packages.unwrap_err().kind,
            crate::error::ErrorKind::PackageNotFound
        );
    }

    #[test]
    fn dependencies() {
        let opman = Opman::new();

        let git_deps = opman.dependencies(&vec!["git".to_string()]).unwrap();
        assert_eq!(git_deps.len(), 10);

        let bash_deps = opman.dependencies(&vec!["bash".to_string()]).unwrap();
        assert_eq!(bash_deps.len(), 4);
    }

    #[test]
    fn dependencies_should_fail() {
        let opman = Opman::new();

        let deps = opman.dependencies(&vec!["this-package-does-not-exist".to_string()]);
        assert!(deps.is_err());
        assert_eq!(
            deps.unwrap_err().kind,
            crate::error::ErrorKind::PackageNotFound
        );
    }

    #[test]
    fn search_one() {
        let opman = Opman::new();

        let results = opman.search(vec!["git".to_string()]).unwrap();
        assert!(results.iter().any(|p| p.name == "git"));

        let results = opman.search(vec!["ash".to_string()]).unwrap();
        assert!(results.iter().any(|p| p.name == "bash"));
    }

    #[test]
    fn search_many() {
        let opman = Opman::new();

        let results = opman
            .search(vec!["ttf".to_string(), "gnu".to_string(), "ee".to_string()])
            .unwrap();
        assert!(results.iter().any(|p| p.name == "gnu-free-fonts"));
    }

    #[test]
    fn search_no_results() {}
}
