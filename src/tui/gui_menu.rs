use crate::commands::GuiInstaller;
use crate::tui::{App, EventHandler, UserEvent};
use crate::system::SystemInfo;
use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

pub fn gui_menu(sys_info: &SystemInfo) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut app = App::new();
    let event_handler = EventHandler::new();
    let mut last_action_msg = String::new();
    let mut message_timeout = 0;

    let options = vec!["Install GUI", "Remove GUI"];

    loop {
        terminal.draw(|f| {
            if message_timeout > 0 {
                app.set_message(last_action_msg.clone());
                message_timeout -= 1;
            } else {
                app.clear_message();
            }

            let area = f.area();
            let chunks = ratatui::layout::Layout::default()
                .direction(ratatui::layout::Direction::Vertical)
                .margin(1)
                .constraints([
                    ratatui::layout::Constraint::Length(3),
                    ratatui::layout::Constraint::Min(5),
                    ratatui::layout::Constraint::Length(3),
                ])
                .split(area);

            let title = match app.language {
                crate::tui::app::Language::English => "GUI Management",
                crate::tui::app::Language::Chinese => "GUI 管理",
            };

            let items: Vec<ratatui::widgets::ListItem> = options
                .iter()
                .enumerate()
                .map(|(idx, opt)| {
                    let is_selected = idx == app.selected_index;
                    let style = if is_selected {
                        ratatui::style::Style::default()
                            .fg(ratatui::style::Color::Black)
                            .bg(ratatui::style::Color::Cyan)
                            .add_modifier(ratatui::style::Modifier::BOLD)
                    } else {
                        ratatui::style::Style::default().fg(ratatui::style::Color::White)
                    };

                    let label = if is_selected {
                        format!("❯ {}", opt)
                    } else {
                        format!("  {}", opt)
                    };

                    let content = vec![ratatui::text::Line::from(ratatui::text::Span::styled(
                        label, style,
                    ))];

                    ratatui::widgets::ListItem::new(content)
                })
                .collect();

            let list = ratatui::widgets::List::new(items)
                .block(
                    ratatui::widgets::Block::default()
                        .borders(ratatui::widgets::Borders::ALL)
                        .title(title)
                        .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Green)),
                )
                .style(ratatui::style::Style::default().fg(ratatui::style::Color::White));

            f.render_widget(ratatui::widgets::Clear, f.area());
            f.render_widget(
                ratatui::widgets::Paragraph::new("GUI Management")
                    .style(ratatui::style::Style::default()
                        .fg(ratatui::style::Color::Cyan)
                        .add_modifier(ratatui::style::Modifier::BOLD))
                    .alignment(ratatui::layout::Alignment::Center)
                    .block(ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::BOTTOM)),
                chunks[0],
            );

            f.render_widget(list, chunks[1]);

            let help_text = match app.language {
                crate::tui::app::Language::English => "↑↓/jk: Navigate │ Enter: Select │ ESC/q: Back │ C-c: Quit",
                crate::tui::app::Language::Chinese => "↑↓/jk: 导航 │ Enter: 选择 │ ESC/q: 返回 │ C-c: 退出",
            };

            f.render_widget(
                ratatui::widgets::Paragraph::new(help_text)
                    .style(ratatui::style::Style::default().fg(ratatui::style::Color::DarkGray))
                    .alignment(ratatui::layout::Alignment::Center)
                    .block(ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::TOP)),
                chunks[2],
            );
        })?;

        if let Some(event) = event_handler.recv() {
            match event {
                UserEvent::MoveUp => app.move_up(),
                UserEvent::MoveDown => {
                    app.move_down(options.len());
                }
                UserEvent::Select => {
                    match app.selected_index {
                        0 => {
                            GuiInstaller::install_gui(&sys_info.distro.to_string()).ok();
                            last_action_msg = "GUI installation started".to_string();
                            message_timeout = 30;
                        }
                        1 => {
                            GuiInstaller::remove_gui(&sys_info.distro.to_string()).ok();
                            last_action_msg = "GUI removal started".to_string();
                            message_timeout = 30;
                        }
                        _ => {}
                    }
                }
                UserEvent::Back => break,
                UserEvent::Quit => break,
                UserEvent::Refresh => {}
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
