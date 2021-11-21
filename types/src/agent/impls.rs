use std::fmt::write;
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
    pub fn city_res(&self) -> Result<CityHandle> {
        self.city().context(format!("Expected position to be city, found {}", self))
    }
    pub fn edge(&self) -> Option<(CityHandle, CityHandle)> {
        match self {
            GraphPosition::Node(_) => None,
            GraphPosition::Edge(to, from) => Some((*to, *from)),
        }
    }
    pub fn edge_res(&self) -> Result<(CityHandle, CityHandle)> {
        self.edge().context(format!("Expected position to be edge, found {}", self))
    }
}

impl std::fmt::Display for Agent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl std::fmt::Display for AgentHandle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.agent)
    }
}

impl std::fmt::Display for GraphPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GraphPosition::Node(city) => write!(f, "{}", city.city),
            GraphPosition::Edge(to, from) => write!(f, "{} <--> {}", to.city, from.city),
        }
    }
}