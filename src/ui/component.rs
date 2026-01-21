use gtk::prelude::*;
use gtk4 as gtk;

use crate::schema::{Align, Component, ContainerOrientation};

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
        Component::ToggleButton {
            text,
            tooltip,
            active,
            group: _,
            action,
        } => {
            let mut builder = gtk::ToggleButton::builder().label(text).active(active);
            if let Some(tip) = tooltip {
                builder = builder.tooltip_text(tip);
            }
            let toggle_btn = builder.build();

            if let Some(action) = action {
                toggle_btn.connect_toggled(move |btn| {
                    let state = if btn.is_active() { "true" } else { "false" };
                    let _ = action.callback.call::<()>(state.to_string());
                });
            }

            toggle_btn.upcast()
        }
        Component::Container {
            orientation,
            halign,
            valign,
            children,
        } => {
            let gtk_orientation = match orientation {
                ContainerOrientation::Vertical => gtk::Orientation::Vertical,
                ContainerOrientation::Horizontal => gtk::Orientation::Horizontal,
            };

            let mut builder = gtk::Box::builder().orientation(gtk_orientation);

            if let Some(align) = halign {
                let gtk_align = match align {
                    Align::Start => gtk::Align::Start,
                    Align::Center => gtk::Align::Center,
                    Align::End => gtk::Align::End,
                    Align::Fill => gtk::Align::Fill,
                };
                builder = builder.halign(gtk_align);
            }

            if let Some(align) = valign {
                let gtk_align = match align {
                    Align::Start => gtk::Align::Start,
                    Align::Center => gtk::Align::Center,
                    Align::End => gtk::Align::End,
                    Align::Fill => gtk::Align::Fill,
                };
                builder = builder.valign(gtk_align);
            }

            let container = builder.build();

            for child in children {
                let widget = build_component(child);
                container.append(&widget);
            }

            container.upcast()
        }
    }
}
