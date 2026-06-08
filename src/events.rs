use crate::app::{App, AppState, InputMode};
use crossterm::event::{KeyCode, KeyEventKind};

pub fn handle_key_event(key: crossterm::event::KeyEvent, app: &mut App) -> bool {
    if key.kind != KeyEventKind::Press {
        return false;
    }
    let should_quit = match app.app_state {
        AppState::Welcome => handle_welcome_event(key, app),
        AppState::Chatting => handle_chatting_event(key, app),
        AppState::Help => handle_help_event(key, app),
    };
    app.clear_notification();
    should_quit
}

fn handle_welcome_event(key: crossterm::event::KeyEvent, app: &mut App) -> bool {
    match key.code {
        KeyCode::Enter => app.start_chatting(),
        KeyCode::Char('c') | KeyCode::Char('C') => app.switch_to_chinese(),
        KeyCode::Char('e') | KeyCode::Char('E') => app.switch_to_english(),
        KeyCode::Char('1') => app.change_theme(0),
        KeyCode::Char('2') => app.change_theme(1),
        KeyCode::Char('3') => app.change_theme(2),
        KeyCode::Char('4') => app.change_theme(3),
        KeyCode::F(1) => app.show_help = true,
        KeyCode::Char('q') => return true,
        _ => {}
    }
    false
}

fn handle_chatting_event(key: crossterm::event::KeyEvent, app: &mut App) -> bool {
    if app.show_help {
        app.show_help = false;
        return false;
    }

    match app.input_mode {
        InputMode::Normal => handle_normal_mode_event(key, app),
        InputMode::Editing => handle_editing_mode_event(key, app),
    }
}

fn handle_normal_mode_event(key: crossterm::event::KeyEvent, app: &mut App) -> bool {
    match key.code {
        KeyCode::Left => {
            if app.input_mode == InputMode::Normal {
                let available_width = 100;
                let max_visible = app.calculate_max_visible(available_width);
                app.select_previous_model(max_visible);
            }
        }
        KeyCode::Right => {
            if app.input_mode == InputMode::Normal {
                let available_width = 100;
                let max_visible = app.calculate_max_visible(available_width);
                app.select_next_model(max_visible);
            }
        }
        KeyCode::Up => {
            let current = app.ai_list_state.selected().unwrap_or(0);
            let max_scroll = app.get_max_scroll_offset();
            if current < max_scroll {
                app.ai_list_state.select(Some(current + 1));
                app.auto_scroll = false;
            }
        }
        KeyCode::Down => {
            let current = app.ai_list_state.selected().unwrap_or(0);
            if current > 0 {
                app.ai_list_state.select(Some(current - 1));
                app.auto_scroll = false;
            }
        }
        KeyCode::PageUp => {
            let current = app.ai_list_state.selected().unwrap_or(0);
            let max_scroll = app.get_max_scroll_offset();
            app.ai_list_state
                .select(Some((current + 10).min(max_scroll)));
            app.auto_scroll = false;
        }
        KeyCode::PageDown => {
            let current = app.ai_list_state.selected().unwrap_or(0);
            app.ai_list_state
                .select(Some(current.saturating_sub(10).max(0)));
            app.auto_scroll = false;
        }
        KeyCode::Home => {
            app.ai_list_state.select(Some(0));
            app.auto_scroll = false;
        }
        KeyCode::End => {
            app.scroll_to_end();
        }
        KeyCode::Char('i') => app.input_mode = InputMode::Editing,
        KeyCode::Char('c') | KeyCode::Char('C') => app.switch_to_chinese(),
        KeyCode::Char('e') | KeyCode::Char('E') => app.switch_to_english(),
        KeyCode::Enter => {
            if !app.input.is_empty() {
                app.send_message();
                app.scroll_to_end();
            }
        }
        KeyCode::F(1) => app.toggle_help(),
        KeyCode::Char('1') => app.change_theme(0),
        KeyCode::Char('2') => app.change_theme(1),
        KeyCode::Char('3') => app.change_theme(2),
        KeyCode::Char('4') => app.change_theme(3),
        KeyCode::Char('q') => return true,
        _ => {}
    }
    false
}

fn handle_editing_mode_event(key: crossterm::event::KeyEvent, app: &mut App) -> bool {
    match key.code {
        KeyCode::Enter => {
            app.send_message();
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Esc => {
            app.clear_input();
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Char(c) => {
            app.input.push(c);
        }
        KeyCode::Backspace => {
            app.input.pop();
        }
        KeyCode::Delete => {
            app.clear_input();
        }
        _ => {}
    }
    false
}

fn handle_help_event(key: crossterm::event::KeyEvent, app: &mut App) -> bool {
    if key.kind == KeyEventKind::Press {
        app.show_help = false;
    }
    false
}
