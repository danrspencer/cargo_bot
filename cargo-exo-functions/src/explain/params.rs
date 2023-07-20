use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct ExplainParams {
    pub explinations: Vec<Explination>,
}

/// A human readable explination of the failure and how it could be solved.
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct Explination {
    /// The original error message that was returned from the compiler.
    pub cause: String,
    /// A discussion on the problem and how it could be solved. If more information from the user could
    /// help solve the problem then ask for it here.
    pub explination: String,
}
