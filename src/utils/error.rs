use bevy::ecs::{
    query::{QueryEntityError, QuerySingleError},
    schedule::StateError,
};

// https://github.com/dtolnay - living rust god
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorMessage {
    #[error("Query failed")]
    BevyQueryError(#[from] QuerySingleError),

    #[error("Entity query failed")]
    BevyQueryEntityError(#[from] QueryEntityError),

    #[error("Cannot set game state")]
    BevyStateError(#[from] StateError),

    #[error("No window")]
    NoWindow,

    #[error("No cursor positon")]
    NoCursorPosition,

    #[error("No entity destination")]
    NoDestination,
}
