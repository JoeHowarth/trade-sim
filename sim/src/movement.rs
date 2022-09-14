use crate::Failed;
use types::{agent::GraphPosition, prelude::*};

pub fn transition_movement(
    movement: &Movement,
    pos: GraphPosition,
) -> Result<GraphPosition> {
    if movement.from == pos {
        Ok(movement.to)
    } else {
        Err(anyhow!(
            "expected Movement action 'from' to be equal to position prior to movement: {:?}, {}",
            movement,
            pos
        ))
    }
}

pub fn movement(
    mut move_reader: EventReader<Movement>,
    mut pos: Query<&mut GraphPosition>,
    mut failed_movement: EventWriter<Failed<Movement>>,
) -> Result<()> {
    for m in move_reader.iter() {
        let _ = pos
            .get_mut(m.entity)
            .context(anyhow!(
                "expected Movement action to be valid, {:?}",
                &m
            ))
            .map(|mut pos| {
                transition_movement(m, pos.clone())
                    .map(|new_pos| *pos = new_pos)
            })
            .map_err(|e| {
                failed_movement.send(Failed(m.clone()));
                e
            })?;
    }
    Ok(())
}
