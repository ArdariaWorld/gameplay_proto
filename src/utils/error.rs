use bevy::ecs::{query::QuerySingleError, schedule::StateError};

// https://github.com/dtolnay - living rust god
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorMessage {
    #[error("Query failed")]
    BevyQueryError(#[from] QuerySingleError),

    #[error("Cannot set game state")]
    BevyStateError(#[from] StateError),

    #[error("No window")]
    NoWindow,

    #[error("No cursor positon")]
    NoCursorPosition,
}
