use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
};

use crate::i18n::Language;
use crate::app::{App, AppState, InputMode, Sender};

pub struct Theme {
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub background: Color,
    pub text: Color,
    pub border: Color,
}

impl Theme {
    pub fn deep_blue() -> Self {
        Theme {
            primary: Color::Rgb(0, 119, 190),
            secondary: Color::Rgb(0, 180, 216),
            accent: Color::Rgb(144, 224, 239),
            background: Color::Rgb(12, 20, 31),
            text: Color::Rgb(230, 240, 255),
            border: Color::Rgb(30, 58, 95),
        }
    }

    pub fn forest_green() -> Self {
        Theme {
            primary: Color::Rgb(46, 125, 50),
            secondary: Color::Rgb(76, 175, 80),
            accent: Color::Rgb(165, 214, 167),
            background: Color::Rgb(24, 30, 24),
            text: Color::Rgb(240, 255, 240),
            border: Color::Rgb(40, 70, 40),
        }
    }

    pub fn sunset() -> Self {
        Theme {
            primary: Color::Rgb(233, 69, 96),
            secondary: Color::Rgb(255, 119, 34),
            accent: Color::Rgb(255, 190, 11),
            background: Color::Rgb(29, 23, 40),
            text: Color::Rgb(255, 240, 230),
            border: Color::Rgb(80, 40, 60),
        }
    }

    pub fn neon() -> Self {
        Theme {
            primary: Color::Rgb(255, 0, 255),
            secondary: Color::Rgb(0, 255, 255),
            accent: Color::Rgb(255, 255, 0),
            background: Color::Rgb(0, 0, 20),
            text: Color::Rgb(255, 255, 255),
            border: Color::Rgb(60, 0, 60),
        }
    }
}

pub fn render(app: &mut App, frame: &mut Frame) {
    match app.app_state {
        AppState::Welcome => {
            render_welcome_page(app, frame);
        }
        AppState::Chatting => {
            if app.show_help {
                let theme = match app.theme_index { 0 => Theme::deep_blue(), 1 => Theme::forest_green(), 2 => Theme::sunset(), 3 => Theme::neon(), _ => Theme::deep_blue(), };
                render_chat_area(app, frame, frame.area(), &theme);
                render_help_modal(app, frame, frame.area());
            } else {
                render_chat_interface(app, frame);
            }
        }
        AppState::Help => {
            render_help_modal(app, frame, frame.area());
        }
    }
    if let Some(notification) = &app.notification {
        render_notification(app, frame, frame.area(), notification);
    }
}

fn render_welcome_page(app: &App, frame: &mut Frame) {
    let theme = match app.theme_index {
        0 => Theme::deep_blue(),
        1 => Theme::forest_green(),
        2 => Theme::sunset(),
        3 => Theme::neon(),
        _ => Theme::deep_blue(),
    };
    let area = frame.area();
    // Minimal welcome: centered text, no borders
    let welcome_lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            " aichat",
            Style::default()
                .fg(theme.primary)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            " terminal AI chat",
            Style::default().fg(theme.secondary),
        )),
        Line::from(""),
        Line::from(Span::styled(
            match app.language {
                Language::Chinese => "← → 切换模型 │ i 编辑 │ Enter 发送 │ 1-4 主题 │ Q 退出",
                Language::English => "← → switch model │ i edit │ Enter send │ 1-4 theme │ Q quit",
            },
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(Span::styled(
            match app.language {
                Language::Chinese => "按 Enter 开始聊天",
                Language::English => "Press Enter to start",
            },
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD),
        )),
    ];
    let paragraph = Paragraph::new(welcome_lines)
        .style(Style::default().bg(theme.background))
        .alignment(ratatui::layout::Alignment::Center);
    frame.render_widget(paragraph, area);
}

fn render_chat_interface(app: &mut App, frame: &mut Frame) {
    let theme = match app.theme_index {
        0 => Theme::deep_blue(),
        1 => Theme::forest_green(),
        2 => Theme::sunset(),
        3 => Theme::neon(),
        _ => Theme::deep_blue(),
    };

    // Compact layout: status bar (1 line) | chat area (flex) | input (3 lines)
    // NO title bar
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),    // status bar: model + theme + lang (no border)
            Constraint::Min(5),      // chat messages
            Constraint::Length(3),    // input area
        ])
        .split(frame.area());

    render_status_bar(app, frame, main_chunks[0], &theme);
    render_chat_area(app, frame, main_chunks[1], &theme);
    render_input_area(app, frame, main_chunks[2], &theme);
}

