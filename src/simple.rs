use crate::ai::{call_real_aliyun_api, call_real_deepseek_api};
use crate::ai_models::AIModel;
use crate::i18n::Language;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use std::io::{self, stdout, Write};
use unicode_width::UnicodeWidthChar;

const USER_COLOR: Color = Color::Cyan;
const AI_COLOR: Color = Color::Green;
const PROMPT_COLOR: Color = Color::Yellow;

pub async fn run_simple_mode(model: AIModel, language: Language) -> io::Result<()> {
    print_welcome(&model, language)?;

    let mut stdout = stdout();
    // Record where input prompt starts (right after welcome banner)
    execute!(stdout, SetForegroundColor(PROMPT_COLOR), Print("> "), SetForegroundColor(USER_COLOR))?;
    stdout.flush()?;

    enable_raw_mode()?;

    let mut input_lines: Vec<String> = Vec::new();
    let mut current_line = String::new();

    loop {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    disable_raw_mode()?;
                    execute!(stdout, ResetColor, Print("\r\n"))?;
                    println!("Bye.");
                    return Ok(());
                }
                KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    if input_lines.is_empty() && current_line.is_empty() {
                        disable_raw_mode()?;
                        execute!(stdout, ResetColor, Print("\r\n"))?;
                        return Ok(());
                    }
                }
                KeyCode::Enter => {
                    if key.modifiers.contains(KeyModifiers::ALT) {
                        execute!(stdout, Print("\r\n  "))?;
                        input_lines.push(std::mem::take(&mut current_line));
                    } else {
                        let mut full_input = input_lines.join("\n");
                        if !full_input.is_empty() {
                            full_input.push('\n');
                        }
                        full_input.push_str(&current_line);

                        if !full_input.trim().is_empty() {
                            disable_raw_mode()?;
                            execute!(stdout, ResetColor, Print("\r\n"))?;
                            execute!(
                                stdout,
                                SetForegroundColor(Color::DarkGrey),
                                Print("...\r\n"),
                                ResetColor,
                            )?;
                            stdout.flush()?;

                            let response = call_ai(&model, &full_input, language).await;

                            execute!(
                                stdout,
                                cursor::MoveUp(1),
                                Clear(ClearType::CurrentLine),
                            )?;

                            print_ai_response(&response)?;
                            execute!(stdout, Print("\n"))?;

                            execute!(stdout, SetForegroundColor(PROMPT_COLOR), Print("> "), SetForegroundColor(USER_COLOR))?;
                            stdout.flush()?;

                            enable_raw_mode()?;
                        } else {
                            execute!(stdout, ResetColor, Print("\r\n"))?;
                            execute!(stdout, SetForegroundColor(PROMPT_COLOR), Print("> "), SetForegroundColor(USER_COLOR))?;
                            stdout.flush()?;
                        }
                        input_lines.clear();
                        current_line.clear();
                    }
                }
                KeyCode::Backspace => {
                    if !handle_backspace(&mut stdout, &mut input_lines, &mut current_line)? {
                        // nothing to delete
                    }
                }
                KeyCode::Char(c) => {
                    current_line.push(c);
                    execute!(stdout, Print(c))?;
                }
                _ => {}
            }
            stdout.flush()?;
        }
    }
}

fn handle_backspace(
    stdout: &mut io::Stdout,
    input_lines: &mut Vec<String>,
    current_line: &mut String,
) -> io::Result<bool> {
    if current_line.is_empty() {
        if let Some(prev) = input_lines.pop() {
            execute!(stdout, cursor::MoveUp(1))?;
            let display_width: u16 = prev.chars().map(|c| char_width(c) as u16).sum();
            execute!(stdout, cursor::MoveToColumn(2 + display_width))?;
            *current_line = prev;
            return Ok(true);
        }
        return Ok(false);
    }
    if let Some(ch) = current_line.pop() {
        let width = char_width(ch) as u16;
        execute!(stdout, cursor::MoveLeft(width))?;
        for _ in 0..width {
            execute!(stdout, Print(" "))?;
        }
        execute!(stdout, cursor::MoveLeft(width))?;
    }
    Ok(true)
}

fn char_width(c: char) -> usize {
    UnicodeWidthChar::width(c).unwrap_or(0)
}

fn print_ai_response(response: &str) -> io::Result<()> {
    let mut stdout = stdout();
    execute!(stdout, SetForegroundColor(AI_COLOR))?;
    for line in response.lines() {
        execute!(stdout, Print(line), Print("\r\n"))?;
    }
    execute!(stdout, ResetColor)?;
    stdout.flush()?;
    Ok(())
}

fn print_welcome(model: &AIModel, language: Language) -> io::Result<()> {
    let model_name = model.name(language);
    let mut stdout = stdout();
    execute!(stdout, Print("\n"))?;
    execute!(stdout, Print("  "), SetForegroundColor(Color::Cyan), Print("aichat --simple"), ResetColor, Print("  "))?;
    execute!(stdout, SetForegroundColor(Color::Green), Print(&model_name), ResetColor, Print("  "))?;
    execute!(
        stdout,
        SetForegroundColor(Color::DarkGrey),
        Print("[Enter] send  [Alt+Enter] newline  [Ctrl+C] quit"),
        ResetColor,
        Print("\n\n"),
    )?;
    stdout.flush()?;
    Ok(())
}

async fn call_ai(model: &AIModel, user_input: &str, language: Language) -> String {
    match model {
        AIModel::DeepSeek => call_real_deepseek_api(user_input, language).await,
        AIModel::AliYun(model_type) => {
            call_real_aliyun_api(user_input, language, *model_type).await
        }
        _ => model.simulate_response(user_input, language),
    }
}
