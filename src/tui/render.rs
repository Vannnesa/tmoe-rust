use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

use super::app::App;
use super::menu::Menu;

pub fn draw(f: &mut Frame, app: &App) {
    // Clear the entire screen first
    f.render_widget(Clear, f.area());
    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(f.area());

    // Title
    draw_title(f, app, chunks[0]);

    // Menu
    draw_menu(f, app, chunks[1]);

    // Footer
    draw_footer(f, app, chunks[2]);
}

fn draw_title(f: &mut Frame, app: &App, area: Rect) {
    let title_style = Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::BOLD);

    let title = match app.language {
        super::app::Language::English => "TMOE Linux Tools",
        super::app::Language::Chinese => "TMOE Linux 工具",
    };

    let paragraph = Paragraph::new(title)
        .style(title_style)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::BOTTOM));

    f.render_widget(paragraph, area);
}

fn draw_menu(f: &mut Frame, app: &App, area: Rect) {
    let menu = Menu::main_menu(app.language);

    let items: Vec<ListItem> = menu
        .items
        .iter()
        .enumerate()
        .map(|(idx, item)| {
            let is_selected = idx == app.selected_index;

            let style = if is_selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let label = if is_selected {
                format!("❯ {}", item.label)
            } else {
                format!("  {}", item.label)
            };

            let content = vec![Line::from(Span::styled(label, style))];

            ListItem::new(content)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(match app.language {
                    super::app::Language::English => "Select an option",
                    super::app::Language::Chinese => "选择选项",
                })
                .border_style(Style::default().fg(Color::Green)),
        )
        .style(Style::default().fg(Color::White));

    f.render_widget(list, area);
}

fn draw_footer(f: &mut Frame, app: &App, area: Rect) {
    let help_text = match app.language {
        super::app::Language::English => "↑↓/jk: Navigate │ Enter: Select │ ESC/q: Back │ C-c: Quit",
        super::app::Language::Chinese => "↑↓/jk: 导航 │ Enter: 选择 │ ESC/q: 返回 │ C-c: 退出",
    };

    let footer = Paragraph::new(help_text)
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::TOP));

    f.render_widget(footer, area);

    // Status message
    if !app.message.is_empty() {
        let msg_style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);
        let message = Paragraph::new(app.message.as_str())
            .style(msg_style)
            .alignment(Alignment::Left);

        let msg_area = Rect {
            x: area.x,
            y: area.y,
            width: area.width,
            height: 1,
        };

        f.render_widget(message, msg_area);
    }
}
