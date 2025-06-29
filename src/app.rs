use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::task::Task;

#[derive(Default, Serialize, Deserialize)]
pub enum Mode {
    #[default]
    Normal,
    Editing,
    ConfirmDelete,
    EditingTask,
}

#[derive(Serialize, Deserialize)]
pub struct App {
    #[serde(skip)]
    pub should_quit: bool,
    pub tasks: StatefulList<Task>,
    pub path: PathBuf,
    #[serde(skip)]
    pub mode: Mode,
    #[serde(skip)]
    pub input: String,
    #[serde(skip)]
    pub cursor_position: usize,
    #[serde(skip)]
    pub task_to_delete: Option<usize>,
    #[serde(skip)]
    pub editing_task_original_title: String,
    #[serde(skip)]
    pub editing_task_index: Option<usize>,
}

impl App {
    pub fn new(path: PathBuf) -> Self {
        Self {
            should_quit: false,
            tasks: StatefulList::new(vec![]),
            path,
            mode: Mode::Normal,
            input: String::new(),
            cursor_position: 0,
            task_to_delete: None,
            editing_task_original_title: String::new(),
            editing_task_index: None,
        }
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn enter_editing_mode(&mut self) {
        self.mode = Mode::Editing;
    }

    pub fn exit_editing_mode(&mut self) {
        self.mode = Mode::Normal;
        self.input.clear();
        self.cursor_position = 0;
    }

    pub fn submit_input(&mut self) {
        if !self.input.is_empty() {
            self.tasks.items.push(Task::new(&self.input));
        }
        self.clear_input();
    }

    pub fn clear_input(&mut self) {
        self.input.clear();
        self.cursor_position = 0;
    }

    pub fn enter_confirm_delete_mode(&mut self) {
        if let Some(selected) = self.tasks.state.selected() {
            self.task_to_delete = Some(selected);
            self.mode = Mode::ConfirmDelete;
        }
    }

    pub fn delete_task(&mut self) {
        if let Some(index) = self.task_to_delete {
            self.tasks.items.remove(index);
            if self.tasks.items.is_empty() {
                self.tasks.unselect();
            } else if index >= self.tasks.items.len() {
                self.tasks.state.select(Some(self.tasks.items.len() - 1));
            }
        }
        self.task_to_delete = None;
        self.mode = Mode::Normal;
    }

    pub fn cancel_delete(&mut self) {
        self.task_to_delete = None;
        self.mode = Mode::Normal;
    }

    pub fn enter_editing_task_mode(&mut self) {
        if let Some(selected) = self.tasks.state.selected() {
            self.mode = Mode::EditingTask;
            self.editing_task_index = Some(selected);
            self.editing_task_original_title = self.tasks.items[selected].title.clone();
            self.input = self.tasks.items[selected].title.clone();
            self.cursor_position = self.input.len();
        }
    }

    pub fn submit_edited_task(&mut self) {
        if let Some(index) = self.editing_task_index {
            self.tasks.items[index].title = self.input.clone();
        }
        self.exit_editing_mode(); // Reuses the exit_editing_mode to clear input and reset cursor
        self.editing_task_index = None;
        self.editing_task_original_title.clear();
    }

    pub fn cancel_editing_task(&mut self) {
        if let Some(index) = self.editing_task_index {
            self.tasks.items[index].title = self.editing_task_original_title.clone();
        }
        self.exit_editing_mode(); // Reuses the exit_editing_mode to clear input and reset cursor
        self.editing_task_index = None;
        self.editing_task_original_title.clear();
    }
}

#[derive(Serialize, Deserialize)]
pub struct StatefulList<T> {
    #[serde(skip)]
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn new(items: Vec<T>) -> Self {
        let mut state = ListState::default();
        if !items.is_empty() {
            state.select(Some(0));
        }

        Self {
            state,
            items,
        }
    }

    pub fn post_deserialize(&mut self) {
        if !self.items.is_empty() {
            self.state.select(Some(0));
        } else {
            self.state.select(None);
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
