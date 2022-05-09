use alpm::Package;
use bytesize::ByteSize;

pub fn print_package(pkg: &Package, compact: bool) {
    print!(
        "{}/{} {}",
        pkg.db().unwrap().name(),
        pkg.name(),
        pkg.version()
    );
    if !compact {
        println!("\n\t{}", pkg.desc().unwrap());
    } else {
        println!("");
    }
}

pub fn print_summary(
    n_packages: usize,
    n_installed: usize,
    n_explicit: usize,
    n_dependencies: usize,
    n_bytes: usize,
    largest: Package,
) {
    println!(
        "Total Packages: {}\n{} installed, {} not installed, {} explicit, {} dependencies\nTotal size: {}\nLargest package: {} @ {}",
        n_packages,
        n_installed,
        n_packages - n_installed,
        n_explicit,
        n_dependencies,
        ByteSize(n_bytes as u64),
        largest.name(),
        ByteSize(largest.isize() as u64)
    );
}
