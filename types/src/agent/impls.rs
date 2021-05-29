use crate::{
    prelude::*,
    agent::*,
    *,
};

impl GraphPosition {
    pub fn city(&self) -> Option<CityHandle> {
        match self {
            GraphPosition::Node(city) => Some(*city),
            GraphPosition::Edge(_, _) => None,
        }
    }
    pub fn edge(&self) -> Option<(CityHandle, CityHandle)> {
        match self {
            GraphPosition::Node(_) => None,
            GraphPosition::Edge(to, from) => Some((*to, *from)),
        }
    }
}