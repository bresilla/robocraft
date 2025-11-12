# Testing Robocraft GUI

## Compilation Status

✅ **Compiles successfully** with Rust 1.85+ and edition2024

```bash
cargo check --package robocraft_gui     # ✅ Passes
cargo build --package robocraft_gui     # ✅ Builds successfully
cargo build --example custom_graph      # ✅ Example builds
```

## Runtime Environment

⚠️ **Requires Desktop Environment**

This application requires a graphical environment to run. It cannot run in headless or containerized environments without proper display setup.

### Successful Environments

The GUI will run on:
- ✅ Linux desktop with X11
- ✅ Linux desktop with Wayland  
- ✅ macOS (with appropriate features)
- ✅ Windows
- ✅ SSH with X11 forwarding (`ssh -X`)

### Failed Environments

The GUI will **NOT** run on:
- ❌ Headless servers
- ❌ Docker without display mounting
- ❌ SSH without X11 forwarding
- ❌ CI/CD pipelines (unless using virtual displays)

## Testing on Desktop

### 1. Build the Application

```bash
cargo build --release --package robocraft_gui
```

### 2. Run the Main Application

```bash
# From workspace root
cargo run --release --package robocraft_gui

# Or directly
./target/release/rcgui
```

### 3. Run the Example

```bash
cargo run --release --package robocraft_gui --example custom_graph
```

## Expected Behavior

When run on a proper desktop environment, you should see:

1. **Main Window** opens (1280x720)
2. **Menu Bar** at top with File, View, Help menus
3. **Graph View** showing 3 demo nodes:
   - Source Node (blue/green outputs)
   - Process Node (blue input, red output)
   - Sink Node (red/green inputs)
4. **Dockable Panels** - you can drag tabs to rearrange
5. **Console Tab** - shows log messages

## Interaction Tests

### Graph Controls
- ✅ **Pan**: Click and drag on background
- ✅ **Zoom**: Ctrl + mouse wheel
- ✅ **Reset**: Double-click background
- ✅ **Connect**: Drag from output port to input port
- ✅ **Disconnect**: Click on a wire/connection
- ✅ **Select**: Right-click drag to select multiple nodes

### Menu Tests
- ✅ File → Clear Graph (removes all nodes)
- ✅ File → Reset Demo (restores demo nodes)
- ✅ View → Switch between Graph/Settings/Console
- ✅ Help → About (shows about dialog)

### Console Tests
- ✅ Messages appear when nodes are added/removed
- ✅ Messages appear when connections are made/broken
- ✅ Clear button removes all messages
- ✅ Auto-scrolls to bottom

## Testing with Virtual Display (Linux)

If you need to test in a headless environment:

```bash
# Install Xvfb (X Virtual Framebuffer)
sudo apt-get install xvfb

# Run with virtual display
xvfb-run -a cargo run --package robocraft_gui

# Or with custom display settings
Xvfb :99 -screen 0 1280x720x24 &
export DISPLAY=:99
cargo run --package robocraft_gui
```

## Compilation Warnings

Current warnings (non-critical):
- Unused methods (public API not yet fully utilized)
- Deprecated egui::menu::bar (uses old API)
- Profile warnings (expected in workspace setup)

All warnings are **safe to ignore** for testing purposes.

## Known Limitations

1. **Platform Support**: Tested primarily on Linux
2. **Display Required**: Cannot run headless without virtual display
3. **Library Dependencies**: Requires system graphics libraries
4. **Edition 2024**: Needs latest Rust toolchain

## Troubleshooting

### "libX11.so.6: cannot open shared object file"
**Solution**: Install X11 libraries
```bash
sudo apt-get install libx11-dev libxcb1-dev
```

### "Error: WinitEventLoop"
**Solution**: You're in a headless environment. Run on a desktop or use Xvfb.

### Compilation errors with older Rust
**Solution**: Update to Rust 1.85+
```bash
rustup update stable
```

## Success Criteria

✅ Application compiles without errors
✅ Binary runs and window appears (on desktop)
✅ Demo nodes are visible
✅ Can pan and zoom the graph
✅ Can connect/disconnect nodes
✅ Menu items work
✅ Console shows messages

## Future Testing

- [ ] Unit tests for graph operations
- [ ] Integration tests for node connections
- [ ] Screenshot tests for visual regression
- [ ] Performance tests with many nodes
- [ ] Stress tests with complex graphs
