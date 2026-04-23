use eframe::egui;

#[derive(Debug, Clone)]
pub enum Intent {
    Arm,
    Disarm,
    Fire,
    Cancel,
}

#[derive(Debug)]
pub enum Command {
    Fire,
    Cancel,
}

#[derive(Debug, Default)]
pub struct AppState {
    pub armed: bool,
    pub firing: bool,
    pub last_action: Option<String>,
}

pub fn apply_intent(state: &mut AppState, intent: &Intent) -> Option<Command> {
    match intent {
        Intent::Arm => {
            state.armed = true;
            state.last_action = Some("Armed".into());
            None
        }
        Intent::Disarm => {
            state.armed = false;
            state.firing = false;
            state.last_action = Some("Disarmed".into());
            None
        }
        Intent::Fire => {
            if state.armed {
                state.firing = true;
                state.last_action = Some("Fired".into());
                Some(Command::Fire)
            } else {
                state.last_action = Some("Blocked: Not armed".into());
                None
            }
        }
        Intent::Cancel => {
            state.firing = false;
            state.last_action = Some("Cancelled".into());
            Some(Command::Cancel)
        }
    }
}

pub fn execute_effect(cmd: Command) {
    match cmd {
        Command::Fire => {
            println!("EFFECT: Sending FIRE command");
        }
        Command::Cancel => {
            println!("EFFECT: Sending CANCEL command");
        }
    }
}

pub fn render_ui(ui: &mut egui::Ui, state: &AppState) -> Vec<Intent> {
    let mut intents = Vec::new();

    ui.heading("Propulsion Control");
    ui.label(format!("Armed: {}", state.armed));
    ui.label(format!("Firing: {}", state.firing));

    if let Some(action) = &state.last_action {
        ui.label(format!("Last action: {}", action));
    }

    ui.separator();

    // Arm / Disarm
    if !state.armed {
        if ui.button("Arm").clicked() {
            intents.push(Intent::Arm);
        }
    } else if ui.button("Disarm").clicked() {
        intents.push(Intent::Disarm);
    }

    ui.separator();

    // Commands
    if ui.button("Fire").clicked() {
        intents.push(Intent::Fire);
    }
    if ui.button("Cancel").clicked() {
        intents.push(Intent::Cancel);
    }

    intents
}

pub struct App {
    pub state: AppState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            state: AppState {
                armed: false,
                firing: false,
                last_action: None,
            },
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let intents = render_ui(ui, &self.state);
        for intent in intents {
            if let Some(cmd) = apply_intent(&mut self.state, &intent) {
                execute_effect(cmd);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fire_is_blocked_when_not_armed() {
        let mut state = AppState::default();
        let cmd = apply_intent(&mut state, &Intent::Fire);

        assert!(cmd.is_none());
        assert!(!state.armed);
        assert!(!state.firing);
        assert_eq!(state.last_action.as_deref(), Some("Blocked: Not armed"));
    }

    #[test]
    fn arm_then_fire_emits_command_and_sets_state() {
        let mut state = AppState::default();

        assert!(apply_intent(&mut state, &Intent::Arm).is_none());
        assert!(state.armed);
        assert!(!state.firing);
        assert_eq!(state.last_action.as_deref(), Some("Armed"));

        let cmd = apply_intent(&mut state, &Intent::Fire);
        assert!(matches!(cmd, Some(Command::Fire)));
        assert!(state.firing);
        assert_eq!(state.last_action.as_deref(), Some("Fired"));
    }
}

