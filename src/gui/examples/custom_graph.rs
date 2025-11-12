use eframe::egui;
use robocraft_gui::{Graph, Node, Port};

/// Example showing how to create a custom graph programmatically
fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Custom Graph Example")
            .with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Custom Graph Example",
        native_options,
        Box::new(|cc| Ok(Box::new(CustomGraphApp::new(cc)))),
    )
}

struct CustomGraphApp {
    graph: Graph,
}

impl CustomGraphApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut graph = Graph::new();

        // Create a sensor node
        let sensor = Node::new("sensor1", "Camera Sensor")
            .with_output(Port::new("image", "Image Stream", egui::Color32::from_rgb(100, 200, 255)))
            .with_output(Port::new("depth", "Depth Data", egui::Color32::from_rgb(255, 150, 100)))
            .with_metadata("type", "sensor")
            .with_metadata("fps", "30")
            .with_metadata("resolution", "1920x1080");

        // Create an image processor node
        let processor = Node::new("processor1", "Image Processor")
            .with_input(Port::new("input", "Raw Image", egui::Color32::from_rgb(100, 200, 255)))
            .with_output(Port::new("filtered", "Filtered", egui::Color32::from_rgb(150, 255, 150)))
            .with_output(Port::new("edges", "Edge Detection", egui::Color32::from_rgb(255, 255, 100)))
            .with_metadata("type", "processor")
            .with_metadata("algorithm", "Canny Edge Detection");

        // Create an AI/ML node
        let ai_node = Node::new("ai1", "Object Detector")
            .with_input(Port::new("image_in", "Image Input", egui::Color32::from_rgb(150, 255, 150)))
            .with_output(Port::new("detections", "Detections", egui::Color32::from_rgb(255, 100, 100)))
            .with_metadata("type", "ai")
            .with_metadata("model", "YOLOv8")
            .with_metadata("confidence", "0.85");

        // Create a control node
        let controller = Node::new("controller1", "Robot Controller")
            .with_input(Port::new("sensor_data", "Sensor Data", egui::Color32::from_rgb(255, 100, 100)))
            .with_output(Port::new("motor_cmd", "Motor Commands", egui::Color32::from_rgb(100, 255, 200)))
            .with_metadata("type", "controller")
            .with_metadata("mode", "autonomous");

        // Create an actuator node
        let actuator = Node::new("actuator1", "Motor Driver")
            .with_input(Port::new("commands", "Control Input", egui::Color32::from_rgb(100, 255, 200)))
            .with_metadata("type", "actuator")
            .with_metadata("motors", "4x DC Motors");

        // Add all nodes to the graph
        graph.add_node(sensor);
        graph.add_node(processor);
        graph.add_node(ai_node);
        graph.add_node(controller);
        graph.add_node(actuator);

        Self { graph }
    }
}

impl eframe::App for CustomGraphApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.heading("Custom Robot Control Graph");
                ui.separator();
                ui.label("Connect nodes by dragging from outputs to inputs");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.graph.show(
                ui,
                &mut |from, to| {
                    println!("Connected: {} -> {}", from, to);
                },
                &mut |from, to| {
                    println!("Disconnected: {} -> {}", from, to);
                },
            );
        });
    }
}
