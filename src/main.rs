use std::process::Command;

use eframe::{run_native, App, CreationContext, NativeOptions};
use egui::{
    ahash::HashMap, CentralPanel, ComboBox, ScrollArea, SidePanel, TextEdit, TopBottomPanel,
};

const ALL_REQUEST_TYPES: [(RequestType, &str); 2] =
    [(RequestType::GET, "GET"), (RequestType::POST, "POST")];

#[derive(Debug, Default, PartialEq)]
enum RequestType {
    #[default]
    GET,
    POST,
}

#[derive(Default)]
struct Ezcurl {
    width: f32,
    height: f32,
    method: RequestType,
    url: String,
    input: HashMap<String, String>,
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
            })
        });

        SidePanel::right("right_panel").show(ctx, |ui| {
            ui.label(self.output.clone());
        });

        CentralPanel::default().show(ctx, |ui| {});
    }

    fn make_request(&mut self) {
        self.output = Command::new("curl")
            .arg("-X")
            .arg(match self.method {
                RequestType::GET => "GET",
                RequestType::POST => "POST",
            })
            .output()
            .map(|out| String::from_utf8_lossy(&out.stdout).to_string())
            .map(|out_string| serde_json::to_string_pretty(&out_string).unwrap())
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
