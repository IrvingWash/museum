use eframe::{NativeOptions, egui};

pub fn run() {
    eframe::run_native(
        "museum",
        NativeOptions::default(),
        Box::new(|_| Ok(Box::<App>::default())),
    )
    .expect("Failed to initialize egui");
}

#[derive(Default)]
struct App {}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {});
    }
}
