use crate::schema::Action;

pub(super) fn call_actions(actions: &[Action], value: String) {
    for action in actions {
        let _ = action.callback.call::<()>(value.clone());
    }
}
