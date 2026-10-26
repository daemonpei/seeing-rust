//! Chapter 13: Crates, Modules and Project Organisation

use ch13_modules::{checker::report, config::Config, run};

fn main() {
    // `use crate::…` writes `crate::`; from a binary that consumes the library
    // the path starts with the package name instead.
    let cfg = Config::from_env().unwrap_or_default();
    let report = run(&cfg);

    println!("{}", report::format(&report));
    println!("{}", report.target);

    // Three ways to name the same item, all valid:
    let _a = Config::default();
    let _b = ch13_modules::Config::default();
    use ch13_modules::Config as Cfg;
    let _c = Cfg::default();

    println!();
    println!("config.port() is the only way in: the field is private.");
    println!("so this would not compile:  cfg.port");
}
