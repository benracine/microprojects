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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SystemState {
    #[default]
    Pending,
    Armed,
    Firing,
}

impl SystemState {
    fn as_str(self) -> &'static str {
        match self {
            SystemState::Pending => "Pending",
            SystemState::Armed => "Armed",
            SystemState::Firing => "Firing",
        }
    }
}

#[derive(Debug, Default)]
pub struct AppState {
    pub system_state: SystemState,
    pub last_action: Option<String>,
}

pub fn apply_intent(state: &mut AppState, intent: &Intent) -> Option<Command> {
    // Transition matrix as match(current_state, intent).
    let (next_state, last_action, command) = match (state.system_state, intent) {
        (SystemState::Pending, Intent::Arm) => (SystemState::Armed, "Armed", None),
        (SystemState::Pending, Intent::Disarm) => (SystemState::Pending, "Already disarmed", None),
        (SystemState::Pending, Intent::Fire) => (SystemState::Pending, "Blocked: Not armed", None),
        (SystemState::Pending, Intent::Cancel) => (SystemState::Pending, "Cancelled", Some(Command::Cancel)),

        (SystemState::Armed, Intent::Arm) => (SystemState::Armed, "Already armed", None),
        (SystemState::Armed, Intent::Disarm) => (SystemState::Pending, "Disarmed", None),
        (SystemState::Armed, Intent::Fire) => (SystemState::Firing, "Fired", Some(Command::Fire)),
        (SystemState::Armed, Intent::Cancel) => (SystemState::Armed, "Cancelled", Some(Command::Cancel)),

        (SystemState::Firing, Intent::Arm) => (SystemState::Firing, "Already firing", None),
        (SystemState::Firing, Intent::Disarm) => (SystemState::Pending, "Disarmed", None),
        (SystemState::Firing, Intent::Fire) => (SystemState::Firing, "Already firing", None),
        (SystemState::Firing, Intent::Cancel) => (SystemState::Armed, "Cancelled", Some(Command::Cancel)),
    };

    state.system_state = next_state;
    state.last_action = Some(last_action.into());
    command
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
    ui.label(format!("Status: {}", state.system_state.as_str()));

    if let Some(action) = &state.last_action {
        ui.label(format!("Last action: {}", action));
    }

    ui.separator();

    // Arm / Disarm
    if matches!(state.system_state, SystemState::Pending) {
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
                system_state: SystemState::Pending,
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
        assert_eq!(state.system_state, SystemState::Pending);
        assert_eq!(state.last_action.as_deref(), Some("Blocked: Not armed"));
    }

    #[test]
    fn arm_then_fire_emits_command_and_sets_state() {
        let mut state = AppState::default();

        assert!(apply_intent(&mut state, &Intent::Arm).is_none());
        assert_eq!(state.system_state, SystemState::Armed);
        assert_eq!(state.last_action.as_deref(), Some("Armed"));

        let cmd = apply_intent(&mut state, &Intent::Fire);
        assert!(matches!(cmd, Some(Command::Fire)));
        assert_eq!(state.system_state, SystemState::Firing);
        assert_eq!(state.last_action.as_deref(), Some("Fired"));
    }
}

