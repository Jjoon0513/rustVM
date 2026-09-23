mod codeeditor;

use eframe::egui;
use eframe::egui::Ui;
use egui_code_editor::{
    self, CodeEditor, ColorTheme, Completer, Syntax, highlighting::Token, push_dropped_files,
};

pub struct RVUI {
    left: bool,
    right: bool,
    top: bool,
    bottom: bool,
    rvacode: String,
}

impl Default for RVUI {
    fn default() -> Self {
        Self {
            left: true,
            right: true,
            top: true,
            bottom: true,
            rvacode: "".to_string(),
        }
    }
}

impl eframe::App for RVUI {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        let Self {
            left,
            right,
            top,
            bottom,
            rvacode,
        } = self;

        egui::Panel::top("top_panel")
            .resizable(true)
            .min_size(32.0)
            .show_collapsible(ui, top, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Expandable Upper Panel");
                    });
                });
            });

        egui::Panel::left("left_panel")
            .resizable(true)
            .default_size(150.0)
            .size_range(80.0..=200.0)
            .show_collapsible(ui, left, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Left Panel");
                });
                egui::ScrollArea::vertical().show(ui, |ui| {});
            });

        egui::Panel::right("right_panel")
            .resizable(true)
            .default_size(150.0)
            .size_range(80.0..=200.0)
            .show_collapsible(ui, right, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Right Panel");
                });
                egui::ScrollArea::vertical().show(ui, |ui| {});
            });

        egui::CentralPanel::default().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Central Panel");
            });

            ui.horizontal(|ui| {
                ui.label("Panel toggles:");
                ui.toggle_value(left, "⬅");
                ui.toggle_value(top, "⬆");
                ui.toggle_value(bottom, "⬇");
                ui.toggle_value(right, "➡");
            });

            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                CodeEditor::default()
                    .id_source("code editor")
                    .with_rows(12)
                    .with_fontsize(14.0)
                    .with_theme(ColorTheme::GRUVBOX)
                    .with_numlines(true)
                    .with_clickable_links(true)
                    .show(ui, rvacode, &Syntax::shell());
            });
        });
    }
}
