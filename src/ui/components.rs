use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};
use crate::ui::app::{App, InputMode};
use crate::models::EntryType;

// Helper function to create a centered block
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

pub fn draw_lock_screen<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();
    let block = Block::default().style(Style::default().bg(Color::Black));
    f.render_widget(block, size);

    let area = centered_rect(60, 40, size);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(area);

    let title = Paragraph::new(vec![
        Spans::from(Span::styled("🔒 Privacy-First Password Manager", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
    ])
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::ALL).border_type(tui::widgets::BorderType::Double));
    f.render_widget(title, area);

    let password_input = Paragraph::new(format!("Enter Master Password:\n{}", "*".repeat(app.master_password.len())))
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(password_input, chunks[1]);

    let instructions = Paragraph::new("Press Enter to unlock, or Ctrl+R to reset.")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    f.render_widget(instructions, chunks[3]);

    if let Some(error) = &app.error_message {
        let error_area = centered_rect(50, 20, size);
        let error_block = Block::default().title("Error").borders(Borders::ALL).border_style(Style::default().fg(Color::Red));
        let error_text = Paragraph::new(error.as_str())
            .style(Style::default().fg(Color::Red))
            .block(error_block)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        f.render_widget(Clear, error_area); //this clears the background
        f.render_widget(error_text, error_area);
    }
}

pub fn draw_main_screen<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());

    let title = Paragraph::new("🔐 Credential Vault")
        .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_type(tui::widgets::BorderType::Rounded));
    f.render_widget(title, chunks[0]);

    let search_bar = Paragraph::new(app.search_query.as_ref())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Search (/)"));
    f.render_widget(search_bar, chunks[1]);

    let items: Vec<ListItem> = app
        .credentials
        .iter()
        .enumerate()
        .map(|(i, cred)| {
            let style = if Some(i) == app.selected_credential {
                Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            };
            let entry_type_icon = match cred.entry_type {
                EntryType::Password => "🔑",
                EntryType::ApiKey => "⚙️",
            };
            let tags = cred.tags.join(", ");
            let content = Spans::from(vec![
                Span::styled(format!("{} ", entry_type_icon), Style::default()),
                Span::styled(format!("{:<20}", cred.service), Style::default().fg(Color::Cyan)),
                Span::raw(" - "),
                Span::styled(format!("{:<20}", cred.username.clone()), Style::default().fg(Color::Yellow)),
                Span::raw(" - "),
                Span::styled(tags, Style::default().fg(Color::Magenta)),
            ]);
            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Credentials"))
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));
    f.render_widget(list, chunks[2]);

    let help_text = if app.credentials.is_empty() {
        "Press 'a' to add your first credential."
    } else {
        "↑/↓: Navigate | Enter: View | a: Add | /: Search | q: Quit"
    };

    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_type(tui::widgets::BorderType::Rounded));
    f.render_widget(help, chunks[3]);
}

fn get_constraints(entry_type: &EntryType) -> Vec<Constraint> {
    let mut constraints = vec![
        Constraint::Length(3), // Title
        Constraint::Length(3), // Type Selector
        Constraint::Length(3), // Service
        Constraint::Length(3), // Username/Account Name
        Constraint::Length(3), // Secret
        Constraint::Length(3), // Password Strength
        Constraint::Min(3),    // Notes (flexible height)
        Constraint::Length(3), // Tags
        Constraint::Length(3), // Custom Fields
    ];
    if *entry_type == EntryType::ApiKey {
        constraints.push(Constraint::Length(3)); // Is Active
    }
    constraints.push(Constraint::Length(3)); // Help
    constraints
}

