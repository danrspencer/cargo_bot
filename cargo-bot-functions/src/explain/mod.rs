pub use self::params::*;

mod params;

pub fn explain(params: &ExplainParams) {
    for explination in &params.explinations {
        let _t = "test";
        println!("{}", explination.cause);
        println!("{}", explination.explination);
    }
}
