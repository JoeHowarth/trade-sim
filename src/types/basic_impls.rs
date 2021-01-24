use crate::types::*;
use crate::prelude::*;

impl Clone for CityHandle {
    fn clone(&self) -> Self {
        Self {
            entity: self.entity.clone(),
            info: self.info.clone(),
        }
    }
}

impl<T: Into<String>> From<T> for Good {
    fn from(x: T) -> Self { Good { name: x.into() } }
}

impl From<Vec2> for Position {
    fn from(other: Vec2) -> Self {
        Position(other)
    }
}

impl Position {
    pub fn new(x: impl Into<Vec2>) -> Self {
        return Self(x.into());
    }
}

impl std::fmt::Display for Good {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
