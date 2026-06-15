#[derive(Debug, PartialEq, Eq)]
struct PackageInfo {
    name: &'static str,
    edition: &'static str,
}

impl PackageInfo {
    fn summary(&self) -> String {
        format!("package {} uses Rust {} Edition", self.name, self.edition)
    }
}

fn default_package() -> PackageInfo {
    PackageInfo {
        name: "rt_01_cargo_project",
        edition: "2024",
    }
}

fn main() {
    let package = default_package();
    println!("{}", package.summary());
    println!("cargo check / build / run / test use the same package metadata.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn package_summary_uses_metadata() {
        assert_eq!(
            default_package().summary(),
            "package rt_01_cargo_project uses Rust 2024 Edition",
        );
    }
}
