//! General utilities.
//!
//! This module provides helper functions for common operations like path
//! construction, string normalization, and cache directory management.
//!
//! These utilities are used internally by the library but are also exposed
//! for use by other crates that need similar functionality.

use crate::{config, RepoType};

use std::path::PathBuf;

/// Returns the user's cache directory path.
///
/// This follows platform conventions for cache storage:
/// - On Unix-like systems: `~/.cache`
/// - On Windows: `%USERPROFILE%\.cache`
/// - On macOS: `~/.cache` (not following macOS conventions for consistency)
///
/// # Panics
///
/// Panics if the user's home directory cannot be determined. This typically
/// only happens in very restricted environments or when the HOME environment
/// variable is not set.
///
/// # Examples
///
/// ```rust
/// use hf_hub::utils::get_cache_dir;
///
/// let cache = get_cache_dir();
/// assert!(cache.ends_with(".cache"));
/// ```
pub fn get_cache_dir() -> PathBuf {
    let mut path = dirs::home_dir().expect("Cache directory cannot be found");
    path.push(config::CACHE_DIR);
    path
}

/// Returns the top-level Hugging Face directory within the cache.
///
/// The path follows the pattern: `{cache_dir}/huggingface`
///
/// # Examples
///
/// ```rust
/// use hf_hub::utils::get_top_level_hf_dir;
///
/// let hf_dir = get_top_level_hf_dir();
/// assert!(hf_dir.ends_with("huggingface"));
/// ```
pub fn get_top_level_hf_dir() -> PathBuf {
    let mut path = get_cache_dir();
    path.push(config::TOP_LEVEL_HF_DIR);
    path
}

/// Returns the Hugging Face Hub directory for repository storage.
///
/// The path follows the pattern: `{cache_dir}/{top_level_hf_dir}/hub`
///
/// # Examples
///
/// ```rust
/// use hf_hub::utils::get_hub_dir;
///
/// let hub_dir = get_hub_dir();
/// assert!(hub_dir.to_string_lossy().contains("huggingface/hub"));
/// ```
pub fn get_hub_dir() -> PathBuf {
    let mut path = get_top_level_hf_dir();
    path.push(config::HUB_DIR);
    path
}

/// Convert a repo type and repo ID to a normalized folder name.
///
/// # Examples
///
/// ```rust
/// use hf_hub::{RepoType, utils::normalized_repo_folder_name};
///
/// let folder_name = normalized_repo_folder_name(&RepoType::Model, "HuggingFaceTB/SmolLM2-135M");
/// assert_eq!(folder_name, "models--HuggingFaceTB--SmolLM2-135M");
///
/// let dataset_folder = normalized_repo_folder_name(&RepoType::Dataset, "squad");
/// assert_eq!(dataset_folder, "datasets--squad");
/// ```
pub fn normalized_repo_folder_name(repo_type: &RepoType, repo_id: &str) -> String {
    let prefix = repo_type.prefix();
    let prefixed_repo_id = format!("{prefix}{}{repo_id}", config::NORMALIZED_SEPARATOR);
    normalize_separator(&prefixed_repo_id)
}

/// Convert path separators in a string to normalized separators.
///
/// # Examples
///
/// ```rust
/// use hf_hub::utils::normalize_separator;
///
/// let normalized = normalize_separator("HuggingFaceTB/SmolLM2-135M");
/// assert_eq!(normalized, "HuggingFaceTB--SmolLM2-135M");
///
/// let no_change = normalize_separator("simple-name");
/// assert_eq!(no_change, "simple-name");
/// ```
pub fn normalize_separator(input: &str) -> String {
    input.replace(config::SEPARATOR, config::NORMALIZED_SEPARATOR)
}

/// Encodes path separators in a string.
///
/// # Examples
///
/// ```rust
/// use hf_hub::utils::encode_separator;
///
/// let normalized = encode_separator("HuggingFaceTB/SmolLM2-135M");
/// assert_eq!(normalized, "HuggingFaceTB%2FSmolLM2-135M");
///
/// let no_change = encode_separator("simple-name");
/// assert_eq!(no_change, "simple-name");
/// ```
pub fn encode_separator(input: &str) -> String {
    input.replace(config::SEPARATOR, config::ENCODED_SEPARATOR)
}
