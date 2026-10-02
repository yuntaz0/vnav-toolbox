use eframe::egui::{self};
use nalgebra::{Matrix3, Rotation3, UnitQuaternion};

struct RotationConverter {
    rot: UnitQuaternion<f64>,
    matrix: String,
    roll_deg: f64,
    pitch_deg: f64,
    yaw_deg: f64,
}

fn format_matrix(rotation: &UnitQuaternion<f64>) -> String {
    let r = rotation.to_rotation_matrix();
    let m = r.matrix();

    format!(
        "[\n  [{:.6}, {:.6}, {:.6}],\n  [{:.6}, {:.6}, {:.6}],\n  [{:.6}, {:.6}, {:.6}]\n]",
        m[(0, 0)],
        m[(0, 1)],
        m[(0, 2)],
        m[(1, 0)],
        m[(1, 1)],
        m[(1, 2)],
        m[(2, 0)],
        m[(2, 1)],
        m[(2, 2)],
    )
}

impl Default for RotationConverter {
    fn default() -> Self {
        Self {
            rot: UnitQuaternion::identity(),
            matrix: String::from("[\n  [1.0, 0.0, 0.0],\n  [0.0, 1.0, 0.0],\n  [0.0, 0.0, 1.0]\n]"),
            roll_deg: 0.0,
            pitch_deg: 0.0,
            yaw_deg: 0.0,
        }
    }
}

impl RotationConverter {
    fn show_rpy(&mut self, ui: &mut egui::Ui) {
        ui.label("RPY (degrees)");

        let roll_changed = ui
            .add(
                egui::DragValue::new(&mut self.roll_deg)
                    .prefix("Roll: ")
                    .fixed_decimals(4)
                    .speed(0.1),
            )
            .changed();

        let pitch_changed = ui
            .add(
                egui::DragValue::new(&mut self.pitch_deg)
                    .prefix("Pitch: ")
                    .fixed_decimals(4)
                    .speed(0.1),
            )
            .changed();

        let yaw_changed = ui
            .add(
                egui::DragValue::new(&mut self.yaw_deg)
                    .prefix("Yaw: ")
                    .fixed_decimals(4)
                    .speed(0.1),
            )
            .changed();

        if roll_changed || pitch_changed || yaw_changed {
            self.rot = UnitQuaternion::from_euler_angles(
                self.roll_deg.to_radians(),
                self.pitch_deg.to_radians(),
                self.yaw_deg.to_radians(),
            );
            self.matrix = format_matrix(&self.rot);
        }
    }

    fn show_quaternion(&mut self, ui: &mut egui::Ui) {
        let q = self.rot.quaternion();

        let mut w = q.w;
        let mut x = q.i;
        let mut y = q.j;
        let mut z = q.k;

        ui.label("Quaternion");

        let w_changed = ui
            .add(
                egui::DragValue::new(&mut w)
                    .prefix("w: ")
                    .fixed_decimals(4)
                    .speed(0.01),
            )
            .changed();
        let x_changed = ui
            .add(
                egui::DragValue::new(&mut x)
                    .prefix("x: ")
                    .fixed_decimals(4)
                    .speed(0.01),
            )
            .changed();
        let y_changed = ui
            .add(
                egui::DragValue::new(&mut y)
                    .prefix("y: ")
                    .fixed_decimals(4)
                    .speed(0.01),
            )
            .changed();
        let z_changed = ui
            .add(
                egui::DragValue::new(&mut z)
                    .prefix("z: ")
                    .fixed_decimals(4)
                    .speed(0.01),
            )
            .changed();

        if w_changed || x_changed || y_changed || z_changed {
            let q = nalgebra::Quaternion::new(w, x, y, z);

            if let Some(unit_q) = UnitQuaternion::try_new(q, 1e-12) {
                self.rot = unit_q;
            }
            self.matrix = format_matrix(&self.rot);
        }
    }

    fn show_matrix(&mut self, ui: &mut egui::Ui) {
        ui.label("Rotation Matrix");

        let response = ui.add(
            egui::TextEdit::multiline(&mut self.matrix)
                .font(egui::TextStyle::Monospace)
                .desired_rows(5)
                .desired_width(300.0),
        );

        if response.changed()
            && let Ok(rows) = serde_json::from_str::<[[f64; 3]; 3]>(&self.matrix)
        {
            let matrix = Matrix3::new(
                rows[0][0], rows[0][1], rows[0][2], rows[1][0], rows[1][1], rows[1][2], rows[2][0],
                rows[2][1], rows[2][2],
            );

            let rotation = Rotation3::from_matrix_unchecked(matrix);

            self.rot = UnitQuaternion::from_rotation_matrix(&rotation);
            self.matrix = format_matrix(&self.rot);
        }
    }

    fn show_axis_angle(&mut self, ui: &mut egui::Ui) {
        ui.label("axis-angle(READ-ONLY)");
        let angle = self.rot.angle().to_degrees();
        if let Some(axis) = self.rot.axis() {
            let axis = axis.into_inner();
            ui.label(format!("Angle: {angle:.3}"));
            ui.label(format!("Axis: [{:.4}, {:.4}, {:.4}]", axis.x, axis.y, axis.z));
        } else {
            ui.label("identity rotation(axis undefined)");
        }
    }

    fn reset_btn(&mut self, ui: &mut egui::Ui) {
        if ui.button("Reset").clicked() {
            *self = Self::default();
        }
    }
}

impl eframe::App for RotationConverter {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Rotation Converter");

            ui.separator();
            self.show_rpy(ui);

            ui.separator();
            self.show_quaternion(ui);

            ui.separator();
            self.show_matrix(ui);

            ui.separator();
            self.show_axis_angle(ui);

            ui.separator();
            self.reset_btn(ui);
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Rotation Converter",
        options,
        Box::new(|_cc| Ok(Box::new(RotationConverter::default()))),
    )
}
