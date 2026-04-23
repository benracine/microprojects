use egui_arch_01::App;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Egui Clean Architecture",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(App::default()))),
    )
}
