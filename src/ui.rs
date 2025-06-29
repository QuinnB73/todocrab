use ratatui::{
    prelude::{Constraint, CrosstermBackend, Direction, Layout, Rect, Terminal},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::app::{App, Mode};

pub fn render(app: &mut App, terminal: &mut Terminal<CrosstermBackend<std::io::Stderr>>) -> std::io::Result<()> {
    terminal.draw(|frame| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                match app.mode {
                    Mode::Editing | Mode::EditingTask => {
                        [Constraint::Percentage(85), Constraint::Percentage(10), Constraint::Percentage(5)].as_ref()
                    }
                    _ => {
                        [Constraint::Percentage(95), Constraint::Percentage(5)].as_ref()
                    }
                },
            )
            .split(frame.area());

        let main_block = Block::default().title("TODO List").borders(Borders::ALL);
        frame.render_widget(main_block, chunks[0]);

        let list_area = Rect::new(
            chunks[0].x + 1,
            chunks[0].y + 1,
            chunks[0].width - 2,
            chunks[0].height - 2,
        );
        let items: Vec<ListItem> = app
            .tasks
            .items
            .iter()
            .map(|task| {
                let state = task.state;
                let title = &task.title;
                ListItem::new(format!("{} {}", state, title))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::NONE))
            .highlight_style(
                Style::default()
                    .fg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            );

        frame.render_stateful_widget(list, list_area, &mut app.tasks.state);

        match app.mode {
            Mode::Normal | Mode::ConfirmDelete => {},
            Mode::Editing | Mode::EditingTask => {
                let input_area = chunks[1];
                let title = match app.mode {
                    Mode::Editing => "New Task",
                    Mode::EditingTask => "Edit Task",
                    _ => unreachable!(),
                };
                let input = Paragraph::new(app.input.as_str())
                    .block(Block::default().borders(Borders::ALL).title(title));
                frame.render_widget(input, input_area);
                frame.set_cursor_position((input_area.x + app.cursor_position as u16 + 1, input_area.y + 1));
            },
        }

        if let Mode::ConfirmDelete = app.mode {
            let confirm_area = chunks[1];
            let confirm_text = Paragraph::new("Are you sure you want to delete this task? (y/n)")
                .block(Block::default().borders(Borders::ALL).title("Confirm Delete"));
            frame.render_widget(confirm_text, confirm_area);
        }

        let legend_text = match app.mode {
            Mode::Normal => "j/k: move, h/l: change state, J/K: move task, a: add, d: delete, e: edit, q: quit",
            Mode::Editing => "Enter: save, Esc: cancel",
            Mode::ConfirmDelete => "y: confirm, n/Esc: cancel",
            Mode::EditingTask => "Enter: save, Esc: cancel",
        };
        let legend = Paragraph::new(legend_text)
            .block(Block::default().borders(Borders::TOP).title("Keys"));
        
        let legend_chunk_index = if matches!(app.mode, Mode::Editing | Mode::EditingTask) { 2 } else { 1 };
        frame.render_widget(legend, chunks[legend_chunk_index]);
    })?;
    Ok(())
}
