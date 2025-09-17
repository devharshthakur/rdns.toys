//! # IFSC (Indian Financial System Code) Service
//!
//! This module parses and indexes IFSC data from JSON files to provide
//! fast lookups for Indian bank branch information by IFSC code.
//! It's a direct port of the Go version's `ifsc` package.

use anyhow::{Context, Ok, Result};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::ifsc;

const IFSC_CODE_LEN: usize = 11;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Branch {
    #[serde(rename = "BANK")]
    pub bank: String,
    #[serde(rename = "IFSC")]
    pub ifsc: String,
    #[serde(rename = "MICR")]
    pub micr: String,
    #[serde(rename = "BRANCH")]
    pub branch: String,
    #[serde(rename = "ADDRESS")]
    pub address: String,
    #[serde(rename = "STATE")]
    pub state: String,
    #[serde(rename = "CITY")]
    pub city: String,
    #[serde(rename = "CENTRE")]
    pub centre: String,
    #[serde(rename = "DISTRICT")]
    pub district: String,
}

pub struct IFSC {
    data: HashMap<String, Branch>,
}

impl IFSC {
    /// Creates a new IFSC instance by loading and indexing IFSC data from JSON files.
    ///
    /// This constructor scans the specified directory for JSON files containing IFSC
    /// (Indian Financial System Code) data and builds an in-memory index for fast lookups.
    /// Each JSON file should contain a mapping of IFSC codes to branch information.
    ///
    /// # Arguments
    ///
    /// * `dir` - A path to the directory containing IFSC JSON data files.
    ///           Can be any type that implements `AsRef<Path>` (e.g., `&str`, `String`, `Path`, `PathBuf`).
    ///
    /// # Returns
    ///
    /// * `Result<Self>` - Returns `Ok(IFSC)` on successful loading, or `Err` if:
    ///   - The directory cannot be read
    ///   - Any JSON file cannot be read or parsed
    ///   - The JSON structure doesn't match the expected `Branch` format
    ///
    /// # File Format
    ///
    /// Each JSON file should contain a JSON object where keys are IFSC codes and values
    /// are branch objects with the following structure:
    /// ```json
    /// {
    ///   "IFSC_CODE": {
    ///     "BANK": "Bank Name",
    ///     "IFSC": "IFSC_CODE",
    ///     "MICR": "MICR Code",
    ///     "BRANCH": "Branch Name",
    ///     "ADDRESS": "Branch Address",
    ///     "STATE": "State",
    ///     "CITY": "City",
    ///     "CENTRE": "Centre",
    ///     "DISTRICT": "District"
    ///   }
    /// }
    /// ```
    pub fn new<P: AsRef<Path>>(dir: P) -> Result<Self> {
        let dir_path = dir.as_ref();
        tracing::info!("IFSC data loading from {}", dir_path.display());

        // Hashmap containing IFSC contents
        let mut ifsc_data = HashMap::new();
        let entries = fs::read_dir(dir_path)
            .with_context(|| format!("Error reading IFSC directory {}", dir_path.display()))?;

        for entry in entries {
            let entry = entry?; // resolve any possible errors
            let path = entry.path();

            if !path.is_file() {
                continue;
            }

            let file_content = fs::read_to_string(&path)
                .with_context(|| format!("Error reading IFSC json file: {}", path.display()))?;

            // Unmarshall the json file
            let branches: HashMap<String, Branch> = serde_json::from_str(&file_content)
                .with_context(|| {
                    format!("Error unmarshalling the IFSC json file {}", path.display())
                })?;

            for (_, branch) in branches {
                ifsc_data.insert(branch.ifsc.clone(), branch);
            }
        }
        tracing::info!("Loaded {} IFSC records", ifsc_data.len());
        Ok(IFSC { data: ifsc_data })
    }
}
