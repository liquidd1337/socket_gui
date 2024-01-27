use crate::socket::SmartSocket;
use eframe::App;
use egui::{CentralPanel, ViewportBuilder};
mod socket;

#[derive(Default)]
struct MyEguiApp {
    socket: SmartSocket,
}

impl MyEguiApp {
    fn new() -> Self {
        let socket = SmartSocket::new("Socket bathroom".to_string());
        Self { socket }
    }
}
impl App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label(self.socket.get_display_string());

            if ui.button("Socket On").clicked() {
                self.socket.socket_on();
            }
            if ui.button("Socket Off").clicked() {
                self.socket.socket_off();
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let app_name = "Socket APP";

    let app = MyEguiApp::new();

    let  window_builder = ViewportBuilder::default().with_inner_size(egui::vec2(300.0, 300.0));

    let native_options = eframe::NativeOptions {
        viewport: window_builder,
        ..Default::default()
    };

    eframe::run_native(
        app_name,
        native_options,
        Box::new(|_cc| Box::new(app) as Box<dyn App>),
    )
}
