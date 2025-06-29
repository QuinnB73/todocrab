use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::{
    fs::{self, File},
    io::{self, stderr, Stderr},
    path::PathBuf,
};

mod app;
mod task;
mod ui;

use app::{App, Mode};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut terminal = init_terminal()?;
    let config_dir = dirs::config_dir().unwrap().join("todocrab");
    fs::create_dir_all(&config_dir)?;
    let state_path = config_dir.join("state.json");

    let mut app = load_state(state_path.clone()).unwrap_or_else(|_| App::new(state_path));
    app.tasks.post_deserialize();

    run_app(&mut terminal, &mut app).await?;

    restore_terminal()?;
    save_state(&app)?;

    Ok(())
}

async fn run_app(terminal: &mut Terminal<CrosstermBackend<Stderr>>, app: &mut App) -> io::Result<()> {
    while !app.should_quit {
        app.tick();
        ui::render(app, terminal)?;
        if event::poll(std::time::Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match app.mode {
                        Mode::Normal => match key.code {
                            KeyCode::Char('q') => app.quit(),
                            KeyCode::Char('j') | KeyCode::Down => app.tasks.next(),
                            KeyCode::Char('k') | KeyCode::Up => app.tasks.previous(),
                            KeyCode::Char('l') | KeyCode::Right => {
                                if let Some(selected) = app.tasks.state.selected() {
                                    app.tasks.items[selected].state = app.tasks.items[selected].state.next();
                                }
                            }
                            KeyCode::Char('h') | KeyCode::Left => {
                                if let Some(selected) = app.tasks.state.selected() {
                                    app.tasks.items[selected].state = app.tasks.items[selected].state.prev();
                                }
                            }
                            KeyCode::Char('J') => {
                                if let Some(selected) = app.tasks.state.selected() {
                                    if selected < app.tasks.items.len() - 1 {
                                        app.tasks.items.swap(selected, selected + 1);
                                        app.tasks.next();
                                    }
                                }
                            }
                            KeyCode::Char('K') => {
                                if let Some(selected) = app.tasks.state.selected() {
                                    if selected > 0 {
                                        app.tasks.items.swap(selected, selected - 1);
                                        app.tasks.previous();
                                    }
                                }
                            }
                            KeyCode::Char('a') => app.enter_editing_mode(),
                            KeyCode::Char('d') => app.enter_confirm_delete_mode(),
                            KeyCode::Char('e') => app.enter_editing_task_mode(),
                            _ => {}
                        },
                        Mode::Editing => match key.code {
                            KeyCode::Enter => app.submit_input(),
                            KeyCode::Esc => app.exit_editing_mode(),
                            KeyCode::Char(c) => {
                                app.input.insert(app.cursor_position, c);
                                app.cursor_position += 1;
                            }
                            KeyCode::Backspace => {
                                if app.cursor_position > 0 {
                                    app.cursor_position -= 1;
                                    app.input.remove(app.cursor_position);
                                }
                            }
                            _ => {}
                        },
                        Mode::ConfirmDelete => match key.code {
                            KeyCode::Char('y') | KeyCode::Char('Y') => app.delete_task(),
                            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => app.cancel_delete(),
                            _ => {}
                        },
                        Mode::EditingTask => match key.code {
                            KeyCode::Enter => app.submit_edited_task(),
                            KeyCode::Esc => app.cancel_editing_task(),
                            KeyCode::Char(c) => {
                                app.input.insert(app.cursor_position, c);
                                app.cursor_position += 1;
                            }
                            KeyCode::Backspace => {
                                if app.cursor_position > 0 {
                                    app.cursor_position -= 1;
                                    app.input.remove(app.cursor_position);
                                }
                            }
                            _ => {}
                        },
                    }
                }
            }
        }
    }
    Ok(())
}

fn init_terminal() -> io::Result<Terminal<CrosstermBackend<Stderr>>> {
    enable_raw_mode()?;
    stderr().execute(EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(stderr()))
}

fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    stderr().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn save_state(app: &App) -> io::Result<()> {
    let file = File::create(&app.path)?;
    serde_json::to_writer(file, app)?;
    Ok(())
}

fn load_state(path: PathBuf) -> io::Result<App> {
    let file = File::open(path)?;
    let app = serde_json::from_reader(file)?;
    Ok(app)
}