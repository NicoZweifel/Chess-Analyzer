use crate::Square;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

#[derive(Event)]
pub(crate) struct DragEvent {
    listener: Entity,
    delta: Vec2,
}
impl DragEvent {
    pub fn new(listener: Entity, delta: Vec2) -> Self {
        Self { listener, delta }
    }
}

impl From<ListenerInput<Pointer<Drag>>> for DragEvent {
    fn from(event: ListenerInput<Pointer<Drag>>) -> Self {
        DragEvent::new(event.listener(), event.delta)
    }
}

pub(crate) fn drag(
    mut ev_drag_end: EventReader<DragEvent>,
    mut q_squares: Query<(&Square, &mut Transform)>,
) {
    for event in ev_drag_end.read() {
        let square = q_squares.get_mut(event.listener);
        if let Ok((_, mut transform)) = square {
            transform.translation.y -= event.delta.y;
            transform.translation.x += event.delta.x;
            transform.translation.z = 0.4;
        }
    }
}
