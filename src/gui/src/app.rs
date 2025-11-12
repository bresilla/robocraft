use eframe::egui;
use egui_dock::{DockArea, DockState, NodeIndex, Style, TabViewer};

use crate::graph::{Graph, Node, Port};

/// Different views/tabs available in the application
#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "persistence", derive(serde::Serialize, serde::Deserialize))]
pub enum View {
    Graph,
    Settings,
    Console,
}

impl View {
    fn title(&self) -> &'static str {
        match self {
            View::Graph => "Graph",
            View::Settings => "Settings",
            View::Console => "Console",
        }
    }
}

/// Application state
pub struct RobocraftApp {
    graph: Graph,
    dock_state: DockState<View>,
    console_messages: Vec<String>,
    show_about: bool,
}

impl RobocraftApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Configure fonts if needed
        let mut style = (*cc.egui_ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(8.0, 6.0);
        cc.egui_ctx.set_style(style);

        let mut app = Self {
            graph: Graph::new(),
            dock_state: DockState::new(vec![View::Graph]),
            console_messages: Vec::new(),
            show_about: false,
        };

        // Add some demo nodes
        app.add_demo_nodes();

        app
    }

    fn add_demo_nodes(&mut self) {
        // Create a simple demo graph
        let node1 = Node::new("node1", "Source Node")
            .with_output(Port::new("out1", "Output 1", egui::Color32::BLUE))
            .with_output(Port::new("out2", "Output 2", egui::Color32::GREEN))
            .with_metadata("type", "source")
            .with_metadata("description", "This is a source node");

        let node2 = Node::new("node2", "Process Node")
            .with_input(Port::new("in1", "Input", egui::Color32::BLUE))
            .with_output(Port::new("out1", "Output", egui::Color32::RED))
            .with_metadata("type", "processor")
            .with_metadata("description", "This node processes data");

        let node3 = Node::new("node3", "Sink Node")
            .with_input(Port::new("in1", "Input 1", egui::Color32::RED))
            .with_input(Port::new("in2", "Input 2", egui::Color32::GREEN))
            .with_metadata("type", "sink")
            .with_metadata("description", "This is a sink node");

        self.graph.add_node(node1);
        self.graph.add_node(node2);
        self.graph.add_node(node3);

        self.log_message("Demo nodes created");
    }

    pub fn add_node(&mut self, node: Node) {
        let node_id = node.id.clone();
        self.graph.add_node(node);
        self.log_message(format!("Node '{}' added", node_id));
    }

    pub fn remove_node(&mut self, node_id: &str) {
        self.graph.remove_node(node_id);
        self.log_message(format!("Node '{}' removed", node_id));
    }

    fn log_message(&mut self, message: impl Into<String>) {
        self.console_messages.push(message.into());
        if self.console_messages.len() > 100 {
            self.console_messages.remove(0);
        }
    }

    fn menu_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Clear Graph").clicked() {
                        self.graph.clear();
                        self.log_message("Graph cleared");
                        ui.close_menu();
                    }
                    
                    ui.separator();
                    
                    if ui.button("Reset Demo").clicked() {
                        self.graph.clear();
                        self.add_demo_nodes();
                        ui.close_menu();
                    }

                    ui.separator();

                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("View", |ui| {
                    let open_tabs: Vec<_> = self
                        .dock_state
                        .iter_all_tabs()
                        .map(|(_, &tab)| tab)
                        .collect();

                    for view in [View::Graph, View::Settings, View::Console] {
                        let is_open = open_tabs.contains(&view);
                        if ui
                            .selectable_label(is_open, view.title())
                            .clicked()
                        {
                            if !is_open {
                                self.dock_state.push_to_focused_leaf(view);
                            }
                            ui.close_menu();
                        }
                    }
                });

                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        self.show_about = true;
                        ui.close_menu();
                    }
                });
            });
        });
    }

    fn show_about_window(&mut self, ctx: &egui::Context) {
        egui::Window::new("About Robocraft GUI")
            .open(&mut self.show_about)
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Robocraft GUI");
                    ui.label("Version 0.0.27");
                    ui.separator();
                    ui.label("A node-based graph interface for robotics");
                    ui.label("Built with egui and egui-snarl");
                });
            });
    }
}

impl eframe::App for RobocraftApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.menu_bar(ctx);

        if self.show_about {
            self.show_about_window(ctx);
        }

        // Main content area with docking
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut viewer = AppTabViewer {
                graph: &mut self.graph,
                console_messages: &mut self.console_messages,
            };

            DockArea::new(&mut self.dock_state)
                .style(Style::from_egui(ui.style().as_ref()))
                .show_inside(ui, &mut viewer);
        });
    }

    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, "dock_state", &self.dock_state);
    }

    #[cfg(feature = "persistence")]
    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }
}

struct AppTabViewer<'a> {
    graph: &'a mut Graph,
    console_messages: &'a mut Vec<String>,
}

impl TabViewer for AppTabViewer<'_> {
    type Tab = View;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.title().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            View::Graph => {
                let mut connect_log = String::new();
                let mut disconnect_log = String::new();

                self.graph.show(
                    ui,
                    &mut |from, to| {
                        connect_log = format!("Connected {} -> {}", from, to);
                    },
                    &mut |from, to| {
                        disconnect_log = format!("Disconnected {} -> {}", from, to);
                    },
                );

                if !connect_log.is_empty() {
                    self.console_messages.push(connect_log);
                }
                if !disconnect_log.is_empty() {
                    self.console_messages.push(disconnect_log);
                }
            }
            View::Settings => {
                ui.heading("Settings");
                ui.separator();
                ui.label("Settings panel - coming soon");
                ui.add_space(10.0);
                
                ui.horizontal(|ui| {
                    ui.label("Theme:");
                    if ui.button("Dark").clicked() {
                        ui.ctx().set_visuals(egui::Visuals::dark());
                    }
                    if ui.button("Light").clicked() {
                        ui.ctx().set_visuals(egui::Visuals::light());
                    }
                });
            }
            View::Console => {
                ui.heading("Console");
                ui.separator();
                
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        for msg in self.console_messages.iter() {
                            ui.label(msg);
                        }
                    });

                ui.separator();
                
                ui.horizontal(|ui| {
                    if ui.button("Clear").clicked() {
                        self.console_messages.clear();
                    }
                });
            }
        }
    }

    fn scroll_bars(&self, _tab: &Self::Tab) -> [bool; 2] {
        [false, false]
    }
}
