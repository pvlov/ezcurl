use std::process::Command;

use eframe::{run_native, App, CreationContext, NativeOptions};
use egui::{CentralPanel, ComboBox, ScrollArea, SidePanel, TextEdit, TopBottomPanel};
use serde_json::json;

const ALL_REQUEST_TYPES: [(RequestType, &str); 2] =
    [(RequestType::Get, "GET"), (RequestType::Post, "POST")];

const CLIPBOARD_EMOJI: &str = "📋";
const WASTEBASKET_EMOJI: &str = "X";

#[derive(Debug, Default, PartialEq)]
enum RequestType {
    #[default]
    Get,
    Post,
}
#[derive(Debug, PartialEq, PartialOrd)]
struct JSONField {
    key: String,
    value: String,
    is_multi_line: bool,
}

#[derive(Default)]
struct Ezcurl {
    width: f32,
    height: f32,
    method: RequestType,
    url: String,
    input: Vec<JSONField>,
    output: String,
}

impl App for Ezcurl {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let rect = ctx.screen_rect();
        self.width = rect.width();
        self.height = rect.height();
        self.draw(ctx);
    }
}

impl Ezcurl {
    fn new(_cc: &CreationContext<'_>) -> Self {
        Ezcurl {
            url: String::from("http://localhost:8080"),
            output: String::from("No output yet!"),
            ..Default::default()
        }
    }

    fn draw(&mut self, ctx: &egui::Context) {
        TopBottomPanel::top("TopPanel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Method:");

                ComboBox::from_label("")
                    .selected_text(format!("{:?}", self.method))
                    .show_ui(ui, |ui| {
                        for req_type in ALL_REQUEST_TYPES {
                            ui.selectable_value(&mut self.method, req_type.0, req_type.1);
                        }
                    });
                ui.heading("URL: ");
                let _ = ui.add(TextEdit::singleline(&mut self.url));

                if ui.button("Make Request").clicked() {
                    self.make_request();
                }

                if ui.button(CLIPBOARD_EMOJI).clicked() {
                    ui.output_mut(|out| {
                        out.copied_text = self.output.clone();
                    });
                }
            })
        });

        SidePanel::right("right_panel").show(ctx, |ui| {
            ui.label(&self.output);
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Add single-line field").clicked() {
                    self.input.push(JSONField {
                        key: String::from(""),
                        value: String::from(""),
                        is_multi_line: false,
                    });
                }

                if ui.button("Add multi-line field").clicked() {
                    self.input.push(JSONField {
                        key: String::from(""),
                        value: String::from(""),
                        is_multi_line: true,
                    })
                }
            });

            ScrollArea::vertical().show(ui, |ui| {
                let mut elem_to_remove = None;
                for (i, entry) in self.input.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(&mut entry.key);
                        if entry.is_multi_line {
                            ui.text_edit_multiline(&mut entry.value);
                        } else {
                            ui.text_edit_singleline(&mut entry.value);
                        }

                        if ui.button(WASTEBASKET_EMOJI).clicked() {
                            elem_to_remove = Some(i);
                        }
                    });
                }
                if let Some(index) = elem_to_remove {
                    self.input.remove(index);
                }
            });
        });
    }

    fn make_request(&mut self) {
        let payload = json!(&self
            .input
            .iter()
            .map(|json_field| format!("{}:{}", json_field.key, json_field.value))
            .collect::<Vec<_>>()
            .join("\n"));

        self.output = Command::new("curl")
            .arg("-X")
            .arg(match self.method {
                RequestType::Get => "GET",
                RequestType::Post => "POST",
            })
            .arg(match self.input.is_empty() {
                true => "",
                false => "-H Content-Type: application/json",
            })
            .arg(match self.input.is_empty() {
                true => String::from(""),
                false => format!("-d {}", payload),
            })
            .arg(&self.url)
            .output()
            .map(|out| String::from_utf8_lossy(&out.stdout).to_string())
            .unwrap_or("Something went wrong while executing curl".to_string());
    }
}

fn main() {
    let _ = Command::new("curl").arg("--help").output().expect(
        "curl is required for this application, please install it if you want to use this tool.",
    );

    let native_options = NativeOptions::default();
    run_native(
        "EZCurl",
        native_options,
        Box::new(|cc| Box::new(Ezcurl::new(cc))),
    )
    .expect("Failed to start Application")
}
