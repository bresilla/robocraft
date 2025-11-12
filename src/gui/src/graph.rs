use std::collections::{HashMap, HashSet};

use eframe::egui;
use egui::emath::TSTransform;
use egui_snarl::{
    InPin, InPinId, NodeId, OutPin, OutPinId, Snarl,
    ui::{PinInfo, SnarlPin, SnarlStyle, WireLayer},
};

/// Represents a port (input or output) on a node
#[derive(Clone)]
pub struct Port {
    pub id: String,
    pub name: String,
    pub color: egui::Color32,
}

impl Port {
    pub fn new(id: impl Into<String>, name: impl Into<String>, color: egui::Color32) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            color,
        }
    }

    fn snarl_pin_info(&self) -> PinInfo {
        PinInfo::circle().with_fill(self.color)
    }
}

/// Represents a node in the graph with inputs and outputs
#[derive(Clone)]
pub struct Node {
    pub id: String,
    pub label: String,
    pub inputs: Vec<Port>,
    pub outputs: Vec<Port>,
    pub metadata: HashMap<String, String>,
}

impl Node {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn with_input(mut self, port: Port) -> Self {
        self.inputs.push(port);
        self
    }

    pub fn with_output(mut self, port: Port) -> Self {
        self.outputs.push(port);
        self
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    pub fn add_input(&mut self, port: Port) {
        self.inputs.push(port);
    }

    pub fn add_output(&mut self, port: Port) {
        self.outputs.push(port);
    }
}

struct Viewer<'a> {
    on_connect: &'a mut dyn FnMut(&str, &str),
    on_disconnect: &'a mut dyn FnMut(&str, &str),
}

impl egui_snarl::ui::SnarlViewer<Node> for Viewer<'_> {
    fn title(&mut self, node: &Node) -> String {
        node.label.clone()
    }

    fn show_header(
        &mut self,
        node: NodeId,
        _inputs: &[InPin],
        _outputs: &[OutPin],
        ui: &mut egui::Ui,
        snarl: &mut Snarl<Node>,
    ) {
        if let Some(node) = snarl.get_node(node) {
            ui.label(&node.id);
            ui.label(&node.label);
        }
    }

    fn has_footer(&mut self, _node: &Node) -> bool {
        true
    }

    fn outputs(&mut self, node: &Node) -> usize {
        node.outputs.len()
    }

    fn inputs(&mut self, node: &Node) -> usize {
        node.inputs.len()
    }

    fn show_footer(
        &mut self,
        node_id: NodeId,
        _inputs: &[InPin],
        _outputs: &[OutPin],
        ui: &mut egui::Ui,
        snarl: &mut Snarl<Node>,
    ) {
        if let Some(node) = snarl.get_node(node_id) {
            if !snarl.get_node_info(node_id).map_or(false, |info| info.open) {
                return;
            }

            ui.separator();
            
            egui::CollapsingHeader::new("Details")
                .default_open(false)
                .show(ui, |ui| {
                    egui::ScrollArea::vertical()
                        .max_height(200.0)
                        .show(ui, |ui| {
                            egui::Grid::new("node_metadata").show(ui, |ui| {
                                for (key, value) in &node.metadata {
                                    ui.label(key);
                                    ui.label(value);
                                    ui.end_row();
                                }
                            });
                        });
                });
        }
    }

    fn show_input(
        &mut self,
        pin: &InPin,
        ui: &mut egui::Ui,
        snarl: &mut Snarl<Node>,
    ) -> impl SnarlPin + 'static {
        let node = snarl
            .get_node(pin.id.node)
            .expect("snarl requested showing of pin not belonging to any node");

        let port = &node.inputs[pin.id.input];

        if snarl.get_node_info(pin.id.node).unwrap().open {
            ui.label(&port.name);
        }

        port.snarl_pin_info()
    }

    fn show_output(
        &mut self,
        pin: &OutPin,
        ui: &mut egui::Ui,
        snarl: &mut Snarl<Node>,
    ) -> impl SnarlPin + 'static {
        let node = snarl
            .get_node(pin.id.node)
            .expect("snarl requested showing of pin not belonging to any node");

        let port = &node.outputs[pin.id.output];

        if snarl.get_node_info(pin.id.node).unwrap().open {
            ui.label(&port.name);
        }

        port.snarl_pin_info()
    }

    fn connect(&mut self, from: &OutPin, to: &InPin, snarl: &mut Snarl<Node>) {
        let Some(out_node) = snarl.get_node(from.id.node) else {
            return;
        };
        let Some(in_node) = snarl.get_node(to.id.node) else {
            return;
        };

        let out_port_id = &out_node.outputs[from.id.output].id;
        let in_port_id = &in_node.inputs[to.id.input].id;

        (self.on_connect)(out_port_id, in_port_id);
    }

    fn disconnect(&mut self, from: &OutPin, to: &InPin, snarl: &mut Snarl<Node>) {
        let Some(out_node) = snarl.get_node(from.id.node) else {
            return;
        };
        let Some(in_node) = snarl.get_node(to.id.node) else {
            return;
        };

        let out_port_id = &out_node.outputs[from.id.output].id;
        let in_port_id = &in_node.inputs[to.id.input].id;

        (self.on_disconnect)(out_port_id, in_port_id);
    }

    fn drop_inputs(&mut self, _pin: &InPin, _snarl: &mut Snarl<Node>) {}
    fn drop_outputs(&mut self, _pin: &OutPin, _snarl: &mut Snarl<Node>) {}
}

