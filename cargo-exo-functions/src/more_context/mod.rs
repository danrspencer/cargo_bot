use std::path::Path;

pub use self::params::*;

mod params;

pub fn more_context(params: &MoreContextParams, project_root: &Path) {
    for file in &params.files {
        println!("{}", project_root.join(file).display());
    }
}
