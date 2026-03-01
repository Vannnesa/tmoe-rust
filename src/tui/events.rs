use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum UserEvent {
    MoveUp,
    MoveDown,
    Select,
    Back,
    Quit,
    Refresh,
}

pub struct EventHandler {
    _tx: mpsc::Sender<UserEvent>,
    rx: mpsc::Receiver<UserEvent>,
}

impl EventHandler {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();

        // Spawn event listener thread
        let tx_clone = tx.clone();
        thread::spawn(move || loop {
            if event::poll(Duration::from_millis(100)).ok() == Some(true) {
                if let Ok(Event::Key(key)) = event::read() {
                    if key.kind == KeyEventKind::Press {
                        let event = match key.code {
                            KeyCode::Up | KeyCode::Char('k') => UserEvent::MoveUp,
                            KeyCode::Down | KeyCode::Char('j') => UserEvent::MoveDown,
                            KeyCode::Enter => UserEvent::Select,
                            KeyCode::Esc | KeyCode::Char('q') => UserEvent::Back,
                            KeyCode::Char('c') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                                UserEvent::Quit
                            }
                            _ => UserEvent::Refresh,
                        };

                        if let Err(_) = tx_clone.send(event) {
                            break;
                        }
                    }
                }
            }
        });

        Self { _tx: tx, rx }
    }

    pub fn recv(&self) -> Option<UserEvent> {
        self.rx.try_recv().ok()
    }

    pub fn blocking_recv(&self) -> Option<UserEvent> {
        self.rx.recv().ok()
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}