/// Main graph structure that manages nodes and connections
pub struct Graph {
    snarl: Snarl<Node>,
    node_ids: HashMap<String, NodeId>,
    unpositioned: HashSet<NodeId>,
    transform: TSTransform,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            snarl: Snarl::new(),
            node_ids: HashMap::new(),
            unpositioned: HashSet::new(),
            transform: TSTransform::IDENTITY,
        }
    }

    /// Add a node to the graph at a specific position
    pub fn add_node_at(&mut self, node: Node, pos: egui::Pos2) {
        let node_id = self.snarl.insert_node(pos, node.clone());
        self.node_ids.insert(node.id.clone(), node_id);
    }

    /// Add a node to the graph (will be auto-positioned)
    pub fn add_node(&mut self, node: Node) {
        let node_id = self.snarl.insert_node(egui::Pos2::ZERO, node.clone());
        self.node_ids.insert(node.id.clone(), node_id);
        self.unpositioned.insert(node_id);
    }

    /// Remove a node from the graph
    pub fn remove_node(&mut self, node_id: &str) {
        if let Some(snarl_node_id) = self.node_ids.remove(node_id) {
            self.snarl.remove_node(snarl_node_id);
            self.unpositioned.remove(&snarl_node_id);
        }
    }

    /// Get a node by ID
    pub fn get_node(&self, node_id: &str) -> Option<&Node> {
        self.node_ids
            .get(node_id)
            .and_then(|id| self.snarl.get_node(*id))
    }

    /// Get a mutable node by ID
    pub fn get_node_mut(&mut self, node_id: &str) -> Option<&mut Node> {
        self.node_ids
            .get(node_id)
            .and_then(|id| self.snarl.get_node_mut(*id))
    }

    /// Connect two ports
    pub fn connect(&mut self, from_node: &str, from_port: usize, to_node: &str, to_port: usize) -> bool {
        let Some(&from_id) = self.node_ids.get(from_node) else {
            return false;
        };
        let Some(&to_id) = self.node_ids.get(to_node) else {
            return false;
        };

        let out_pin = OutPinId {
            node: from_id,
            output: from_port,
        };
        let in_pin = InPinId {
            node: to_id,
            input: to_port,
        };

        self.snarl.connect(out_pin, in_pin)
    }

    /// Auto-position unpositioned nodes
    fn auto_position_nodes(&mut self, ui: &egui::Ui) {
        if self.unpositioned.is_empty() {
            return;
        }

        const NODE_SPACING: egui::Vec2 = egui::vec2(300.0, 200.0);

        let screen_pos_to_graph = |pos: egui::Pos2| {
            (pos.to_vec2() + self.transform.translation - ui.max_rect().center().to_vec2())
                / self.transform.scaling
        };

        let mut next_pos = screen_pos_to_graph(egui::pos2(
            ui.max_rect().left() + 50.0,
            ui.max_rect().top() + 150.0,
        ));

            for node_id in self.unpositioned.drain() {
            if let Some(node_info) = self.snarl.get_node_info_mut(node_id) {
                node_info.pos = egui::pos2(next_pos.x, next_pos.y);
                next_pos.y += NODE_SPACING.y;
            }
        }
    }

    /// Show the graph with callbacks for connect/disconnect
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        on_connect: &mut dyn FnMut(&str, &str),
        on_disconnect: &mut dyn FnMut(&str, &str),
    ) {
        self.auto_position_nodes(ui);

        let style = SnarlStyle {
            min_scale: Some(0.5),
            max_scale: Some(2.0),
            bg_pattern: Some(egui_snarl::ui::BackgroundPattern::grid(
                egui::vec2(50., 50.),
                0.,
            )),
            node_frame: Some(
                egui::Frame::window(ui.style())
                    .multiply_with_opacity(0.9)
                    .shadow(egui::Shadow::NONE),
            ),
            wire_layer: Some(WireLayer::AboveNodes),
            ..SnarlStyle::default()
        };

        let mut viewer = Viewer {
            on_connect,
            on_disconnect,
        };

        self.snarl.show(&mut viewer, &style, "robocraft_graph", ui);

        // Show controls overlay
        self.show_controls(ui);
    }

    fn show_controls(&self, ui: &mut egui::Ui) {
        let controls_layer_id =
            egui::LayerId::new(ui.layer_id().order, ui.layer_id().id.with("controls"));
        
        ui.scope_builder(
            egui::UiBuilder::new()
                .layer_id(controls_layer_id)
                .max_rect(ui.max_rect().shrink(3.)),
            |ui| {
                egui::Frame::canvas(ui.style())
                    .shadow(egui::Shadow::NONE)
                    .inner_margin(egui::Margin::symmetric(5, 2))
                    .stroke(ui.style().visuals.window_stroke)
                    .show(ui, |ui| {
                        egui::CollapsingHeader::new("Graph Controls")
                            .default_open(false)
                            .show_unindented(ui, |ui| {
                                ui.label(
                                    "Reset view: Double click\n\
                                     Pan: Click & drag\n\
                                     Select nodes: Secondary click & drag\n\
                                     Zoom: Ctrl + scroll\n\
                                     Connect ports: Click & drag from output to input\n\
                                     Delete connection: Click on wire",
                                );
                            });
                    });
            },
        );

        ui.ctx().set_sublayer(ui.layer_id(), controls_layer_id);
    }

    /// Clear the graph
    pub fn clear(&mut self) {
        self.snarl = Snarl::new();
        self.node_ids.clear();
        self.unpositioned.clear();
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}
