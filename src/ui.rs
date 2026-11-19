use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
};

use crate::app::{App, CurrentScreen, CurrentlyEditing};

const ACCENT: Color = Color::LightMagenta;
const KEY_COLOR: Color = Color::Yellow;
const LABEL_COLOR: Color = Color::Green;
const MUTED_COLOR: Color = Color::DarkGray;

pub fn ui(frame: &mut Frame, app: &App) {
    let [header_area, list_area, footer_area] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(1),
        Constraint::Length(3),
    ])
    .areas(frame.area());

    render_header(frame, header_area);
    render_pairs(frame, list_area, app);
    render_footer(frame, footer_area, app);

    match app.screen() {
        CurrentScreen::Main => {}
        CurrentScreen::Editing => render_editor(frame, app),
        CurrentScreen::Exiting => render_exit_prompt(frame),
    }
}

fn render_header(frame: &mut Frame, area: Rect) {
    let title =
        Paragraph::new("Create New JSON".fg(LABEL_COLOR)).block(Block::new().borders(Borders::ALL));
    frame.render_widget(title, area);
}

fn render_pairs(frame: &mut Frame, area: Rect, app: &App) {
    let items: Vec<ListItem> = if app.pairs().is_empty() {
        vec![ListItem::new(
            "No pairs yet. Press (e) to add one.".fg(MUTED_COLOR),
        )]
    } else {
        app.pairs()
            .iter()
            .map(|(key, value)| {
                ListItem::new(Line::from(vec![
                    Span::styled(format!("{key:<25}"), Style::default().fg(KEY_COLOR)),
                    Span::raw(": "),
                    Span::raw(value.clone()),
                ]))
            })
            .collect()
    };

    let list = List::new(items).block(Block::new().borders(Borders::ALL));
    frame.render_widget(list, area);
}

fn render_footer(frame: &mut Frame, area: Rect, app: &App) {
    let [mode_area, hints_area] =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(area);

    let mode = match app.screen() {
        CurrentScreen::Main => "Normal Mode".fg(LABEL_COLOR).bold(),
        CurrentScreen::Editing => "Editing Mode".fg(Color::Yellow).bold(),
        CurrentScreen::Exiting => "Exiting".fg(Color::LightRed).bold(),
    };

    let focus = match app.editing() {
        Some(CurrentlyEditing::Key) => "Editing JSON Key".fg(LABEL_COLOR),
        Some(CurrentlyEditing::Value) => "Editing JSON Value".fg(Color::LightGreen),
        None => "Not Editing Anything".fg(MUTED_COLOR),
    };

    let hints = match app.screen() {
        CurrentScreen::Main => "(q) Quit / (e) New Pair",
        CurrentScreen::Editing => "(Esc) Cancel / (Tab) Switch / (Enter) Save",
        CurrentScreen::Exiting => "(y) Confirm / (n) Cancel",
    };

    let mode_line = Paragraph::new(Line::from(vec![mode, Span::raw("  |  "), focus]))
        .block(Block::new().borders(Borders::ALL));

    let hints_line = Paragraph::new(hints.fg(Color::Red)).block(Block::new().borders(Borders::ALL));

    frame.render_widget(mode_line, mode_area);
    frame.render_widget(hints_line, hints_area);
}

fn render_editor(frame: &mut Frame, app: &App) {
    let area = centered_rect(60, 25, frame.area());
    frame.render_widget(Clear, area);
    frame.render_widget(
        Block::new()
            .title("Enter a new key-value pair")
            .borders(Borders::ALL),
        area,
    );

    let [key_area, value_area] =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .margin(1)
            .areas(area);

    let active_style = Style::default().bg(ACCENT).fg(Color::Black);
    let editing_value = app.editing() == Some(CurrentlyEditing::Value);

    render_input(
        frame,
        key_area,
        "Key",
        app.key_input(),
        !editing_value,
        active_style,
    );
    render_input(
        frame,
        value_area,
        "Value",
        app.value_input(),
        editing_value,
        active_style,
    );

    // Keep the caret visible in the field being edited.
    let (active_area, active_text) = if editing_value {
        (value_area, app.value_input())
    } else {
        (key_area, app.key_input())
    };
    let x =
        (active_area.x + 1 + active_text.len() as u16).min(active_area.right().saturating_sub(2));
    frame.set_cursor_position((x, active_area.y + 1));
}

fn render_input(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    text: &str,
    active: bool,
    active_style: Style,
) {
    let block = Block::new()
        .title(title)
        .borders(Borders::ALL)
        .style(if active {
            active_style
        } else {
            Style::default()
        });

    frame.render_widget(Paragraph::new(text).block(block), area);
}

fn render_exit_prompt(frame: &mut Frame) {
    let area = centered_rect(60, 25, frame.area());
    frame.render_widget(Clear, area);

    let prompt = Paragraph::new("Would you like to output the buffer as JSON? (y/n)")
        .fg(Color::Red)
        .wrap(Wrap { trim: false })
        .block(Block::new().title("Y/N").borders(Borders::ALL));

    frame.render_widget(prompt, area);
}

/// Returns a rectangle centered inside `area`, sized as a percentage of it.
fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let [_, middle, _] = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .areas(area);

    let [_, centered, _] = Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .areas(middle);

    centered
}