/// Single-line status bar: model │ theme │ lang  — no borders, just colored text
fn render_status_bar(app: &App, frame: &mut Frame, area: Rect, theme: &Theme) {
    let current_model = &app.ai_models[app.selected_model_index];
    let model_name = current_model.name(app.language);
    let model_color = current_model.color();

    // Theme indicator: compact dots (current theme highlighted)
    let theme_indices = [0, 1, 2, 3];
    let theme_spans: Vec<Span> = theme_indices.iter().map(|&i| {
        let s = if i == app.theme_index {
            Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        Span::styled("●", s)
    }).collect();

    // Language indicator
    let lang_text = match app.language {
        Language::Chinese => "中",
        Language::English => "EN",
    };

    let status_line = Line::from(vec![
        Span::styled(" ", Style::default()),
        Span::styled(format!("● {}", model_name), Style::default().fg(model_color).add_modifier(Modifier::BOLD)),
        Span::styled("  ", Style::default()),
    ].into_iter().chain(theme_spans.into_iter()).chain([
        Span::styled("  ", Style::default()),
        Span::styled(lang_text, Style::default().fg(theme.accent)),
        Span::styled(" ", Style::default()),
    ]).collect::<Vec<_>>());

    let paragraph = Paragraph::new(status_line)
        .style(Style::default().bg(theme.background))
        .alignment(ratatui::layout::Alignment::Left);
    frame.render_widget(paragraph, area);
}

fn render_chat_area(app: &App, frame: &mut Frame, area: Rect, theme: &Theme) {
    // No block border — just render messages directly
    render_messages(app, frame, area, theme);
}

fn render_messages(app: &App, frame: &mut Frame, area: Rect, theme: &Theme) {
    let messages = app.messages.lock().unwrap();
    if messages.is_empty() {
        let empty_text = Paragraph::new("")
            .style(Style::default().bg(theme.background));
        frame.render_widget(empty_text, area);
        return;
    }

    // Compact: use small padding, tighter spacing
    let inner = Rect {
        x: area.x + 1,
        y: area.y,
        width: area.width.saturating_sub(2),
        height: area.height,
    };

    let mut lines = Vec::new();
    for msg in messages.iter() {
        let (prefix, prefix_color) = match &msg.sender {
            Sender::User => ("▸ ", theme.accent),
            Sender::Thinking(model) => {
                let c = model.color();
                ("◌ ", c)
            }
            Sender::AI(model) => {
                let c = model.color();
                ("· ", c)
            }
        };

        // Minimal timestamp + prefix on same line
        lines.push(Line::from(vec![
            Span::styled(
                format!("{} ", msg.timestamp.format("%H:%M")),
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled(prefix, Style::default().fg(prefix_color)),
            match &msg.sender {
                Sender::User => Span::styled(
                    msg.content.chars().take(80).collect::<String>(),
                    Style::default().fg(theme.text),
                ),
                Sender::Thinking(_) => Span::styled(
                    match app.language {
                        Language::Chinese => "思考中...",
                        Language::English => "thinking...",
                    },
                    Style::default().fg(Color::DarkGray),
                ),
                Sender::AI(_) => Span::styled("", Style::default()),
            },
        ]));

        // AI response body on subsequent lines (indented)
        if matches!(msg.sender, Sender::AI(_)) || matches!(msg.sender, Sender::Thinking(_)) {
            if matches!(msg.sender, Sender::AI(_)) {
                for line in msg.content.lines() {
                    lines.push(Line::from(Span::styled(
                        format!("  {}", line),
                        Style::default().fg(theme.text),
                    )));
                }
            }
        }

        // User message full content (if truncated above)
        if matches!(msg.sender, Sender::User) && msg.content.len() > 80 {
            for line in msg.content.chars().skip(80).collect::<String>().lines() {
                lines.push(Line::from(Span::styled(
                    format!("  {}", line),
                    Style::default().fg(theme.accent),
                )));
            }
        }

        lines.push(Line::from(""));
    }

    let scroll_offset = if app.auto_scroll {
        let total_lines = lines.len();
        let viewport_height = inner.height as usize;
        if total_lines > viewport_height {
            (total_lines - viewport_height) as u16
        } else {
            0
        }
    } else {
        let selected_index = app.ai_list_state.selected().unwrap_or(0);
        (selected_index * 3).min(lines.len().saturating_sub(1)) as u16
    };

    let paragraph = Paragraph::new(lines)
        .style(Style::default().bg(theme.background))
        .wrap(Wrap { trim: true })
        .scroll((scroll_offset, 0));
    frame.render_widget(paragraph, inner);
}

fn render_input_area(app: &App, frame: &mut Frame, area: Rect, theme: &Theme) {
    // Minimal input: bottom border line only, no full block border
    let display_text = if app.input.is_empty() {
        match app.input_mode {
            InputMode::Normal => match app.language {
                Language::Chinese => " i 编辑 │ Enter 发送".to_string(),
                Language::English => " i edit │ Enter send".to_string(),
            },
            InputMode::Editing => {
                if app.cursor_blink_state {
                    "█".to_string()
                } else {
                    " ".to_string()
                }
            }
        }
    } else {
        if app.cursor_blink_state && app.input_mode == InputMode::Editing {
            format!("{}█", app.input)
        } else {
            app.input.clone()
        }
    };

    let input_style = match app.input_mode {
        InputMode::Normal => Style::default().fg(Color::DarkGray),
        InputMode::Editing => Style::default()
            .fg(theme.text)
            .add_modifier(Modifier::BOLD),
    };

    // Just a top border line to separate from chat
    let border_line = Line::from(Span::styled(
        "─".repeat(area.width as usize),
        Style::default().fg(theme.border),
    ));

    let input_paragraph = Paragraph::new(display_text)
        .style(input_style)
        .wrap(Wrap { trim: false });

    let border_rect = Rect {
        x: area.x,
        y: area.y,
        width: area.width,
        height: 1,
    };
    let input_rect = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(1),
        height: area.height.saturating_sub(1),
    };

    frame.render_widget(Paragraph::new(border_line), border_rect);
    frame.render_widget(input_paragraph, input_rect);
}

fn render_help_modal(app: &App, frame: &mut Frame, area: Rect) {
    let theme = Theme::deep_blue();
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(theme.primary))
        .title(app.t("help_title"))
        .title_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .style(Style::default().bg(Color::Black));
    let text = vec![
        Line::from(Span::styled(
            match app.language {
                Language::Chinese => "AI 聊天终端 - 用户指南",
                Language::English => "AI Chat Terminal - User Guide",
            },
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            app.t("help_nav_title"),
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(app.t("help_nav_line1")),
        Line::from(app.t("help_nav_line2")),
        Line::from(app.t("help_nav_line3")),
        Line::from(app.t("help_nav_line4")),
        Line::from(app.t("help_nav_line5")),
        Line::from(app.t("help_nav_line6")),
        Line::from(app.t("help_nav_line7")),
        Line::from(app.t("help_nav_line8")),
        Line::from(app.t("help_nav_line9")),
        Line::from(app.t("help_nav_line10")),
        Line::from(""),
        Line::from(Span::styled(
            app.t("help_edit_title"),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(app.t("help_edit_line1")),
        Line::from(app.t("help_edit_line2")),
        Line::from(app.t("help_edit_line3")),
        Line::from(app.t("help_edit_line4")),
        Line::from(""),
        Line::from(Span::styled(
            app.t("help_theme_title"),
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(app.t("help_theme_line1")),
        Line::from(app.t("help_theme_line2")),
        Line::from(app.t("help_theme_line3")),
        Line::from(app.t("help_theme_line4")),
        Line::from(""),
        Line::from(Span::styled(
            app.t("help_tips_title"),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(app.t("help_tips_line1")),
        Line::from(app.t("help_tips_line2")),
        Line::from(app.t("help_tips_line3")),
        Line::from(app.t("help_tips_line4")),
        Line::from(""),
        Line::from(Span::styled(
            app.t("help_close_hint"),
            Style::default()
                .fg(Color::Gray)
                .add_modifier(Modifier::ITALIC),
        )),
    ];
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    let area = centered_rect(70, 80, area);
    frame.render_widget(Clear, area);
    frame.render_widget(paragraph, area);
}

fn render_notification(app: &App, frame: &mut Frame, area: Rect, notification: &str) {
    let theme = match app.theme_index {
        0 => Theme::deep_blue(),
        1 => Theme::forest_green(),
        2 => Theme::sunset(),
        3 => Theme::neon(),
        _ => Theme::deep_blue(),
    };
    let notification_text = vec![
        Line::from(Span::styled(
            notification,
            Style::default().fg(theme.text),
        )),
    ];
    // Compact notification: just a thin bar at the bottom
    let block = Block::default()
        .border_type(BorderType::Plain)
        .border_style(Style::default().fg(theme.primary))
        .style(Style::default().bg(theme.background));
    let paragraph = Paragraph::new(notification_text)
        .block(block)
        .alignment(ratatui::layout::Alignment::Center);
    // Small area at bottom center
    let w = notification.len().max(20).min(area.width as usize) as u16;
    let notification_area = Rect {
        x: area.x + area.width.saturating_sub(w) / 2,
        y: area.y + area.height.saturating_sub(3),
        width: w,
        height: 2,
    };
    frame.render_widget(Clear, notification_area);
    frame.render_widget(paragraph, notification_area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
