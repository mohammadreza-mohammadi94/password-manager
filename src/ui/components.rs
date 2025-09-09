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
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    let title = Paragraph::new("🔒 Privacy-First Password Manager")
        .style(Style::default().fg(Color::Blue))
        .alignment(tui::layout::Alignment::Center);
    f.render_widget(title, chunks[0]);

    let password = Paragraph::new(format!("Master Password: {}", "*".repeat(app.master_password.len())))
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(password, chunks[1]);

    let instructions = Paragraph::new("Enter master password and press Enter to unlock\nPress Ctrl+R to reset vault")
        .style(Style::default().fg(Color::Gray));
    f.render_widget(instructions, chunks[2]);

    if let Some(error) = &app.error_message {
        let error_msg = Paragraph::new(error.as_str())
            .style(Style::default().fg(Color::Red));
        f.render_widget(error_msg, chunks[3]);
    }
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

    let active_style = Style::default().fg(Color::Yellow);
    let inactive_style = Style::default().fg(Color::White);

    let service_block = Block::default().borders(Borders::ALL).title("Service");
    let service = Paragraph::new(app.service_input.as_ref()).block(
        if app.active_field == Some(crate::ui::app::ActiveField::Service) {
            service_block.border_style(active_style)
        } else {
            service_block.border_style(inactive_style)
        },
    );
    f.render_widget(service, chunks[1]);

    let username_block = Block::default().borders(Borders::ALL).title("Username");
    let username = Paragraph::new(app.username_input.as_ref()).block(
        if app.active_field == Some(crate::ui::app::ActiveField::Username) {
            username_block.border_style(active_style)
        } else {
            username_block.border_style(inactive_style)
        },
    );
    f.render_widget(username, chunks[2]);

    let password_block = Block::default().borders(Borders::ALL).title("Password");
    let password = Paragraph::new(app.password_input.as_ref()).block(
        if app.active_field == Some(crate::ui::app::ActiveField::Password) {
            password_block.border_style(active_style)
        } else {
            password_block.border_style(inactive_style)
        },
    );
    f.render_widget(password, chunks[3]);

    let notes_block = Block::default().borders(Borders::ALL).title("Notes");
    let notes = Paragraph::new(app.notes_input.as_ref()).block(
        if app.active_field == Some(crate::ui::app::ActiveField::Notes) {
            notes_block.border_style(active_style)
        } else {
            notes_block.border_style(inactive_style)
        },
    );
    f.render_widget(notes, chunks[4]);

    let help_text = match app.input_mode {
        crate::ui::app::InputMode::Normal => "Tab: Next, i: Insert, Enter: Save, Esc: Cancel",
        crate::ui::app::InputMode::Editing => "Esc: Stop editing",
    };
    let help = Paragraph::new(help_text).style(Style::default().fg(Color::Gray));
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

            let password_display = if app.show_password {
                String::from_utf8_lossy(&cred.password).to_string()
            } else {
                "********".to_string()
            };
            let password = Paragraph::new(format!("Password: {}", password_display))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(password, chunks[3]);

            let notes = Paragraph::new(format!("Notes: {}", cred.notes))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(notes, chunks[4]);

            let help = Paragraph::new("s: show/hide password, q/Esc: return")
                .style(Style::default().fg(Color::Gray));
            f.render_widget(help, chunks[5]);
        }
    }
}