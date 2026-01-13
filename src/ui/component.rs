use gtk::prelude::*;
use gtk4 as gtk;

use crate::{
    schema::{Component, ContainerOrientation},
    ui::action::call_actions,
};

use super::UI;

impl UI {
    pub(super) fn build_component(&self, component: Component) -> gtk::Widget {
        match component {
            Component::Label { text, tooltip } => {
                let lbl = gtk::Label::builder()
                    .tooltip_text(tooltip)
                    .label(text)
                    .build();
                lbl.upcast()
            }
            Component::Button {
                text,
                tooltip,
                actions,
            } => {
                let btn = gtk::Button::builder()
                    .tooltip_text(tooltip)
                    .label(text)
                    .build();

                if let Some(actions) = actions {
                    btn.connect_clicked(move |_| {
                        call_actions(&actions, "".to_string());
                    });
                }

                btn.upcast()
            }
            Component::Entry {
                text,
                tooltip,
                actions,
            } => {
                let entry = gtk::Entry::builder()
                    .tooltip_text(tooltip)
                    .text(text)
                    .build();

                if let Some(actions) = actions {
                    entry.connect_activate(move |entry| {
                        call_actions(&actions, entry.text().to_string());
                    });
                }

                entry.upcast()
            }
            Component::Container {
                orientation,
                children,
            } => {
                let gtk_orientation = match orientation {
                    ContainerOrientation::Vertical => gtk::Orientation::Vertical,
                    ContainerOrientation::Horizontal => gtk::Orientation::Horizontal,
                };

                let container = gtk::Box::builder().orientation(gtk_orientation).build();

                for child in children {
                    let widget = self.build_component(child);
                    container.append(&widget);
                }

                container.upcast()
            }
        }
    }
}
