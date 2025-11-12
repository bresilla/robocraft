pub mod app;
pub mod graph;

pub use app::RobocraftApp;
pub use graph::{Graph, Node, Port};

/// Run the GUI application
pub fn run_gui() -> eframe::Result<()> {
    // Check for display environment
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() {
        eprintln!("Error: No display environment found!");
        eprintln!("This is a graphical application that requires a desktop environment.");
        eprintln!("Please run this on a system with X11 or Wayland display server.");
        eprintln!("\nFor testing in headless environments, use:");
        eprintln!("  xvfb-run -a cargo run --package robocraft_gui");
        std::process::exit(1);
    }

    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("Robocraft GUI")
            .with_inner_size([1280.0, 720.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    match eframe::run_native(
        "Robocraft GUI",
        native_options,
        Box::new(|cc| Ok(Box::new(RobocraftApp::new(cc)))),
    ) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Failed to start GUI: {}", e);
            eprintln!("\nPossible causes:");
            eprintln!("1. Missing display libraries (libX11, libxcb, libwayland)");
            eprintln!("2. No access to display server");
            eprintln!("3. Running in headless/containerized environment");
            eprintln!("\nSolutions:");
            eprintln!("- On Debian/Ubuntu: sudo apt-get install libx11-dev libxcb1-dev");
            eprintln!("- Use virtual display: xvfb-run -a cargo run --package robocraft_gui");
            eprintln!("- Run on a desktop system with GUI");
            Err(e)
        }
    }
}
