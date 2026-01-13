use gtk::prelude::*;
use gtk4 as gtk;

use crate::schema::{Component, ContainerOrientation};

pub(super) fn build_component(component: Component) -> gtk::Widget {
    match component {
        Component::Label { text, tooltip } => {
            let mut builder = gtk::Label::builder().label(text);
            if let Some(tip) = tooltip {
                builder = builder.tooltip_text(tip);
            }
            let lbl = builder.build();
            lbl.upcast()
        }
        Component::Button {
            text,
            tooltip,
            action,
        } => {
            let mut builder = gtk::Button::builder().label(text);
            if let Some(tip) = tooltip {
                builder = builder.tooltip_text(tip);
            }
            let btn = builder.build();

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
            let mut builder = gtk::Entry::builder().text(text);
            if let Some(tip) = tooltip {
                builder = builder.tooltip_text(tip);
            }
            let entry = builder.build();

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
