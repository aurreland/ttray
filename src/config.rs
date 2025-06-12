use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction},
    style::{Color, Style},
    widgets::{BorderType, Borders},
};

use crate::event::AppEvent;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyCombo {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

impl KeyCombo {
    pub fn new(code: KeyCode, modifiers: KeyModifiers) -> Self {
        Self { code, modifiers }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub keybinds: HashMap<KeyCombo, AppEvent>,
    pub direction: Direction,
    pub constraints: (Constraint, Constraint),
    pub border: (Borders, Borders),
    pub border_type: (Option<BorderType>, Option<BorderType>),
    pub border_style: (Style, Style),
    pub regular_style: (Style, Style),
    pub selected_style: (Style, Style),
}

impl Default for Config {
    fn default() -> Config {
        use KeyCode::*;
        use KeyModifiers as KM;
        let mut binds = HashMap::new();

        binds.insert(KeyCombo::new(Esc, KM::NONE), AppEvent::Quit);
        binds.insert(KeyCombo::new(Char('q'), KM::NONE), AppEvent::Quit);
        binds.insert(KeyCombo::new(Char('c'), KM::CONTROL), AppEvent::Quit);
        binds.insert(KeyCombo::new(Char('C'), KM::CONTROL), AppEvent::Quit);

        binds.insert(KeyCombo::new(Up, KM::NONE), AppEvent::ActionPrev);
        binds.insert(KeyCombo::new(Char('k'), KM::NONE), AppEvent::ActionPrev);
        binds.insert(KeyCombo::new(Down, KM::NONE), AppEvent::ActionNext);
        binds.insert(KeyCombo::new(Char('j'), KM::NONE), AppEvent::ActionNext);

        binds.insert(KeyCombo::new(Left, KM::NONE), AppEvent::AppPrev);
        binds.insert(KeyCombo::new(Char('h'), KM::NONE), AppEvent::AppPrev);
        binds.insert(KeyCombo::new(Right, KM::NONE), AppEvent::AppNext);
        binds.insert(KeyCombo::new(Char('l'), KM::NONE), AppEvent::AppNext);

        binds.insert(
            KeyCombo::new(Char(' '), KM::NONE),
            AppEvent::ToggleActionNode,
        );
        binds.insert(KeyCombo::new(Enter, KM::NONE), AppEvent::ActivateAction);

        Config {
            keybinds: binds,
            direction: Direction::Horizontal,
            constraints: (Constraint::Percentage(33), Constraint::Fill(1)),
            border: (Borders::all(), Borders::all()),
            border_type: (None, None),
            border_style: (Style::default(), Style::default()),
            regular_style: (Style::default(), Style::default()),
            selected_style: (
                Style::default().bg(Color::Blue).fg(Color::Black),
                Style::default().bg(Color::Blue).fg(Color::Black),
            ),
        }
    }
}

impl Config {
    pub fn get(&self, key_event: &KeyEvent) -> Option<&AppEvent> {
        self.keybinds
            .get(&KeyCombo::new(key_event.code, key_event.modifiers))
    }
}
