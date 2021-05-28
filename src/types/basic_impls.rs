use crate::types::*;
use crate::prelude::*;
use bevy::ecs::system::QueryComponentError;

impl Clone for CityHandle {
    fn clone(&self) -> Self {
        Self {
            entity: self.entity.clone(),
            city: self.city.clone(),
        }
    }
}

impl<T: Into<String>> From<T> for Good {
    fn from(x: T) -> Self { Good { name: x.into() } }
}

impl From<Vec2> for GridPosition {
    fn from(other: Vec2) -> Self {
        GridPosition(other)
    }
}

impl GridPosition {
    pub fn new(x: impl Into<Vec2>) -> Self {
        return Self(x.into());
    }
}

impl std::fmt::Display for Good {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub fn ecs_err(e: impl Error) -> anyhow::Error {
    anyhow::Error::msg(format!("QueryError: {:?}", e))
}