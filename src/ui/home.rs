use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Wrap},
};

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &App) {
    let parent_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(frame.area());

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(parent_chunks[0]);

    draw_left_list(frame, app, chunks[0]);
    draw_right_details(frame, app, chunks[1]);
    draw_footer(frame, parent_chunks[1]);
}

fn draw_left_list(frame: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .dependencies
        .iter()
        .map(|dep| ListItem::new(dep.compact_line()))
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(" Cargo Dependencies ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    frame.render_stateful_widget(list, area, &mut app.selected.clone());
}

fn draw_right_details(frame: &mut Frame, app: &App, area: Rect) {
    let mut lines = vec![];

    if let Some(dep) = app.selected_dep() {
        lines.push(Line::from(vec![
            Span::styled("Name:    ", Style::default().fg(Color::Cyan)),
            Span::raw(&dep.name),
        ]));

        lines.push(Line::from(vec![
            Span::styled("Version: ", Style::default().fg(Color::Cyan)),
            Span::raw(&dep.version),
        ]));

        lines.push(Line::from(vec![
            Span::styled("Kind:    ", Style::default().fg(Color::Cyan)),
            Span::styled(&dep.kind, Style::default().fg(dep.kind_color())),
        ]));

        lines.push(Line::from("")); // spacer

        lines.push(Line::from(Span::styled(
            "Features:",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )));

        // ← This is the important change
        lines.extend(dep.features_lines());
    } else {
        lines.push(Line::from(Span::styled(
            "No dependency selected",
            Style::default().fg(Color::DarkGray),
        )));
    }

    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .title(" Details ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area);
}

pub fn draw_footer(frame: &mut Frame, area: Rect) {
    let text = Line::from(vec![
        Span::styled(
            " q/Esc ",
            Style::default().fg(Color::Black).bg(Color::White),
        ),
        Span::raw(" Quit  "),
        //
        Span::styled(" ↑/k ", Style::default().fg(Color::Black).bg(Color::White)),
        Span::raw(" Prev  "),
        //
        Span::styled(" ↓/j ", Style::default().fg(Color::Black).bg(Color::White)),
        Span::raw(" Next  "),
        //
        Span::styled(" Space ", Style::default().fg(Color::Black).bg(Color::Cyan)),
        Span::raw(" Toggle features  "),
        //
        Span::styled(" s ", Style::default().fg(Color::Black).bg(Color::Green)),
        Span::raw(" Search  "),
        //
        Span::styled(" u ", Style::default().fg(Color::Black).bg(Color::Yellow)),
        Span::raw(" Update deps "),
    ]);

    let paragraph = Paragraph::new(text)
        // .block(
        //     Block::default()
        //         .borders(Borders::TOP)
        //         .border_type(BorderType::Rounded),
        // )
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, area);
}
