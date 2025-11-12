# Robocraft GUI

A node-based graph interface for robotics built with egui. Extracted and simplified from the [coppwr](https://github.com/dimtpap/coppwr) project.

## Requirements

**Important:** This crate requires Rust 1.76 or newer due to dependencies using edition2024 features.

```bash
# Check your Rust version
rustc --version

# Update Rust if needed
rustup update stable
```

### Runtime Requirements

This is a **desktop GUI application** and requires:
- A display server (X11 or Wayland)
- Graphics libraries (libX11, libxcb, or Wayland libraries)
- A windowing environment (not suitable for headless/server environments)

**Note:** The application will **not run** in:
- SSH sessions without X11 forwarding
- Docker containers without display access
- Headless servers
- CI/CD environments without virtual displays

## Features

- **Interactive Node Graph**: Visual node-based programming interface using egui-snarl
- **Docking Layout**: Flexible workspace with dockable panels using egui_dock
- **Multiple Views**: 
  - Graph view for node editing
  - Console for logging
  - Settings panel
- **Graph Operations**:
  - Add/remove nodes
  - Connect/disconnect ports
  - Auto-positioning of nodes
  - Pan, zoom, and navigate
  - Color-coded ports

## Building

From the workspace root:

```bash
cargo build --package robocraft_gui
```

## Running

```bash
# Run the main GUI application
cargo run --package robocraft_gui

# Or using the binary name directly
cargo run --bin rcgui

# Run the custom graph example
cargo run --package robocraft_gui --example custom_graph
```

## Architecture

### Modules

- **graph.rs**: Core graph data structure and rendering logic
  - `Graph`: Main graph container
  - `Node`: Represents a node with inputs/outputs
  - `Port`: Connection points on nodes
  
- **app.rs**: Main application and UI logic
  - `RobocraftApp`: Main application state
  - Tab views and docking layout
  - Menu system

### Graph Usage

```rust
use robocraft_gui::{Graph, Node, Port};

let mut graph = Graph::new();

// Create a node
let node = Node::new("node1", "My Node")
    .with_input(Port::new("in1", "Input", egui::Color32::BLUE))
    .with_output(Port::new("out1", "Output", egui::Color32::RED))
    .with_metadata("type", "processor");

graph.add_node(node);
```

## Controls

- **Pan**: Click and drag
- **Zoom**: Ctrl + scroll
- **Reset View**: Double click
- **Connect Ports**: Drag from output to input
- **Disconnect**: Click on wire
- **Select Nodes**: Right-click and drag

## Dependencies

- `eframe`: egui framework for desktop apps
- `egui`: Immediate mode GUI library
- `egui_dock`: Docking support
- `egui-snarl`: Node graph library
- `serde`: Serialization (for persistence feature)

## Features

- `persistence` (default): Save/restore window layout and state

## License

See workspace LICENSE
