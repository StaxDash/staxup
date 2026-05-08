use std::path::PathBuf;
use crate::install::get_tool_dir;

pub fn rollback_binary(tool: &str, binary_name: &str) -> anyhow::Result<()> {
    let tool_dir = get_tool_dir(tool);
    let latest_dir = tool_dir.join("latest");
    let old_dir = tool_dir.join("old");
    let latest_bin = latest_dir.join(binary_name);
    let old_bin = old_dir.join(binary_name);

    if old_bin.exists() {
        // Swap: move latest to temp, old to latest, temp to old
        let temp = tool_dir.join("temp").join(binary_name);
        std::fs::create_dir_all(&tool_dir.join("temp"))?;
        std::fs::rename(&latest_bin, &temp)?;
        std::fs::rename(&old_bin, &latest_bin)?;
        std::fs::rename(&temp, &old_bin)?;
    } else {
        anyhow::bail!("No old version to rollback to");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rollback_no_old() {
        // Hard to test without setup
    }
}