pub fn draw_add_credential_screen<B: Backend>(f: &mut Frame<B>, app: &App) {
    let constraints = get_constraints(&app.entry_type);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(constraints.as_slice())
        .split(f.size());

    let title_text = if app.selected_id.is_some() {
        match app.entry_type {
            EntryType::Password => "✏️ Edit Password",
            EntryType::ApiKey => "✏️ Edit API Key",
        }
    } else {
        match app.entry_type {
            EntryType::Password => "➕ Add New Password",
            EntryType::ApiKey => "➕ Add New API Key",
        }
    };
    let title = Paragraph::new(title_text)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_type(tui::widgets::BorderType::Rounded));
    f.render_widget(title, chunks[0]);

    let active_style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);
    let _inactive_style = Style::default().fg(Color::White);

    // Entry Type Selector
    let type_text = match app.entry_type {
        EntryType::Password => "Type: Password (press 't' to switch)",
        EntryType::ApiKey => "Type: API Key (press 't' to switch)",
    };
    let type_paragraph = Paragraph::new(type_text)
        .style(if app.selected_id.is_some() { Style::default().fg(Color::DarkGray) } else { Style::default() })
        .block(Block::default().borders(Borders::ALL).title("Entry Type"));
    f.render_widget(type_paragraph, chunks[1]);

    // Service
    let service_block = Block::default().borders(Borders::ALL).title("Service");
    let service = Paragraph::new(app.service_input.as_ref()).block(
        if app.active_field == Some(crate::ui::app::ActiveField::Service) {
            service_block.border_style(active_style)
        } else {
            service_block
        },
    );
    f.render_widget(service, chunks[2]);

    // Username / Account Name
    let username_title = match app.entry_type {
        EntryType::Password => "Username",
        EntryType::ApiKey => "Account Name",
    };
    let username_block = Block::default().borders(Borders::ALL).title(username_title);
    let username = Paragraph::new(app.username_input.as_ref()).block(
        if app.active_field == Some(crate::ui::app::ActiveField::Username) {
            username_block.border_style(active_style)
        } else {
            username_block
        },
    );
    f.render_widget(username, chunks[3]);

    // Secret
    let secret_title = match app.entry_type {
        EntryType::Password => "Password",
        EntryType::ApiKey => "API Key",
    };
    let secret_block = Block::default().borders(Borders::ALL).title(secret_title);
    let secret_text = if app.show_secret {
        app.secret_input.clone()
    } else {
        "*".repeat(app.secret_input.len())
    };
    let secret = Paragraph::new(secret_text).block(
        if app.active_field == Some(crate::ui::app::ActiveField::Secret) {
            secret_block.border_style(active_style)
        } else {
            secret_block
        },
    );
    f.render_widget(secret, chunks[4]);

    // Password Strength
    let strength_block = Block::default().borders(Borders::ALL).title("Password Strength");
    let (strength_text, strength_style) = if let Some(score) = app.password_strength {
        match score {
            0 => ("Very Weak", Style::default().fg(Color::Red)),
            1 => ("Weak", Style::default().fg(Color::Red)),
            2 => ("Moderate", Style::default().fg(Color::Yellow)),
            3 => ("Strong", Style::default().fg(Color::Green)),
            4 => ("Very Strong", Style::default().fg(Color::Green)),
            _ => ("", Style::default()),
        }
    } else {
        ("", Style::default())
    };
    let strength_meter = Paragraph::new(strength_text)
        .style(strength_style)
        .block(strength_block);
    f.render_widget(strength_meter, chunks[5]);

    // Notes
    let notes_block = Block::default().borders(Borders::ALL).title("Notes");
    let notes = Paragraph::new(app.notes_input.as_ref()).wrap(Wrap { trim: true }).block(
        if app.active_field == Some(crate::ui::app::ActiveField::Notes) {
            notes_block.border_style(active_style)
        } else {
            notes_block
        },
    );
    f.render_widget(notes, chunks[6]);

    // Tags
    let tags_block = Block::default().borders(Borders::ALL).title("Tags (comma-separated)");
    let tags = Paragraph::new(app.tags_input.as_ref()).block(
        if app.active_field == Some(crate::ui::app::ActiveField::Tags) {
            tags_block.border_style(active_style)
        } else {
            tags_block
        },
    );
    f.render_widget(tags, chunks[7]);

    // Custom Fields
    let custom_fields_block = Block::default().borders(Borders::ALL).title("Custom Fields (key:value, comma-separated)");
    let custom_fields = Paragraph::new(app.custom_fields_input.as_ref()).block(
        if app.active_field == Some(crate::ui::app::ActiveField::CustomFields) {
            custom_fields_block.border_style(active_style)
        } else {
            custom_fields_block
        },
    );
    f.render_widget(custom_fields, chunks[8]);

    let help_chunk_index = if app.entry_type == EntryType::ApiKey {
        // Is Active for API Key
        let is_active_block = Block::default().borders(Borders::ALL).title("Active Status");
        let is_active_text = if app.is_active_input { "✅ Active" } else { "❌ Inactive" };
        let is_active = Paragraph::new(is_active_text).block(
            if app.active_field == Some(crate::ui::app::ActiveField::IsActive) {
                is_active_block.border_style(active_style)
            } else {
                is_active_block
            },
        );
        f.render_widget(is_active, chunks[9]);
        10
    } else {
        9
    };

    // Help Text
    let mode_text = match app.input_mode {
        InputMode::Normal => "Mode: Normal (Press 'i' to edit)",
        InputMode::Editing => "Mode: Editing (Press 'Esc' to stop)",
    };
    let help_text = format!("{} | Tab: Next Field | Enter: Save | Esc: Cancel", mode_text);
    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_type(tui::widgets::BorderType::Rounded));
    f.render_widget(help, chunks[help_chunk_index]);
}

