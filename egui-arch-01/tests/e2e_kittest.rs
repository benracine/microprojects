use egui_arch_01::App;
use egui_kittest::{kittest::Queryable, Harness};

#[test]
fn e2e_arm_then_fire_updates_ui_state() {
    let mut harness = Harness::new_eframe(|_cc| App::default());

    harness.get_by_label("Arm").click();
    harness.run();
    harness.get_by_label_contains("Status: Armed");

    harness.get_by_label("Fire").click();
    harness.run();

    harness.get_by_label_contains("Status: Firing");
    harness.get_by_label_contains("Last action: Fired");
}

