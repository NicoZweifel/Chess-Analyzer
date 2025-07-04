use crate::Square;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

#[derive(Event)]
pub(crate) struct DragEndEvent {
    listener: Entity,
    distance: Vec2,
}

impl DragEndEvent {
    pub fn new(listener: Entity, distance: Vec2) -> Self {
        Self { listener, distance }
    }
}

impl From<ListenerInput<Pointer<DragEnd>>> for DragEndEvent {
    fn from(event: ListenerInput<Pointer<DragEnd>>) -> Self {
        DragEndEvent::new(event.listener(), event.distance)
    }
}

pub(crate) fn drag_end(
    mut ev_drag_end: EventReader<DragEndEvent>,
    mut q_squares: Query<(&Square, &mut Transform)>,
) {
    for event in ev_drag_end.read() {
        let square = q_squares.get_mut(event.listener);
        if let Ok((_, mut transform)) = square {
            transform.translation.y += event.distance.y;
            transform.translation.x -= event.distance.x;
            transform.translation.z = 0.2;
        }
    }
}
