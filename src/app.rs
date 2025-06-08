use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::event::{AppEvent, Event, EventHandler};
use log::{debug, info};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
};
use system_tray::{
    client::{ActivateRequest, Client},
    item::StatusNotifierItem,
    menu::{MenuItem, TrayMenu},
};
use tui_tree_widget::TreeState;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Event handler.
    pub events: EventHandler,
    /// System Tray Client.
    pub client: Client,
    /// Focused Item Index
    pub app_index: usize,
    /// State for the Actions Tree for selected Item
    pub actions_state: TreeState<usize>,
    /// Items from [`Client`]
    items: Arc<Mutex<HashMap<String, (StatusNotifierItem, Option<TrayMenu>)>>>,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(client: Client) -> Self {
        let items = client.items();
        Self {
            running: true,
            events: EventHandler::new(client.subscribe()),
            client,
            app_index: 0,
            actions_state: TreeState::default(),
            items,
        }
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            match self.events.next().await? {
                Event::Crossterm(event) => match event {
                    crossterm::event::Event::Key(key_event) => self.handle_key_events(key_event)?,
                    _ => {}
                },
                Event::App(app_event) => match app_event {
                    AppEvent::Quit => self.quit(),
                    AppEvent::AppPrev => self.prev_app(),
                    AppEvent::AppNext => self.next_app(),
                    AppEvent::ActionPrev => _ = self.actions_state.key_up(),
                    AppEvent::ActionNext => _ = self.actions_state.key_down(),
                    AppEvent::ToggleActionNode => _ = self.actions_state.toggle_selected(),
                    AppEvent::ActivateAction => _ = self.activate_action().await,
                },
                Event::SystemTray(_) => self.update(),
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }

            KeyCode::Up | KeyCode::Char('k') => self.events.send(AppEvent::ActionPrev),
            KeyCode::Down | KeyCode::Char('j') => self.events.send(AppEvent::ActionNext),

            KeyCode::Left | KeyCode::Char('h') => self.events.send(AppEvent::AppPrev),
            KeyCode::Right | KeyCode::Char('l') => self.events.send(AppEvent::AppNext),

            KeyCode::Char(' ') => self.events.send(AppEvent::ToggleActionNode),
            KeyCode::Enter => self.events.send(AppEvent::ActivateAction),

            _ => {}
        }
        Ok(())
    }

    /// Updates Items from [`Client`]
    pub fn update(&mut self) {
        info!("Updating Client Items");
        let selected_key = self.get_selected_key();
        self.items = self.client.items();
        if let Some(key) = selected_key {
            if let Some(index) = self.get_index_from_key(&key) {
                self.app_index = index;
            }
        }
    }

    /// Returns a list of all item titles with a `Some(title)` from the items map.
    pub fn get_titles(&self) -> Vec<String> {
        let map = self.items.lock().unwrap();
        let titles = map
            .values()
            .filter_map(|(item, _)| item.title.as_ref().cloned())
            .collect();
        titles
    }

    /// Returns the key (identifier) of the currently selected item, if any.
    pub fn get_selected_key(&self) -> Option<String> {
        let map = self.items.lock().unwrap();
        map.iter()
            .filter(|(_, (item, _))| item.title.is_some())
            .nth(self.app_index)
            .map(|(k, _)| k.clone())
    }

    /// Returns the index of an item given its key (if present).
    pub fn get_index_from_key(&self, key: &str) -> Option<usize> {
        let map = self.items.lock().unwrap();
        map.iter()
            .filter(|(_, (item, _))| item.title.is_some())
            .map(|(k, _)| k)
            .position(|k| k == key)
    }

    /// Returns the currently selected item and its optional menu.
    pub fn get_selected(&self) -> Option<(StatusNotifierItem, Option<TrayMenu>)> {
        let map = self.items.lock().unwrap();
        let items = map
            .values()
            .filter(|(item, _)| item.title.is_some())
            .collect::<Vec<&(StatusNotifierItem, Option<TrayMenu>)>>();
        items
            .get(self.app_index)
            .map(|(sni, menu)| (sni.clone(), menu.clone()))
    }

    /// Returns the menu associated with the currently selected item, if available.
    pub fn get_selected_menu(&self) -> Option<TrayMenu> {
        let map = self.items.lock().unwrap();
        let items = map
            .values()
            .filter(|(item, _)| item.title.is_some())
            .collect::<Vec<&(StatusNotifierItem, Option<TrayMenu>)>>();
        items
            .get(self.app_index)
            .map(|(_, menu)| menu.clone())
            .flatten()
    }

    /// Returns the maximum valid index for selecting an item.
    pub fn max_index(&self) -> usize {
        let map = self.items.lock().unwrap();
        map.values()
            .filter(|(item, _)| item.title.is_some())
            .count()
            .saturating_sub(1)
    }

    /// Decrements the `app_index` to select the previous item, if possible.
    pub fn prev_app(&mut self) {
        if self.app_index > 0 {
            self.app_index -= 1;
        }
    }

    /// Increments the `app_index` to select the next item, if not at the end.
    pub fn next_app(&mut self) {
        let max = self.max_index();
        if self.app_index < max {
            self.app_index += 1;
        }
    }

    /// Activates the selected action from the menu, if it's a leaf node.
    /// If the node has a submenu, it toggles its expanded state instead.
    pub async fn activate_action(&mut self) -> Option<()> {
        let ids = self.actions_state.selected();
        let key = self.get_selected_key()?;
        let (sni, menu) = self.get_selected()?;
        let menu = menu?;

        let item = find_menu_by_usize(&menu, ids)?;

        if item.submenu.is_empty() {
            if let Some(path) = &sni.menu {
                let activate_request = ActivateRequest::MenuItem {
                    address: key.to_string(),
                    menu_path: path.to_string(),
                    submenu_id: item.id,
                };
                debug!("{:?}", activate_request);
                _ = self.client.activate(activate_request).await;

                _ = self
                    .client
                    .about_to_show_menuitem(key.to_string(), path.to_string(), 0)
                    .await;
            }
        } else {
            self.actions_state.toggle_selected();
        }

        Some(())
    }

    /// Sets the application state to stop running.
    pub fn quit(&mut self) {
        self.running = false;
    }
}

/// Recursively finds a [`MenuItem`] by a list of indices into the nested submenu structure.
fn find_menu_by_usize(tray_menu: &TrayMenu, ids: &[usize]) -> Option<MenuItem> {
    if ids.len() == 0 {
        return None;
    }
    let mut result: &MenuItem = tray_menu.submenus.get(ids[0])?;
    let mut submenus = &result.submenu;
    for i in ids.iter().skip(1) {
        result = submenus.get(*i)?;
        submenus = &result.submenu;
    }

    Some(result.clone())
}
