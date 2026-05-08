use stax_up::{Updater, ToolManifest};
use stax_up::rollback::rollback_binary;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    match args.as_slice() {
        [cmd, tool] if cmd == "update" => {
            let manifest_path = format!("{}.toml", tool);
            Updater::new(tool).with_manifest_path(manifest_path).check_and_apply()?;
        }
        [cmd, tool] if cmd == "check" => {
            let manifest_path = format!("{}.toml", tool);
            let manifest = ToolManifest::from_file(&manifest_path)?;
            println!("Current version: {}", manifest.version);
            if let Some(meta) = &manifest.meta {
                if let Some(name) = &meta.name {
                    println!("Name: {}", name);
                }
            }
        }
        [cmd, tool] if cmd == "rollback" => {
            rollback_binary(tool, tool)?;
            println!("Rolled back {}", tool);
        }
        _ => {
            eprintln!("Usage: staxup <update|check|rollback> <tool>");
        }
    }

    Ok(())
}
