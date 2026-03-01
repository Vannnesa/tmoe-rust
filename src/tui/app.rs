use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UIMode {
    MainMenu,
    Settings,
    Tools,
    Help,
    Confirm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    Running,
    Quit,
    Error,
}

pub struct App {
    pub mode: UIMode,
    pub state: AppState,
    pub selected_index: usize,
    pub message: String,
    pub language: Language,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    English,
    Chinese,
}

impl App {
    pub fn new() -> Self {
        Self {
            mode: UIMode::MainMenu,
            state: AppState::Running,
            selected_index: 0,
            message: String::new(),
            language: Self::detect_language(),
        }
    }

    fn detect_language() -> Language {
        let lang = std::env::var("LANG").unwrap_or_default();
        if lang.contains("zh") || lang.contains("CN") {
            Language::Chinese
        } else {
            Language::English
        }
    }

    pub fn move_down(&mut self, max: usize) {
        if self.selected_index < max - 1 {
            self.selected_index += 1;
        }
    }

    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn set_message(&mut self, msg: String) {
        self.message = msg;
    }

    pub fn clear_message(&mut self) {
        self.message.clear();
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
