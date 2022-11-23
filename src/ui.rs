use crate::app::{App, Mode};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Cell, List, ListItem, Row, Table, Tabs},
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    let titles = app
        .tabs
        .titles
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Green))))
        .collect();
    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(match app.mode.mode {
                    Mode::FocusOnTab => Color::White,
                    _ => Color::DarkGray,
                }))
                .title(app.title),
        )
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.index);
    f.render_widget(tabs, chunks[0]);
    match app.tabs.index {
        0 => draw_first_tab(f, app, chunks[1]),
        1 => draw_second_tab(f, app, chunks[1]),
        _ => {}
    };
}

fn draw_first_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let (left, right) = {
        let chunks = Layout::default()
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .direction(Direction::Horizontal)
            .split(area);
        (chunks[0], chunks[1])
    };

    let (left_top, left_bottom) = {
        let chunks = Layout::default()
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(left);
        (chunks[0], chunks[1])
    };

    let (left_top_left, left_top_right) = {
        let chunks = Layout::default()
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .direction(Direction::Horizontal)
            .split(left_top);
        (chunks[0], chunks[1])
    };

    // Draw tasks
    let tasks: Vec<ListItem> = app
        .tables
        .items
        .iter()
        .map(|i| ListItem::new(vec![Spans::from(Span::raw(*i))]))
        .collect();
    let tasks = List::new(tasks)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(match app.mode.mode {
                    Mode::FocusOnTable => Color::White,
                    _ => Color::DarkGray,
                }))
                .title("List"),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");
    f.render_stateful_widget(tasks, left_top_left, &mut app.tables.state);

    // Draw logs
    let logs = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
        .title("Indexes");
    f.render_widget(logs, left_top_right);

    let operations = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(match app.mode.mode {
            Mode::FocusOnOperation => Color::White,
            _ => Color::DarkGray,
        }))
        .title("Operations");
    f.render_widget(operations, left_bottom);

    let item_table = Table::new(vec![
        Row::new(vec!["Row11", "Row12", "Row13"]),
        Row::new(vec!["Row21", "Row22", "Row23"]),
        Row::new(vec!["Row31", "Row32", "Row33"]),
        Row::new(vec!["Row41", "Row42", "Row43"]),
    ])
    .style(Style::default().fg(Color::White))
    .header(
        Row::new(vec!["Col1", "Col2", "Col3"])
            .style(Style::default().fg(Color::Yellow))
            .bottom_margin(1),
    )
    .block(
        Block::default()
            .title("Items")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(match app.mode.mode {
                Mode::FocusOnItem => Color::White,
                _ => Color::DarkGray,
            })),
    )
    .widths(&[
        Constraint::Length(5),
        Constraint::Length(5),
        Constraint::Length(5),
    ])
    .column_spacing(1)
    // If you wish to highlight a row in any specific way when it is selected...
    .highlight_style(Style::default().add_modifier(Modifier::BOLD))
    // ...and potentially show a symbol in front of the selection.
    .highlight_symbol(">>");

    f.render_widget(item_table, right);
}

fn draw_second_tab<B>(f: &mut Frame<B>, _app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
        .split(area);
    let colors = [
        Color::Reset,
        Color::Black,
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::Gray,
        Color::DarkGray,
        Color::LightRed,
        Color::LightGreen,
        Color::LightYellow,
        Color::LightBlue,
        Color::LightMagenta,
        Color::LightCyan,
        Color::White,
    ];
    let items: Vec<Row> = colors
        .iter()
        .map(|c| {
            let cells = vec![
                Cell::from(Span::raw(format!("{:?}: ", c))),
                Cell::from(Span::styled("Foreground", Style::default().fg(*c))),
                Cell::from(Span::styled("Background", Style::default().bg(*c))),
            ];
            Row::new(cells)
        })
        .collect();
    let table = Table::new(items)
        .block(Block::default().title("Colors").borders(Borders::ALL))
        .widths(&[
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ]);
    f.render_widget(table, chunks[0]);
}
