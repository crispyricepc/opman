pub mod alpm_package;
pub mod aur_package;

pub use alpm_package::AlpmPackage;
pub use aur_package::AurPackage;

use alpm::Dep;

pub trait Package {
    fn name(&self) -> String;
    fn version(&self) -> String;
    fn desc(&self) -> String;
    fn arch(&self) -> String;
    fn url(&self) -> String;
    fn licenses(&self) -> Vec<String>;
    fn groups(&self) -> Vec<String>;
    fn provides(&self) -> Vec<String>;
    fn depends(&self) -> Vec<Dep>;
    fn depends_optional(&self) -> Vec<Dep>;
    fn required_by(&self) -> Vec<String>;
    fn required_by_optional(&self) -> Vec<String>;
    fn conflicts(&self) -> Vec<String>;
    fn replaces(&self) -> Vec<String>;
    fn installed_size(&self) -> usize;
    fn packager(&self) -> String;
    fn build_date(&self) -> String;
    fn install_date(&self) -> String;
    fn install_reason(&self) -> String;
    fn validation(&self) -> String;
}
