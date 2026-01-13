use gtk::{gio::ActionEntry, prelude::*};
use gtk4 as gtk;

use super::UI;

impl UI {
    pub fn add_esc_keyboard_action(&self, window: &gtk::ApplicationWindow) {
        let action_close = ActionEntry::builder("esc_close")
            .activate(|window: &gtk::ApplicationWindow, _, _| {
                window.close();
            })
            .build();

        window.add_action_entries([action_close]);
        self.app.set_accels_for_action("win.esc_close", &["Escape"]);
    }

    pub fn add_vim_keyboard_actions(&self, window: &gtk::ApplicationWindow) {
        let action_close = ActionEntry::builder("q_close")
            .activate(|window: &gtk::ApplicationWindow, _, _| {
                window.close();
            })
            .build();

        let action_up = ActionEntry::builder("up")
            .activate(|window: &gtk::ApplicationWindow, _, _| {
                window.child_focus(gtk::DirectionType::Up);
            })
            .build();

        let action_down = ActionEntry::builder("down")
            .activate(|window: &gtk::ApplicationWindow, _, _| {
                window.child_focus(gtk::DirectionType::Down);
            })
            .build();

        let action_left = ActionEntry::builder("left")
            .activate(|window: &gtk::ApplicationWindow, _, _| {
                window.child_focus(gtk::DirectionType::Left);
            })
            .build();

        let action_right = ActionEntry::builder("right")
            .activate(|window: &gtk::ApplicationWindow, _, _| {
                window.child_focus(gtk::DirectionType::Right);
            })
            .build();

        window.add_action_entries([
            action_close,
            action_up,
            action_down,
            action_left,
            action_right,
        ]);

        self.app.set_accels_for_action("win.q_close", &["q"]);
        self.app.set_accels_for_action("win.up", &["k"]);
        self.app.set_accels_for_action("win.down", &["j"]);
        self.app.set_accels_for_action("win.left", &["h"]);
        self.app.set_accels_for_action("win.right", &["l"]);
    }
}