pub fn draw_view_credential_screen<B: Backend>(f: &mut Frame<B>, app: &App) {
    if let Some(index) = app.selected_credential {
        if let Some(cred) = app.credentials.get(index) {
            let constraints = match cred.entry_type {
                EntryType::Password => vec![
                    Constraint::Length(3), // Title
                    Constraint::Length(3), // Service
                    Constraint::Length(3), // Username
                    Constraint::Length(3), // Password
                    Constraint::Min(3),    // Notes
                    Constraint::Length(3), // Tags
                    Constraint::Length(3), // Custom Fields
                    Constraint::Length(3), // Help
                ],
                EntryType::ApiKey => vec![
                    Constraint::Length(3), // Title
                    Constraint::Length(3), // Service
                    Constraint::Length(3), // Account Name
                    Constraint::Length(3), // API Key
                    Constraint::Min(3),    // Notes
                    Constraint::Length(3), // Tags
                    Constraint::Length(3), // Custom Fields
                    Constraint::Length(3), // Is Active
                    Constraint::Length(3), // Help
                ],
            };

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(constraints.as_slice())
                .split(f.size());

            let title_text = match cred.entry_type {
                EntryType::Password => "👁️ View Password",
                EntryType::ApiKey => "👁️ View API Key",
            };
            let title = Paragraph::new(title_text)
                .style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).border_type(tui::widgets::BorderType::Rounded));
            f.render_widget(title, chunks[0]);

            let service = Paragraph::new(Spans::from(vec![
                Span::styled("Service: ", Style::default().fg(Color::Gray)),
                Span::styled(&cred.service, Style::default().fg(Color::White)),
            ]))
            .block(Block::default().borders(Borders::ALL));
            f.render_widget(service, chunks[1]);

            let username_label = match cred.entry_type {
                EntryType::Password => "Username",
                EntryType::ApiKey => "Account Name",
            };
            let username = Paragraph::new(Spans::from(vec![
                Span::styled(format!("{}: ", username_label), Style::default().fg(Color::Gray)),
                Span::styled(&cred.username, Style::default().fg(Color::White)),
            ]))
            .block(Block::default().borders(Borders::ALL));
            f.render_widget(username, chunks[2]);

            let secret_label = match cred.entry_type {
                EntryType::Password => "Password",
                EntryType::ApiKey => "API Key",
            };
            let secret_display = if app.show_secret {
                String::from_utf8_lossy(&cred.secret).to_string()
            } else {
                "•".repeat(cred.secret.len())
            };
            let secret = Paragraph::new(Spans::from(vec![
                Span::styled(format!("{}: ", secret_label), Style::default().fg(Color::Gray)),
                Span::styled(secret_display, Style::default().fg(Color::White)),
            ]))
            .block(Block::default().borders(Borders::ALL));
            f.render_widget(secret, chunks[3]);

            let notes = Paragraph::new(Spans::from(vec![
                Span::styled("Notes: ", Style::default().fg(Color::Gray)),
                Span::styled(&cred.notes, Style::default().fg(Color::White)),
            ]))
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL));
            f.render_widget(notes, chunks[4]);

            let tags = Paragraph::new(Spans::from(vec![
                Span::styled("Tags: ", Style::default().fg(Color::Gray)),
                Span::styled(cred.tags.join(", "), Style::default().fg(Color::White)),
            ]))
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL));
            f.render_widget(tags, chunks[5]);

            let custom_fields_display = cred.custom_fields.iter().map(|(k, v)| format!("{}: {}", k, v)).collect::<Vec<String>>().join("\n");
            let custom_fields = Paragraph::new(Spans::from(vec![
                Span::styled("Custom Fields: ", Style::default().fg(Color::Gray)),
                Span::styled(custom_fields_display, Style::default().fg(Color::White)),
            ]))
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL));
            f.render_widget(custom_fields, chunks[6]);

            let help_chunk_index = match cred.entry_type {
                EntryType::ApiKey => {
                    let active_text = if cred.is_active { "✅ Active" } else { "❌ Inactive" };
                    let is_active = Paragraph::new(Spans::from(vec![
                        Span::styled("Status: ", Style::default().fg(Color::Gray)),
                        Span::styled(active_text, Style::default().fg(Color::White)),
                    ]))
                    .block(Block::default().borders(Borders::ALL));
                    f.render_widget(is_active, chunks[7]);
                    8
                },
                EntryType::Password => 7,
            };
            
            let help = Paragraph::new("c: Copy | s: Show/Hide Secret | e: Edit | d: Delete | q/Esc: Back")
                .style(Style::default().fg(Color::DarkGray))
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).border_type(tui::widgets::BorderType::Rounded));
            f.render_widget(help, chunks[help_chunk_index]);

            if let Some(info) = &app.info_message {
                let info_area = centered_rect(50, 20, f.size());
                let info_block = Block::default().title("Info").borders(Borders::ALL).border_style(Style::default().fg(Color::Green));
                let info_text = Paragraph::new(info.as_str())
                    .style(Style::default().fg(Color::Green))
                    .block(info_block)
                    .alignment(Alignment::Center)
                    .wrap(Wrap { trim: true });
                f.render_widget(Clear, info_area); //this clears the background
                f.render_widget(info_text, info_area);
            }
        }
    }
}
