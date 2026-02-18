use std::env;
use std::process::Command;

pub fn desktop_info() -> String {
    // Intentar obtener desde variables de entorno (más común)
    if let Ok(de) = env::var("XDG_CURRENT_DESKTOP") {
        if !de.is_empty() {
            return format_de(&de);
        }
    }
    
    if let Ok(de) = env::var("DESKTOP_SESSION") {
        if !de.is_empty() {
            return format_de(&de);
        }
    }
    
    if let Ok(de) = env::var("GNOME_DESKTOP_SESSION_ID") {
        if !de.is_empty() {
            return "GNOME".to_string();
        }
    }
    
    // Detectar Wayland compositors específicos
    if env::var("WAYLAND_DISPLAY").is_ok() {
        if let Ok(wayland_display) = env::var("WAYLAND_DISPLAY") {
            if wayland_display.contains("sway") {
                return "sway".to_string();
            }
            if wayland_display.contains("wayfire") {
                return "Wayfire".to_string();
            }
        }
        
        // Detectar Hyprland
        if env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok() {
            return "Hyprland".to_string();
        }
    }
    
    // Intentar detectar sesión con loginctl
    if let Ok(output) = Command::new("loginctl").args(["show-session", "self", "-p", "Desktop"]).output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Some((_, value)) = stdout.split_once('=') {
            let value = value.trim();
            if !value.is_empty() && value != "Desktop=" {
                return format_de(value);
            }
        }
    }
    
    // Intentar detectar i3/sway desde el proceso
    if env::var("I3SOCK").is_ok() {
        return "i3".to_string();
    }
    
    if let Ok(sw_sock) = env::var("SWAYSOCK") {
        if !sw_sock.is_empty() {
            return "sway".to_string();
        }
    }
    
    // Detectar bspwm, dwm, awesome, etc. desde procesos
    if let Ok(output) = Command::new("ps").args(["-e", "-o", "comm="]).output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            let wm = match line.trim() {
                "i3" => "i3",
                "bspwm" => "bspwm",
                "dwm" => "dwm",
                "awesome" => "awesome",
                "xmonad" | "xmonad-x86_64-linux" => "xmonad",
                "qtile" => "Qtile",
                "openbox" => "Openbox",
                "fluxbox" => "Fluxbox",
                "spectrwm" => "spectrwm",
                "herbstluftwm" => "herbstluftwm",
                "river" => "River",
                "dwl" => "dwl",
                _ => continue,
            };
            return wm.to_string();
        }
    }
    
    "Unknown".to_string()
}

fn format_de(de: &str) -> String {
    match de {
        "GNOME" => "GNOME".to_string(),
        "gnome" => "GNOME".to_string(),
        "KDE" | "plasma" | "Plasma" => "KDE Plasma".to_string(),
        "XFCE" | "Xfce" => "Xfce".to_string(),
        "MATE" => "MATE".to_string(),
        "Cinnamon" => "Cinnamon".to_string(),
        "LXDE" => "LXDE".to_string(),
        "LXQt" => "LXQt".to_string(),
        "Unity" => "Unity".to_string(),
        "Budgie" => "Budgie".to_string(),
        "Pantheon" => "Pantheon".to_string(),
        "Deepin" => "Deepin".to_string(),
        "sway" => "sway".to_string(),
        "i3" => "i3".to_string(),
        "Hyprland" => "Hyprland".to_string(),
        "Wayfire" => "Wayfire".to_string(),
        _ => de.to_string(),
    }
}
