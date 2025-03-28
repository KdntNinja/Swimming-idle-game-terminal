use rand::rng;
use miette::{IntoDiagnostic, Result};
use rand::prelude::*;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
pub struct NameData {
    pub first_names: Vec<String>,
    pub last_names: Vec<String>,
    pub nicknames: Vec<String>,
}

/// Loads swimmer name data from JSON file
///
/// # Returns
/// A Result containing the NameData or an error
pub fn load_name_data() -> Result<NameData> {
    let path: &Path = Path::new("src/data/swimmer_names.json");
    let data: String = fs::read_to_string(path).into_diagnostic()?;
    let name_data: NameData = serde_json::from_str::<NameData>(&data).into_diagnostic()?;
    Ok(name_data)
}

/// Generates a random swimmer name using the provided name data
///
/// # Arguments
/// * `name_data` - Reference to name data structure
///
/// # Returns
/// A String containing the generated name
pub fn generate_random_name(name_data: &NameData) -> String {
    let mut rng: ThreadRng = rng();
    let use_nickname: bool = rng.random_bool(0.3_f64); // 30% chance to use a nickname

    if use_nickname {
        let nickname_index: usize = rng.random_range(0_usize..name_data.nicknames.len());
        let nickname: &String = &name_data.nicknames[nickname_index];

        let lastname_index: usize = rng.random_range(0_usize..name_data.last_names.len());
        let lastname: &String = &name_data.last_names[lastname_index];

        format!("\"{}\" {}", nickname, lastname)
    } else {
        let firstname_index: usize = rng.random_range(0_usize..name_data.first_names.len());
        let firstname: &String = &name_data.first_names[firstname_index];

        let lastname_index: usize = rng.random_range(0_usize..name_data.last_names.len());
        let lastname: &String = &name_data.last_names[lastname_index];

        format!("{} {}", firstname, lastname)
    }
}
