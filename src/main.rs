mod test;
mod ui;
mod vm;
mod vm_core;
mod vm_debuger;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "My App",
        options,
        Box::new(|_cc| Ok(Box::new(ui::RVUI::default()))),
    )
}
