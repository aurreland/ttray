use ratatui::{
    Frame,
    layout::Layout,
    widgets::{Block, List, ListState},
};
use system_tray::menu::MenuItem;
use tui_tree_widget::{Tree, TreeItem};

use crate::app::App;

impl App {
    pub fn render(&mut self, frame: &mut Frame) {
        let layout = Layout::new(
            self.config.direction,
            &[self.config.constraints.0, self.config.constraints.1],
        )
        .split(frame.area());

        let mut list_state = ListState::default();
        list_state.select(Some(self.app_index));

        let mut list = List::new(self.get_titles())
            .style(self.config.regular_style.0)
            .highlight_style(self.config.selected_style.0);
        if let Some(border_type) = self.config.border_type.0 {
            list = list.block(
                Block::new()
                    .borders(self.config.border.0)
                    .border_type(border_type),
            );
        }

        frame.render_stateful_widget(list, layout[0], &mut list_state);

        let maybe_menu = self.get_selected_menu();
        let tree_items = maybe_menu
            .as_ref()
            .map(|menu| menu_items_to_tree_items(&menu.submenus))
            .unwrap_or_else(Vec::new);

        let mut tree = Tree::new(&tree_items)
            .unwrap()
            .style(self.config.regular_style.1)
            .highlight_style(self.config.selected_style.1);
        if let Some(border_type) = self.config.border_type.1 {
            tree = tree.block(
                Block::new()
                    .borders(self.config.border.1)
                    .border_type(border_type),
            );
        }

        frame.render_stateful_widget(tree, layout[1], &mut self.actions_state);
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
