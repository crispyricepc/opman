use std::hash::Hash;

use alpm::Dep;

use crate::Package;

pub struct AlpmPackage<'h> {
    inner: alpm::Package<'h>,
}

impl<'h> Package for AlpmPackage<'h> {
    fn name(&self) -> String {
        todo!()
    }

    fn version(&self) -> String {
        todo!()
    }

    fn desc(&self) -> String {
        todo!()
    }

    fn arch(&self) -> String {
        todo!()
    }

    fn url(&self) -> String {
        todo!()
    }

    fn licenses(&self) -> Vec<String> {
        todo!()
    }

    fn groups(&self) -> Vec<String> {
        todo!()
    }

    fn provides(&self) -> Vec<String> {
        todo!()
    }

    fn depends(&self) -> Vec<Dep> {
        todo!()
    }

    fn depends_optional(&self) -> Vec<Dep> {
        todo!()
    }

    fn required_by(&self) -> Vec<String> {
        todo!()
    }

    fn required_by_optional(&self) -> Vec<String> {
        todo!()
    }

    fn conflicts(&self) -> Vec<String> {
        todo!()
    }

    fn replaces(&self) -> Vec<String> {
        todo!()
    }

    fn installed_size(&self) -> usize {
        todo!()
    }

    fn packager(&self) -> String {
        todo!()
    }

    fn build_date(&self) -> String {
        todo!()
    }

    fn install_date(&self) -> String {
        todo!()
    }

    fn install_reason(&self) -> String {
        todo!()
    }

    fn validation(&self) -> String {
        todo!()
    }
}

impl<'h> From<alpm::Package<'h>> for AlpmPackage<'h> {
    fn from(value: alpm::Package<'h>) -> Self {
        Self { inner: value }
    }
}

impl PartialEq for AlpmPackage<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.name() == other.inner.name()
    }
}

impl Eq for AlpmPackage<'_> {}

impl Hash for AlpmPackage<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.name().hash(state);
    }
}
