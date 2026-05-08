pub enum Platform {
    LinuxAmd64,
    LinuxArm64,
    WindowsX64,
    MacOSUniversal,
}

pub fn detect() -> Platform {
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    {
        Platform::LinuxAmd64
    }
    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    {
        Platform::LinuxArm64
    }
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    {
        Platform::WindowsX64
    }
    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    {
        Platform::MacOSUniversal
    }
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    {
        Platform::MacOSUniversal
    }
    #[cfg(not(any(
        all(target_os = "linux", target_arch = "x86_64"),
        all(target_os = "linux", target_arch = "aarch64"),
        all(target_os = "windows", target_arch = "x86_64"),
        all(target_os = "macos", target_arch = "x86_64"),
        all(target_os = "macos", target_arch = "aarch64")
    )))]
    {
        panic!("Unsupported platform");
    }
}

impl Platform {
    pub fn key(&self) -> &'static str {
        match self {
            Platform::LinuxAmd64 => "linux_amd64",
            Platform::LinuxArm64 => "linux_arm64",
            Platform::WindowsX64 => "windows_x64",
            Platform::MacOSUniversal => "macos_universal",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_detect() {
        let p = detect();
        // Just assert it returns something (doesn't panic)
        match p {
            Platform::LinuxAmd64 | Platform::LinuxArm64 | Platform::WindowsX64 | Platform::MacOSUniversal => {}
        }
    }
}