use crate::*;
use bevy::prelude::Vec2;
use std::error::Error;


impl<'a, T: Into<&'a str>> From<T> for Good {
    fn from(x: T) -> Self { Good { name: ustr(x.into()) } }
}

impl From<String> for City {
    fn from(s: String) -> Self {
        City{ name: ustr(&s) }
    }
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