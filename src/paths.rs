// ABOUTME: Resolves the data directory shared by the completion output and the user registry.
// ABOUTME: Follows the XDG layout on every platform, macOS included.

use std::ffi::OsString;
use std::path::PathBuf;

/// `$XDG_DATA_HOME`, else `~/.local/share`.
pub fn data_home() -> PathBuf {
    resolve_data_home(
        std::env::var_os("XDG_DATA_HOME"),
        dirs::home_dir().unwrap_or_default(),
    )
}

fn resolve_data_home(xdg_data_home: Option<OsString>, home: PathBuf) -> PathBuf {
    xdg_data_home
        .map(PathBuf::from)
        .unwrap_or_else(|| home.join(".local").join("share"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_home_prefers_xdg_data_home() {
        assert_eq!(
            resolve_data_home(Some("/xdg/data".into()), PathBuf::from("/home/me")),
            PathBuf::from("/xdg/data")
        );
    }

    #[test]
    fn test_data_home_falls_back_to_local_share_under_home() {
        assert_eq!(
            resolve_data_home(None, PathBuf::from("/home/me")),
            PathBuf::from("/home/me/.local/share")
        );
    }
}
