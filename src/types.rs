// types for trade-sim
use crate::prelude::*;
use std::fmt::Formatter;


#[derive(Debug, From, Clone, Eq, PartialEq, Hash)]
pub struct City {
    pub name: String,
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct CityHandle {
    pub entity: Entity,
    pub info: City,
}

impl Clone for CityHandle {
    fn clone(&self) -> Self {
        Self {
            entity: self.entity.clone(),
            info: self.info.clone(),
        }
    }
}

#[derive(Deref, Debug, Clone)]
pub struct LinkedCities(pub Vec<CityHandle>);

#[derive(Deref, Eq, Clone, Debug, PartialEq, Hash)]
pub struct Good {
    pub name: String,
}

#[derive(Deref, Eq, Clone, Debug, PartialEq, Hash)]
pub struct GoodHandle {
    pub read: Arc<Good>,
}

impl From<Good> for GoodHandle {
    fn from(g: Good) -> Self { GoodHandle { read: Arc::new(g) } }
}

impl<T: Into<String>> From<T> for Good {
    fn from(x: T) -> Self { Good { name: x.into() } }
}

pub struct Agent;

///////////// Market Types ///////////


////////////// Display Types ///////////////

#[derive(Deref, Debug, Clone, Copy)]
pub struct Position(pub Vec2);

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