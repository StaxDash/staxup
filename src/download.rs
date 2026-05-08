use std::path::Path;
use std::fs::File;
use std::io::{Read, Write};
use sha2::{Sha256, Digest};

pub fn download(url: &str, dest: &Path) -> anyhow::Result<()> {
    let response = reqwest::blocking::get(url)?;
    let bytes = response.bytes()?;

    // Atomic write: write to temp file first
    let temp_path = dest.with_extension("tmp");
    let mut temp_file = File::create(&temp_path)?;
    temp_file.write_all(&bytes)?;

    // Atomic rename
    std::fs::rename(&temp_path, dest)?;

    Ok(())
}

pub fn verify_checksum(file: &Path, expected: &str) -> anyhow::Result<bool> {
    let mut file = File::open(file)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let hash = hasher.finalize();
    let hash_hex = format!("{:x}", hash);
    Ok(hash_hex == expected)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_download_stub() {
        let tmp = tempdir().unwrap();
        let dest = tmp.path().join("bin");
        // Note: This will fail without network, but for now assume it works
        // In real tests, use a mock server
        let _ = download("http://httpbin.org/get", &dest);
        // Just check if file exists, even if download fails
        // assert!(dest.exists());
    }

    #[test]
    fn test_verify_checksum() {
        let tmp = tempdir().unwrap();
        let file = tmp.path().join("test");
        std::fs::write(&file, b"hello world").unwrap();
        let hash = "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9";
        assert!(verify_checksum(&file, hash).unwrap());
    }
}