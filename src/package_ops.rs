use alpm::{Alpm, AlpmListMut, Db, Package, PackageReason, SigLevel};
use log::info;

use crate::display::{print_package, print_summary};

pub struct PackageOps {
    handle: Alpm,
}

impl PackageOps {
    fn dbs(&self) -> Vec<Db> {
        let mut dbs = vec![self.handle.localdb()];
        dbs.extend(self.handle.syncdbs());
        dbs
    }

    fn find_package(&self, pkgname: &str) -> Result<Package, String> {
        for db in self.dbs() {
            if let Ok(pkg) = db.pkg(pkgname) {
                return Ok(pkg);
            }
        }
        Err(format!("Could not find package {}", pkgname))
    }

    fn find_packages(&self, pkgs: &Vec<&str>) -> Result<Vec<Package>, String> {
        let mut packages = vec![];
        for pkgname in pkgs {
            // First search the local database
            packages.push(match self.find_package(pkgname) {
                Ok(pkg) => pkg,
                Err(e) => {
                    return Err(e);
                }
            });
        }
        Ok(packages)
    }

    fn search_packages<'a>(&'a self, queries: &Vec<&str>) -> AlpmListMut<Package<'a>> {
        info!("Searching for packages");
        let mut pkgs = AlpmListMut::new(&self.handle);
        for db in self.dbs() {
            pkgs.extend(db.search(queries.iter()).unwrap());
        }

        pkgs
    }

    fn recurse_dependencies<'a>(&'a self, pkg: Package, deps: &mut Vec<Package<'a>>) {
        for dep in pkg.depends() {
            if deps.iter().any(|d| d.name() == dep.name()) {
                continue;
            }
            if let Ok(dep_pkg) = self.find_package(dep.name()) {
                deps.push(dep_pkg.clone());
                self.recurse_dependencies(dep_pkg, deps);
            }
        }
    }

    fn get_dependencies<'a>(&'a self, pkgs: Vec<Package<'a>>) -> Vec<Package> {
        let mut deps = pkgs.clone();
        for pkg in pkgs {
            self.recurse_dependencies(pkg, &mut deps);
        }
        deps
    }

    fn summary_pkgs(&self, pkgs: &Vec<Package>) {
        let mut n_installed = 0i64;
        let mut n_bytes = 0i64;
        let mut n_explicit = 0i64;
        let mut n_dependencies = 0i64;
        let mut largest: Option<Package> = None;

        for pkg in pkgs.clone() {
            if largest.is_none() || pkg.isize() > largest.unwrap().isize() {
                largest = Some(pkg);
            }

            if pkg.install_date().is_some() {
                n_installed += 1;

                if pkg.reason() == PackageReason::Explicit {
                    n_explicit += 1;
                }
                if pkg.reason() == PackageReason::Depend {
                    n_dependencies += 1;
                }
            }
            n_bytes += pkg.isize();
        }

        print_summary(
            pkgs.len(),
            n_installed as usize,
            n_explicit as usize,
            n_dependencies as usize,
            n_bytes as usize,
            largest.unwrap(),
        )
    }

    pub fn new() -> PackageOps {
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
        PackageOps { handle }
    }

    pub fn summary(self, pkgs: &Vec<&str>) {
        info!("Building package summary");
        if pkgs.len() > 0 {
            self.summary_pkgs(&self.get_dependencies(self.find_packages(pkgs).unwrap()));
        } else {
            self.summary_pkgs(&self.handle.localdb().pkgs().iter().collect());
        }
    }

    pub fn dependencies(&self, pkgs: &Vec<&str>) {
        let deps = self.get_dependencies(self.find_packages(pkgs).unwrap());
        for dep in deps {
            print_package(&dep, false);
        }
    }

    pub fn search(&self, queries: &Vec<&str>) {
        let pkgs = self.search_packages(queries);
        for pkg in pkgs {
            print_package(&pkg, false);
        }
    }

    pub fn install(&self, _pkgs: &Vec<&str>) {
        panic!("Not implemented");
    }
}
