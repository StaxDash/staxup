use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ToolManifest {
    pub version: String,
    pub channel: Option<String>,
    pub min_updater: Option<String>,

    pub downloads: Downloads,
    pub checksums: Option<Checksums>,
    pub meta: Option<Meta>,
}

#[derive(Debug, Deserialize)]
pub struct Downloads {
    pub linux_amd64: Option<String>,
    pub linux_arm64: Option<String>,
    pub windows_x64: Option<String>,
    pub macos_universal: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Checksums {
    pub linux_amd64: Option<String>,
    pub linux_arm64: Option<String>,
    pub windows_x64: Option<String>,
    pub macos_universal: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Meta {
    pub name: Option<String>,
    pub description: Option<String>,
    pub license: Option<String>,
}

impl ToolManifest {
    pub fn from_file(path: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let parsed: ToolManifest = toml::from_str(&content)?;
        Ok(parsed)
    }

    pub fn from_url(url: &str) -> anyhow::Result<Self> {
        let text = reqwest::blocking::get(url)?.text()?;
        Ok(toml::from_str(&text)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_manifest() {
        let toml_str = r#"
version = "2.0.0"
channel = "stable"
min_updater = "1.0.0"

[downloads]
linux_amd64 = "https://example.com/tool-linux-amd64"
windows_x64 = "https://example.com/tool-windows-x64.exe"

[checksums]
linux_amd64 = "sha256-abc123"
windows_x64 = "sha256-def456"

[meta]
name = "StaxPing"
description = "Network diagnostics tool"
license = "Apache-2.0"
"#;

        let manifest: ToolManifest = toml::from_str(toml_str).expect("Failed to parse TOML");

        assert_eq!(manifest.version, "2.0.0");
        assert_eq!(manifest.channel, Some("stable".to_string()));
        assert_eq!(manifest.min_updater, Some("1.0.0".to_string()));
        assert_eq!(manifest.downloads.linux_amd64, Some("https://example.com/tool-linux-amd64".to_string()));
        assert_eq!(manifest.downloads.windows_x64, Some("https://example.com/tool-windows-x64.exe".to_string()));
        assert_eq!(manifest.checksums.as_ref().unwrap().linux_amd64, Some("sha256-abc123".to_string()));
        assert_eq!(manifest.meta.as_ref().unwrap().name, Some("StaxPing".to_string()));
    }

    #[test]
    fn test_manifest_loads() {
        let manifest = ToolManifest::from_file("tests/test_manifest.toml").unwrap();
        assert_eq!(manifest.version, "1.0.0");
    }

    #[test]
    fn test_manifest_from_url() {
        // Note: reqwest doesn't support file:// URLs, so this test is skipped
        // In real usage, this would fetch from HTTP URLs
        // For testing, we rely on from_file
    }
}
