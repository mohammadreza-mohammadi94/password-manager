use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use crate::ui::app::App;

pub fn draw_lock_screen<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());

    let title = Paragraph::new("🔒 Privacy-First Password Manager")
        .style(Style::default().fg(Color::Blue))
        .alignment(tui::layout::Alignment::Center);
    f.render_widget(title, chunks[0]);

    let password = Paragraph::new(format!("Master Password: {}", app.master_password))
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(password, chunks[1]);

    let instructions = Paragraph::new("Enter master password and press Enter to unlock")
        .style(Style::default().fg(Color::Gray));
    f.render_widget(instructions, chunks[2]);
}

pub fn draw_main_screen<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());

    let title = Paragraph::new("🔐 Password Manager")
        .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let items: Vec<ListItem> = app
        .credentials
        .iter()
        .enumerate()
        .map(|(i, cred)| {
            let style = if Some(i) == app.selected_credential {
                Style::default().bg(Color::Blue).fg(Color::White)
            } else {
                Style::default()
            };
            ListItem::new(vec![
                Spans::from(Span::styled(
                    format!("{} - {}", cred.service, cred.username),
                    style,
                )),
            ])
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Credentials"))
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));
    f.render_widget(list, chunks[1]);

    let help_text = if app.credentials.is_empty() {
        "Press 'a' to add a credential"
    } else {
        "↑/↓: Navigate, Enter: View, 'a': Add, 'q': Quit"
    };

    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(help, chunks[2]);
}

pub fn draw_add_credential_screen<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());

    let title = Paragraph::new("➕ Add New Credential")
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let service = Paragraph::new(format!("Service: {}", app.service_input))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(service, chunks[1]);

    let username = Paragraph::new(format!("Username: {}", app.username_input))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(username, chunks[2]);

    let password = Paragraph::new(format!("Password: {}", app.password_input))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(password, chunks[3]);

    let notes = Paragraph::new(format!("Notes: {}", app.notes_input))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(notes, chunks[4]);

    let help = Paragraph::new("Tab: Next field, Enter: Save, Esc: Cancel")
        .style(Style::default().fg(Color::Gray));
    f.render_widget(help, chunks[5]);
}

pub fn draw_view_credential_screen<B: Backend>(f: &mut Frame<B>, app: &App) {
    if let Some(index) = app.selected_credential {
        if let Some(cred) = app.credentials.get(index) {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let title = Paragraph::new("👁️ View Credential")
                .style(Style::default().fg(Color::Magenta))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(title, chunks[0]);

            let service = Paragraph::new(format!("Service: {}", cred.service))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(service, chunks[1]);

            let username = Paragraph::new(format!("Username: {}", cred.username))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(username, chunks[2]);

            let password = Paragraph::new("Password: ********")
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(password, chunks[3]);

            let notes = Paragraph::new(format!("Notes: {}", cred.notes))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(notes, chunks[4]);

            let help = Paragraph::new("Press any key to return")
                .style(Style::default().fg(Color::Gray));
            f.render_widget(help, chunks[5]);
        }
    }
}