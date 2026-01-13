use gtk::prelude::*;
use gtk4 as gtk;

use crate::schema::{Component, ContainerOrientation};

pub(super) fn build_component(component: Component) -> gtk::Widget {
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
            action,
        } => {
            let btn = gtk::Button::builder()
                .tooltip_text(tooltip)
                .label(text)
                .build();

            if let Some(action) = action {
                btn.connect_clicked(move |_| {
                    let _ = action.callback.call::<()>("".to_string());
                });
            }

            btn.upcast()
        }
        Component::Entry {
            text,
            tooltip,
            action,
        } => {
            let entry = gtk::Entry::builder()
                .tooltip_text(tooltip)
                .text(text)
                .build();

            if let Some(action) = action {
                entry.connect_activate(move |entry| {
                    let _ = action.callback.call::<()>(entry.text().to_string());
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
                let widget = build_component(child);
                container.append(&widget);
            }

            container.upcast()
        }
    }
}
