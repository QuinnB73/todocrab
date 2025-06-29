use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskState {
    Todo,
    InProgress,
    Done,
}

impl TaskState {
    pub fn next(&self) -> Self {
        match self {
            Self::Todo => Self::InProgress,
            Self::InProgress => Self::Done,
            Self::Done => Self::Todo,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            Self::Todo => Self::Done,
            Self::InProgress => Self::Todo,
            Self::Done => Self::InProgress,
        }
    }
}

impl fmt::Display for TaskState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Todo => write!(f, "[ ]"),
            Self::InProgress => write!(f, "[>]"),
            Self::Done => write!(f, "[x]"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub title: String,
    pub state: TaskState,
}

impl Task {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            state: TaskState::Todo,
        }
    }
}