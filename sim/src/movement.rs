use types::agent::GraphPosition;
use types::prelude::*;

pub fn movement(
    mut move_reader: EventReader<Movement>,
    mut pos: Query<&mut GraphPosition>,
) {
    for m@Movement{to, from, entity} in move_reader.iter() {
        if let Ok(mut pos) = pos.get_mut(*entity) {
            debug_assert!(*pos == *from, "expected Movement action 'from' to be current location of entity, instead, {:?}, {:?}", m, pos);
            *pos = *to;
        } else {
            error!("expected Movement action to be valid, {:?}", m);
        }
    }
}