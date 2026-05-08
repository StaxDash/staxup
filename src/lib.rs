pub mod manifest;
pub mod download;
pub mod install;
pub mod rollback;
pub mod platform;
pub mod errors;

pub use crate::manifest::ToolManifest;
use crate::download::{download, verify_checksum};
use crate::install::install_binary;
use semver::Version;

pub struct Updater {
    tool_name: String,
    local_manifest_path: Option<String>,
    remote_manifest_url: Option<String>,
}

impl Updater {
    pub fn new<T: Into<String>>(tool_name: T) -> Self {
        Self {
            tool_name: tool_name.into(),
            local_manifest_path: None,
            remote_manifest_url: None,
        }
    }

    pub fn with_manifest_path<T: Into<String>>(mut self, path: T) -> Self {
        self.local_manifest_path = Some(path.into());
        self
    }

    pub fn with_remote_manifest<T: Into<String>>(mut self, url: T) -> Self {
        self.remote_manifest_url = Some(url.into());
        self
    }

    pub fn check_and_apply(&self) -> anyhow::Result<()> {
        let manifest_path = self.local_manifest_path
            .as_ref()
            .expect("manifest path not set");

        let local_manifest = ToolManifest::from_file(manifest_path)?;

        let remote_manifest = if let Some(url) = &self.remote_manifest_url {
            Some(ToolManifest::from_url(url)?)
        } else {
            None
        };

        if let Some(remote) = remote_manifest {
            let local_ver = Version::parse(&local_manifest.version)?;
            let remote_ver = Version::parse(&remote.version)?;

            if remote_ver > local_ver {
                // Update needed
                let platform = crate::platform::detect();
                let platform_key = platform.key();

                let download_url = match platform_key {
                    "linux_amd64" => remote.downloads.linux_amd64.as_ref(),
                    "linux_arm64" => remote.downloads.linux_arm64.as_ref(),
                    "windows_x64" => remote.downloads.windows_x64.as_ref(),
                    "macos_universal" => remote.downloads.macos_universal.as_ref(),
                    _ => None,
                }.ok_or_else(|| anyhow::anyhow!("No download URL for platform {}", platform_key))?;

                let checksum = remote.checksums.as_ref().and_then(|c| match platform_key {
                    "linux_amd64" => c.linux_amd64.as_ref(),
                    "linux_arm64" => c.linux_arm64.as_ref(),
                    "windows_x64" => c.windows_x64.as_ref(),
                    "macos_universal" => c.macos_universal.as_ref(),
                    _ => None,
                }).ok_or_else(|| anyhow::anyhow!("No checksum for platform {}", platform_key))?;

                // Download to temp
                let temp_dir = tempfile::tempdir()?;
                let temp_file = temp_dir.path().join("binary");
                download(download_url, &temp_file)?;

                // Verify
                if !verify_checksum(&temp_file, checksum)? {
                    anyhow::bail!("Checksum verification failed");
                }

                // Install
                install_binary(&self.tool_name, &temp_file, &self.tool_name)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_updater_creation() {
        let updater = Updater::new("test_tool");
        assert_eq!(updater.tool_name, "test_tool");
        assert!(updater.local_manifest_path.is_none());
        assert!(updater.remote_manifest_url.is_none());
    }

    #[test]
    fn test_updater_with_manifest() {
        let updater = Updater::new("test_tool").with_manifest_path("path/to/manifest.toml");
        assert_eq!(updater.local_manifest_path, Some("path/to/manifest.toml".to_string()));
    }

    #[test]
    fn test_check_and_apply() {
        let updater = Updater::new("test_tool").with_manifest_path("tests/test_manifest.toml");
        let result = updater.check_and_apply();
        assert!(result.is_ok());
    }

    #[test]
    fn test_version_compare() {
        use semver::Version;
        assert!(Version::parse("2.0.0").unwrap() > Version::parse("1.0.0").unwrap());
    }
}
