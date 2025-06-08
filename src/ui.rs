use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::{List, ListState},
};
use system_tray::menu::MenuItem;
use tui_tree_widget::{Tree, TreeItem};

use crate::app::App;

impl App {
    pub fn render(&mut self, frame: &mut Frame) {
        let layout = Layout::new(
            ratatui::layout::Direction::Horizontal,
            [Constraint::Percentage(30), Constraint::Fill(1)],
        )
        .split(frame.area());

        let list = List::new(self.get_titles())
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::Black));
        let mut list_state = ListState::default();
        list_state.select(Some(self.app_index));
        frame.render_stateful_widget(list, layout[0], &mut list_state);

        if let Some(menu) = self.get_selected_menu() {
            let tree_items = menu_items_to_tree_items(&menu.submenus);
            if let Ok(mut tree) = Tree::new(&tree_items) {
                tree = tree.highlight_style(Style::default().bg(Color::Blue).fg(Color::Black));
                frame.render_stateful_widget(tree, layout[1], &mut self.actions_state);
            }
        }
    }
}

fn menu_items_to_tree_items(menus: &[MenuItem]) -> Vec<TreeItem<usize>> {
    menus
        .iter()
        .enumerate()
        .map(|(index, menu_item)| menu_item_to_tree_item(index, menu_item))
        .filter_map(|x| x)
        .collect()
}

fn menu_item_to_tree_item(id: usize, menu_item: &MenuItem) -> Option<TreeItem<usize>> {
    if menu_item.submenu.is_empty() {
        match &menu_item.label {
            Some(label) => return Some(TreeItem::new_leaf(id, label.clone())),
            None => return None,
        }
    }
    let children = menu_items_to_tree_items(&menu_item.submenu);
    let root = TreeItem::new(
        id,
        menu_item.label.clone().unwrap_or(String::from("no_label")),
        children,
    );

    root.ok()
}
