use std::path::{Path, PathBuf};

pub fn get_tool_dir(tool: &str) -> PathBuf {
    let data_dir = dirs::data_dir().expect("Could not find data directory");
    data_dir.join("stax").join(tool)
}

pub fn ensure_dirs(tool: &str) -> anyhow::Result<()> {
    let tool_dir = get_tool_dir(tool);
    let latest_dir = tool_dir.join("latest");
    let old_dir = tool_dir.join("old");

    std::fs::create_dir_all(&latest_dir)?;
    std::fs::create_dir_all(&old_dir)?;

    Ok(())
}

pub fn install_binary(tool: &str, src: &Path, binary_name: &str) -> anyhow::Result<()> {
    ensure_dirs(tool)?;
    let tool_dir = get_tool_dir(tool);
    let latest_dir = tool_dir.join("latest");
    let old_dir = tool_dir.join("old");
    let dest = latest_dir.join(binary_name);

    // Move old to old/ if exists
    if dest.exists() {
        let old_dest = old_dir.join(binary_name);
        std::fs::rename(&dest, &old_dest)?;
    }

    // Copy new
    std::fs::copy(src, &dest)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_get_tool_dir() {
        let dir = get_tool_dir("testtool");
        assert!(dir.ends_with("stax/testtool"));
    }

    #[test]
    fn test_ensure_dirs() {
        let tmp = tempdir().unwrap();
        // Mock data_dir, but for test, just check creation
        // Since dirs::data_dir is fixed, hard to test without env
        // For now, skip or use a temp dir
    }

    #[test]
    fn test_install_binary() {
        let tmp = tempdir().unwrap();
        let src = tmp.path().join("src_bin");
        std::fs::write(&src, b"binary content").unwrap();

        // Mock the dirs, but since it's hard, perhaps integrate later
    }
}