use crate::package::Package;

pub fn print_package(pkg: &Package, compact: bool) {
    print!("{}/{} {}", pkg.db_name, pkg.name, pkg.version);
    if !compact {
        println!("\n\t{}", &pkg.desc.as_ref().unwrap());
    } else {
        println!();
    }
}

pub fn print_packages<'a>(pkgs: impl Iterator<Item = &'a Package>, compact: bool) {
    for pkg in pkgs {
        print_package(&pkg, compact);
    }
}
