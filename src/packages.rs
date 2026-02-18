use std::process::Command;

pub fn packages_info(os: String) -> String {
    let (command, args) = match os.as_str() {
        // RPM-based
        os if os.contains("Fedora") => ("dnf", vec!["list", "--installed"]),
        os if os.contains("openSUSE") || os.contains("SUSE") => ("zypper", vec!["packages", "--installed-only"]),
        os if os.contains("Red Hat") || os.contains("RHEL") || os.contains("CentOS") || os.contains("Rocky") || os.contains("Alma") => ("rpm", vec!["-qa"]),
        
        // Debian-based (incluye Linux Mint, Pop!_OS, elementary, etc.)
        os if os.contains("Debian") || os.contains("Ubuntu") || os.contains("Mint") || os.contains("Pop") || os.contains("elementary") => ("dpkg", vec!["--get-selections"]),
        
        // Arch-based (incluye Manjaro, EndeavourOS, etc.)
        os if os.contains("Arch") || os.contains("Manjaro") || os.contains("Endeavour") => ("pacman", vec!["-Q"]),
        
        // Alpine Linux
        os if os.contains("Alpine") => ("apk", vec!["list", "--installed"]),
        
        // Gentoo
        os if os.contains("Gentoo") => ("qlist", vec!["-I"]),
        
        // Void Linux
        os if os.contains("Void") => ("xbps-query", vec!["-l"]),
        
        // Solus
        os if os.contains("Solus") => ("eopkg", vec!["list-installed"]),
        
        // NixOS
        os if os.contains("NixOS") => ("nix-store", vec!["--query", "--requisites", "/run/current-system/sw"]),
        
        _ => return "OS not supported".to_string(),
    };

    let output = Command::new(command)
        .args(&args)
        .output();
    
    match output {
        Ok(output) => String::from_utf8_lossy(&output.stdout)
            .lines()
            .count()
            .to_string(),
        Err(_) => "N/A".to_string(),
    }
}
