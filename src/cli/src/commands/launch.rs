use robocraft_gui::run_gui;

pub fn handle() {
    if let Err(e) = run_gui() {
        eprintln!("Failed to launch GUI: {}", e);
        std::process::exit(1);
    }
}
