use eframe::egui;

use std::env::args;

mod cpu;
mod elf;
mod memory;

fn main() -> eframe::Result {
    let binary = args().nth(1).expect("No binary provided");
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        vsync: false,
        ..Default::default()
    };

    let mut app = MyApp {
        binary: std::fs::read(binary).expect("Failed to read binary"),
        ..Default::default()
    };

    let entry_point = app.memory.load_elf(&app.binary);
    app.cpu.registers.pc = entry_point;
    // app.cpu.run(&mut app.memory);
    // std::process::exit(0);

    eframe::run_native("Hello World", options, Box::new(|_cc| Ok(Box::new(app))))
}

#[derive(Default)]
struct MyApp {
    cpu: cpu::CPU,
    memory: memory::Memory,

    binary: Vec<u8>,

    running: bool,
}

impl eframe::App for MyApp {
    fn update(&mut self, _ctx: &egui::Context, _cc: &mut eframe::Frame) {
        _ctx.request_repaint_after(std::time::Duration::from_millis(0));
        if self.running {
            for _ in 0..100 {
                self.cpu.step(&mut self.memory);
            }
        }
        egui::CentralPanel::default().show(_ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Run").clicked() {
                    self.running = true;
                }

                if ui.button("step").clicked() {
                    self.cpu.step(&mut self.memory);
                }
            });

            ui.separator();
            ui.horizontal_top(|ui| {
                self.draw_registers(ui);
                self.draw_memory(ui);
                ui.vertical(|ui| {
                    ui.heading("Status");
                    if self.cpu.exception != cpu::Exception::None {
                        ui.label(format!("{}", self.cpu.exception));
                    } else if self.cpu.halted {
                        ui.label("Halted");
                    } else {
                        ui.label("Running");
                    }
                });
            });
        });
    }
}

impl MyApp {
    fn draw_registers(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("Registers");
            egui::Grid::new("registers")
                .striped(true)
                .spacing(egui::vec2(3.0, 8.0))
                .num_columns(3)
                .show(ui, |ui| {
                    ui.label("Name");
                    ui.label("Decimal Value");
                    ui.label("Hex Value");
                    ui.allocate_space(egui::vec2(0.0, 0.0));
                    ui.end_row();
                    for (name, register) in self.cpu.registers.clone().into_iter() {
                        ui.label(name);
                        ui.label(register.to_string());
                        ui.label(egui::RichText::new(format!("0x{:08x}", register)).monospace());
                        ui.allocate_space(egui::vec2(0.0, 0.0));
                        ui.end_row();
                    }
                });
        });
    }

    fn draw_memory(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            self.draw_text_segment(ui);
            self.draw_data_segment(ui);
            self.draw_heap_segment(ui);
        });
    }

    fn draw_text_segment(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("Text Segment");
            egui::Grid::new("Text segment")
                .striped(true)
                .spacing(egui::vec2(10.0, 8.0))
                .num_columns(2)
                .show(ui, |ui| {
                    ui.label("Address");
                    ui.label("Decimal Value");
                    ui.label("Hex Value");
                    ui.allocate_space(egui::vec2(0.0, 0.0));
                    ui.end_row();
                    for i in 0..self.memory.text.len() / 4 {
                        let address = self.memory.text_address + (i * 4) as u32;
                        let value = self.memory.read_word(address);
                        ui.label(egui::RichText::new(format!("0x{:08x}", address)).monospace());
                        ui.label(egui::RichText::new(format!("{}", value)));
                        ui.label(egui::RichText::new(format!("0x{:08x}", value)).monospace());
                        ui.allocate_space(egui::vec2(0.0, 0.0));
                        ui.end_row();
                    }
                });
        });
    }

    fn draw_data_segment(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("Data Segment");
            egui::Grid::new("Data segment")
                .striped(true)
                .spacing(egui::vec2(10.0, 8.0))
                .num_columns(2)
                .show(ui, |ui| {
                    ui.label("Address");
                    ui.label("Decimal Value");
                    ui.label("Hex Value");
                    ui.allocate_space(egui::vec2(0.0, 0.0));
                    ui.end_row();
                    for i in 0..self.memory.data.len() / 4 {
                        let address = self.memory.data_address + (i * 4) as u32;
                        let value = self.memory.read_word(address);
                        ui.label(egui::RichText::new(format!("0x{:08x}", address)).monospace());
                        ui.label(egui::RichText::new(format!("{}", value)));
                        ui.label(egui::RichText::new(format!("0x{:08x}", value)).monospace());
                        ui.allocate_space(egui::vec2(0.0, 0.0));
                        ui.end_row();
                    }
                });
        });
    }

    fn draw_heap_segment(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("Heap Segment");
            egui::Grid::new("Heap segment")
                .striped(true)
                .spacing(egui::vec2(10.0, 8.0))
                .num_columns(2)
                .show(ui, |ui| {
                    ui.label("Address");
                    ui.label("Decimal Value");
                    ui.label("Hex Value");
                    ui.allocate_space(egui::vec2(0.0, 0.0));
                    ui.end_row();
                    for i in 0..self.memory.heap.len() / 4 {
                        let address = 0x10400000 + i as u32 * 4;
                        let value = self.memory.read_word(address);
                        ui.label(egui::RichText::new(format!("0x{:08x}", address)).monospace());
                        ui.label(egui::RichText::new(format!("{}", value)));
                        ui.label(egui::RichText::new(format!("0x{:08x}", value)).monospace());
                        ui.allocate_space(egui::vec2(0.0, 0.0));
                        ui.end_row();
                    }
                });
        });
    }
}
