use std::env::vars;
use std::path::PathBuf;

/// Commonly required Alfred information from the user's settings.
/// See <https://www.alfredapp.com/help/workflows/script-environment-variables/> for more info.
pub struct AlfredEnv;

impl AlfredEnv {
    /// If the user currently has the debug panel open for this workflow.
    pub fn is_debug() -> bool {
        vars()
            .find(|(key, _)| key == "alfred_debug")
            .map(|(_, value)| value == "1")
            .unwrap_or(false)
    }

    /// Bundle Identifier of the running workflow.
    /// Note: this will only be populated if your workflow has a bundle id set.
    pub fn workflow_bundle_id() -> Option<String> {
        vars()
            .find(|(key, _)| key == "alfred_workflow_bundleid")
            .map(|(_, value)| value)
    }

    /// Recommended locations for volatile workflow data.
    /// Note: this will only be populated if your workflow has a bundle id set.
    pub fn workflow_cache_dir() -> Option<PathBuf> {
        vars()
            .find(|(key, _)| key == "alfred_workflow_cache")
            .map(|(_, value)| PathBuf::from(value))
    }

    /// Recommended locations for non-volatile workflow data.
    /// Note: this will only be populated if your workflow has a bundle id set.
    pub fn workflow_data_dir() -> Option<PathBuf> {
        vars()
            .find(|(key, _)| key == "alfred_workflow_data")
            .map(|(_, value)| PathBuf::from(value))
    }
}
