use crate::error::FetchError;
use std::process::Command;

pub fn packages_info(os: &str) -> Result<String, FetchError> {
    let (command, args) = match os {
        os if os.contains("Fedora") => ("dnf", vec!["list", "--installed"]),
        os if os.contains("openSUSE") || os.contains("SUSE") => {
            ("zypper", vec!["packages", "--installed-only"])
        }
        os if os.contains("Red Hat")
            || os.contains("RHEL")
            || os.contains("CentOS")
            || os.contains("Rocky")
            || os.contains("Alma") => ("rpm", vec!["-qa"]),
        os if os.contains("Debian")
            || os.contains("Ubuntu")
            || os.contains("Mint")
            || os.contains("Pop")
            || os.contains("elementary") => ("dpkg", vec!["--get-selections"]),
        os if os.contains("Arch") || os.contains("Manjaro") || os.contains("Endeavour") => {
            ("pacman", vec!["-Q"])
        }
        os if os.contains("Alpine") => ("apk", vec!["list", "--installed"]),
        os if os.contains("Gentoo") => ("qlist", vec!["-I"]),
        os if os.contains("Void") => ("xbps-query", vec!["-l"]),
        os if os.contains("Solus") => ("eopkg", vec!["list-installed"]),
        os if os.contains("NixOS") => (
            "nix-store",
            vec!["--query", "--requisites", "/run/current-system/sw"],
        ),
        _ => {
            return Err(FetchError::NotSupported(format!(
                "OS not supported: {}",
                os
            )))
        }
    };

    let output = Command::new(command)
        .args(&args)
        .output()
        .map_err(|e| FetchError::CommandFailed(format!("{}: {}", command, e)))?;

    let count = String::from_utf8_lossy(&output.stdout).lines().count();
    Ok(count.to_string())
}
