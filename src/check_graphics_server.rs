use std::fs;

pub enum GraphicsSystem {
    Wayland,
    XWayland,
    CLI,
    Unknown,
}

pub fn get_graphics_system(pid: u32) -> GraphicsSystem {
    let proc_path = format!("/proc/{}/environ", pid);
    match fs::read_to_string(&proc_path) {
        Ok(environ_file) => {
            if environ_file.contains("WAYLAND_DISPLAY") {
                GraphicsSystem::Wayland
            } else if environ_file.contains("XWAYLAND_DISPLAY") {
                GraphicsSystem::XWayland
            } else {
                // Add more checks for other graphics systems if needed
                GraphicsSystem::CLI
            }
        }
        Err(err) => {
            // Log or handle the error more appropriately
            eprintln!("Error reading {}: {}", proc_path, err);
            GraphicsSystem::Unknown
        }
    }
}