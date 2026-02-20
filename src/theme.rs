use crate::error::FetchError;
use std::env;
use std::process::Command;

pub fn theme_info() -> Result<String, FetchError> {
    let mut parts = Vec::new();

    if let Some(theme) = get_gtk_theme() {
        parts.push(format!("Theme: {}", theme));
    }

    if let Some(icons) = get_icon_theme() {
        parts.push(format!("Icons: {}", icons));
    }

    if let Some(cursor) = get_cursor_theme() {
        parts.push(format!("Cursor: {}", cursor));
    }

    if parts.is_empty() {
        Err(FetchError::NotFound)
    } else {
        Ok(parts.join(" | "))
    }
}

fn get_gtk_theme() -> Option<String> {
    if let Ok(output) = Command::new("gsettings")
        .args(["get", "org.gnome.desktop.interface", "gtk-theme"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let theme = stdout.trim().trim_matches('\'').to_string();
        if !theme.is_empty() && theme != "N/A" {
            return Some(theme);
        }
    }

    if let Ok(output) = Command::new("xfconf-query")
        .args(["-c", "xsettings", "-p", "/Net/ThemeName"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let theme = stdout.trim().to_string();
        if !theme.is_empty() {
            return Some(theme);
        }
    }

    if let Ok(home) = env::var("HOME") {
        let gtk3_path = format!("{}/.config/gtk-3.0/settings.ini", home);
        if let Some(theme) = parse_gtk_ini(&gtk3_path, "gtk-theme-name") {
            return Some(theme);
        }

        let gtk2_path = format!("{}/.gtkrc-2.0", home);
        if let Some(theme) = parse_gtk_rc(&gtk2_path, "gtk-theme-name") {
            return Some(theme);
        }
    }

    None
}

fn get_icon_theme() -> Option<String> {
    if let Ok(output) = Command::new("gsettings")
        .args(["get", "org.gnome.desktop.interface", "icon-theme"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let theme = stdout.trim().trim_matches('\'').to_string();
        if !theme.is_empty() && theme != "N/A" {
            return Some(theme);
        }
    }

    if let Ok(output) = Command::new("xfconf-query")
        .args(["-c", "xsettings", "-p", "/Net/IconThemeName"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let theme = stdout.trim().to_string();
        if !theme.is_empty() {
            return Some(theme);
        }
    }

    if let Ok(home) = env::var("HOME") {
        let gtk3_path = format!("{}/.config/gtk-3.0/settings.ini", home);
        if let Some(theme) = parse_gtk_ini(&gtk3_path, "gtk-icon-theme-name") {
            return Some(theme);
        }

        let gtk2_path = format!("{}/.gtkrc-2.0", home);
        if let Some(theme) = parse_gtk_rc(&gtk2_path, "gtk-icon-theme-name") {
            return Some(theme);
        }
    }

    None
}

fn get_cursor_theme() -> Option<String> {
    if let Ok(output) = Command::new("gsettings")
        .args(["get", "org.gnome.desktop.interface", "cursor-theme"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let theme = stdout.trim().trim_matches('\'').to_string();
        if !theme.is_empty() && theme != "N/A" {
            return Some(theme);
        }
    }

    if let Ok(output) = Command::new("xfconf-query")
        .args(["-c", "xsettings", "-p", "/Gtk/CursorThemeName"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let theme = stdout.trim().to_string();
        if !theme.is_empty() {
            return Some(theme);
        }
    }

    None
}

fn parse_gtk_ini(path: &str, key: &str) -> Option<String> {
    if let Ok(content) = std::fs::read_to_string(path) {
        for line in content.lines() {
            if line.starts_with(key) {
                if let Some((_, value)) = line.split_once('=') {
                    let value = value.trim().to_string();
                    if !value.is_empty() {
                        return Some(value);
                    }
                }
            }
        }
    }
    None
}

fn parse_gtk_rc(path: &str, key: &str) -> Option<String> {
    if let Ok(content) = std::fs::read_to_string(path) {
        let search = format!("{}=\"", key);
        for line in content.lines() {
            if line.contains(&search) {
                if let Some(start) = line.find('"') {
                    if let Some(end) = line[start + 1..].find('"') {
                        let value = line[start + 1..start + 1 + end].to_string();
                        if !value.is_empty() {
                            return Some(value);
                        }
                    }
                }
            }
        }
    }
    None
}
