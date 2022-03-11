use alpm::{Alpm, Db, Package, PackageReason};
use bytesize::ByteSize;

fn find_package<'a>(handle: &'a Alpm, pkgname: &str) -> Result<Package<'a>, String> {
    let mut dbs = vec![handle.localdb()];
    dbs.append(&mut handle.syncdbs().iter().collect::<Vec<Db>>());
    for db in dbs {
        if let Ok(pkg) = db.pkg(pkgname) {
            return Ok(pkg);
        }
    }
    Err(format!("Could not find package {}", pkgname))
}

fn find_packages<'a>(handle: &'a Alpm, pkgs: Vec<&str>) -> Result<Vec<Package<'a>>, String> {
    let mut packages = vec![];
    for pkgname in pkgs {
        // First search the local database
        packages.push(match find_package(handle, pkgname) {
            Ok(pkg) => pkg,
            Err(e) => {
                return Err(e);
            }
        });
    }
    Ok(packages)
}

fn recurse_dependencies<'a>(handle: &'a Alpm, pkg: &Package, deps: &mut Vec<Package<'a>>) {
    for dep in pkg.depends() {
        if deps.iter().any(|d| d.name() == dep.name()) {
            continue;
        }
        if let Ok(dep_pkg) = find_package(handle, dep.name()) {
            deps.push(dep_pkg);
            recurse_dependencies(handle, &dep_pkg, deps);
        }
    }
}

fn summary_pkgs(handle: &Alpm, pkgs: &Vec<Package>) {
    let mut pkgs_plus_deps = pkgs.clone();
    for pkg in pkgs {
        recurse_dependencies(handle, pkg, &mut pkgs_plus_deps);
    }

    let mut n_installed = 0i64;
    let mut n_bytes = 0i64;
    let mut n_explicit = 0i64;
    let mut n_dependencies = 0i64;
    let mut largest: Option<&Package> = None;

    for pkg in &pkgs_plus_deps {
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

    println!(
        "Total Packages: {}\n{} installed, {} not installed, {} explicit, {} dependencies\nTotal size: {}\nLargest package: {} @ {}",
        pkgs_plus_deps.len(),
        n_installed,
        pkgs_plus_deps.len() - n_installed as usize,
        n_explicit,
        n_dependencies,
        ByteSize(n_bytes as u64),
        largest.unwrap().name(),
        ByteSize(largest.unwrap().isize() as u64)
    );
}

pub fn summary(handle: &Alpm, pkgs: Vec<&str>) {
    if pkgs.len() > 0 {
        summary_pkgs(handle, &find_packages(handle, pkgs).unwrap());
    } else {
        summary_pkgs(handle, &handle.localdb().pkgs().iter().collect());
    }
}
