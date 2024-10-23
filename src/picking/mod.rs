use bevy::{
    app::Update,
    prelude::{App, IntoSystemConfigs, Plugin},
};

mod drag;
mod drag_end;
mod drop;
mod select;

pub(crate) use drag::*;
pub(crate) use drag_end::*;
pub(crate) use drop::*;
pub(crate) use select::*;

pub struct PickingPlugin;

impl Plugin for PickingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DropEvent>()
            .add_event::<SelectEvent>()
            .add_event::<DragEvent>()
            .add_event::<DragEndEvent>()
            .add_systems(
                Update,
                (drop.before(drag_end), select, drag, drag_end.after(drop)),
            );
    }
}
