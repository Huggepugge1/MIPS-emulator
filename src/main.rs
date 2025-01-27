use eframe::egui;

mod cpu;
mod memory;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };
    eframe::run_native(
        "Hello World",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {
    cpu: cpu::CPU,
    memory: memory::Memory,
}

impl eframe::App for MyApp {
    fn update(&mut self, _ctx: &egui::Context, _cc: &mut eframe::Frame) {
        egui::CentralPanel::default().show(_ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Load").clicked() {
                    self.cpu.load_program(
                        &mut self.memory,
                        vec![
                            0x00, 0x00, 0x00, 0x0c, // syscall
                            0x00, 0x00, 0x00, 0x0c, // syscall
                        ],
                    );
                }
                if ui.button("Run").clicked() {
                    self.cpu.run(&self.memory);
                }
            });

            // Registers
            egui::Grid::new("registers")
                .striped(true)
                .spacing(egui::vec2(10.0, 10.0))
                .num_columns(3)
                .show(ui, |ui| {
                    ui.label("Name");
                    ui.label("Decimal Value");
                    ui.label("Hex Value");
                    ui.end_row();
                    for (name, register) in self.cpu.registers.clone().into_iter() {
                        ui.label(name);
                        ui.label(register.to_string());
                        ui.label(format!("0x{:08x}", register));
                        ui.end_row();
                    }
                });
        });
    }
}
