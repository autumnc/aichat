use crate::ai::{call_real_aliyun_api, call_real_deepseek_api};
use crate::ai_models::AIModel;
use crate::i18n::Language;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use pulldown_cmark::{Event as MdEvent, Parser, Tag, TagEnd};
use std::io::{self, stdout, Write};
use unicode_width::UnicodeWidthChar;

// fbterm-compatible ANSI codes
const ANSI_RESET: &str = "\x1b[0m";
const ANSI_BOLD: &str = "\x1b[1m";
const ANSI_ITALIC: &str = "\x1b[3m";
const ANSI_UNDERLINE: &str = "\x1b[4m";
const ANSI_STRIKETHROUGH: &str = "\x1b[9m";
const ANSI_CYAN: &str = "\x1b[36m";
const ANSI_GREEN: &str = "\x1b[32m";
const ANSI_YELLOW: &str = "\x1b[33m";
const ANSI_GREY: &str = "\x1b[90m";

pub async fn run_simple_mode(model: AIModel, language: Language) -> io::Result<()> {
    print_welcome(&model, language)?;

    let mut stdout = stdout();
    write!(stdout, "{ANSI_YELLOW}> {ANSI_CYAN}")?;
    stdout.flush()?;

    enable_raw_mode()?;

    let mut input_lines: Vec<String> = Vec::new();
    let mut current_line = String::new();

    loop {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => {
                    disable_raw_mode()?;
                    write!(stdout, "{ANSI_RESET}\r\nBye.\n")?;
                    return Ok(());
                }
                KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    if input_lines.is_empty() && current_line.is_empty() {
                        disable_raw_mode()?;
                        write!(stdout, "{ANSI_RESET}\r\n")?;
                        return Ok(());
                    }
                }
                KeyCode::Enter => {
                    if key.modifiers.contains(KeyModifiers::ALT) {
                        write!(stdout, "\r\n  ")?;
                        input_lines.push(std::mem::take(&mut current_line));
                    } else {
                        let mut full_input = input_lines.join("\n");
                        if !full_input.is_empty() {
                            full_input.push('\n');
                        }
                        full_input.push_str(&current_line);

                        if !full_input.trim().is_empty() {
                            disable_raw_mode()?;
                            write!(stdout, "{ANSI_RESET}\r\n")?;

                            write!(stdout, "{ANSI_GREY}...\r\n{ANSI_RESET}")?;
                            stdout.flush()?;

                            let response = call_ai(&model, &full_input, language).await;

                            execute!(
                                stdout,
                                cursor::MoveUp(1),
                                Clear(ClearType::CurrentLine),
                            )?;

                            print_ai_response(&response)?;
                            write!(stdout, "\n")?;

                            write!(stdout, "{ANSI_YELLOW}> {ANSI_CYAN}")?;
                            stdout.flush()?;

                            enable_raw_mode()?;
                        } else {
                            write!(stdout, "{ANSI_RESET}\r\n")?;
                            write!(stdout, "{ANSI_YELLOW}> {ANSI_CYAN}")?;
                            stdout.flush()?;
                        }
                        input_lines.clear();
                        current_line.clear();
                    }
                }
                KeyCode::Backspace => {
                    if !handle_backspace(&mut stdout, &mut input_lines, &mut current_line)? {}
                }
                KeyCode::Char(c) => {
                    current_line.push(c);
                    write!(stdout, "{c}")?;
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
            write!(stdout, " ")?;
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
    let rendered = render_markdown(response);
    write!(stdout, "{rendered}")?;
    stdout.flush()?;
    Ok(())
}

fn render_markdown(text: &str) -> String {
    let parser = Parser::new(text);
    let mut out = String::new();
    let mut styles = StyleCounts::new();

    // Always start with green base
    styles.inc_green();
    emit_codes(&mut out, &styles);

    for event in parser {
        match event {
            MdEvent::Start(tag) => {
                match tag {
                    Tag::Emphasis => {
                        styles.inc_italic();
                        emit_codes(&mut out, &styles);
                    }
                    Tag::Strong => {
                        styles.inc_bold();
                        emit_codes(&mut out, &styles);
                    }
                    Tag::Strikethrough => {
                        styles.inc_strikethrough();
                        emit_codes(&mut out, &styles);
                    }
                    Tag::CodeBlock(_) => {
                        out.push_str("\r\n");
                        styles.inc_dim();
                        emit_codes(&mut out, &styles);
                    }
                    Tag::Heading { .. } => {
                        out.push_str("\r\n");
                        styles.inc_bold();
                        styles.inc_underline();
                        emit_codes(&mut out, &styles);
                    }
                    Tag::BlockQuote(_) => {
                        out.push_str("\r\n");
                        styles.inc_dim();
                        emit_codes(&mut out, &styles);
                    }
                    Tag::Item => {
                        out.push_str("  • ");
                    }
                    Tag::List(_) => {}
                    Tag::Paragraph => {
                        // Soft newline before paragraphs (but not first)
                        if !out.is_empty()
                            && !out.ends_with("\r\n\r\n")
                            && !out.ends_with("\r\n")
                        {
                            out.push_str("\r\n");
                        }
                    }
                    Tag::Link { .. } => {
                        styles.inc_underline();
                        emit_codes(&mut out, &styles);
                    }
                    _ => {}
                }
            }
            MdEvent::End(tag) => {
                match tag {
                    TagEnd::Emphasis => {
                        styles.dec_italic();
                        emit_codes(&mut out, &styles);
                    }
                    TagEnd::Strong => {
                        styles.dec_bold();
                        emit_codes(&mut out, &styles);
                    }
                    TagEnd::Strikethrough => {
                        styles.dec_strikethrough();
                        emit_codes(&mut out, &styles);
                    }
                    TagEnd::CodeBlock => {
                        styles.dec_dim();
                        emit_codes(&mut out, &styles);
                        out.push_str("\r\n");
                    }
                    TagEnd::Heading(_) => {
                        styles.dec_underline();
                        styles.dec_bold();
                        emit_codes(&mut out, &styles);
                        out.push_str("\r\n");
                    }
                    TagEnd::BlockQuote(_) => {
                        styles.dec_dim();
                        emit_codes(&mut out, &styles);
                        out.push_str("\r\n");
                    }
                    TagEnd::Item => {
                        out.push_str("\r\n");
                    }
                    TagEnd::List(_) => {}
                    TagEnd::Paragraph => {
                        out.push_str("\r\n");
                    }
                    TagEnd::Link => {
                        styles.dec_underline();
                        emit_codes(&mut out, &styles);
                    }
                    _ => {}
                }
            }
            MdEvent::Text(text) => {
                out.push_str(&text);
            }
            MdEvent::Code(code) => {
                styles.inc_dim();
                emit_codes(&mut out, &styles);
                out.push_str(&code);
                styles.dec_dim();
                emit_codes(&mut out, &styles);
            }
            MdEvent::SoftBreak => {
                out.push(' ');
            }
            MdEvent::HardBreak => {
                out.push_str("\r\n");
            }
            MdEvent::Rule => {
                let term_width = crossterm::terminal::size().map(|(w, _)| w as usize).unwrap_or(60);
                let line = "─".repeat(term_width.saturating_sub(2));
                out.push_str(&format!("\r\n{line}\r\n"));
            }
            _ => {}
        }
    }

    // Ensure final reset
    out.push_str(ANSI_RESET);
    out
}

struct StyleCounts {
    bold: u32,
    italic: u32,
    underline: u32,
    strikethrough: u32,
    dim: u32,
    green: u32,
}

impl StyleCounts {
    fn new() -> Self {
        Self {
            bold: 0,
            italic: 0,
            underline: 0,
            strikethrough: 0,
            dim: 0,
            green: 0,
        }
    }

    fn inc_bold(&mut self) { self.bold += 1; }
    fn inc_italic(&mut self) { self.italic += 1; }
    fn inc_underline(&mut self) { self.underline += 1; }
    fn inc_strikethrough(&mut self) { self.strikethrough += 1; }
    fn inc_dim(&mut self) { self.dim += 1; }
    fn inc_green(&mut self) { self.green += 1; }

    fn dec_bold(&mut self) { self.bold = self.bold.saturating_sub(1); }
    fn dec_italic(&mut self) { self.italic = self.italic.saturating_sub(1); }
    fn dec_underline(&mut self) { self.underline = self.underline.saturating_sub(1); }
    fn dec_strikethrough(&mut self) { self.strikethrough = self.strikethrough.saturating_sub(1); }
    fn dec_dim(&mut self) { self.dim = self.dim.saturating_sub(1); }
}

fn emit_codes(out: &mut String, s: &StyleCounts) {
    let any = s.bold > 0
        || s.italic > 0
        || s.underline > 0
        || s.strikethrough > 0
        || s.dim > 0
        || s.green > 0;
    if !any {
        out.push_str(ANSI_RESET);
        return;
    }
    out.push_str(ANSI_RESET);
    if s.green > 0 {
        out.push_str(ANSI_GREEN);
    }
    if s.bold > 0 {
        out.push_str(ANSI_BOLD);
    }
    if s.italic > 0 {
        out.push_str(ANSI_ITALIC);
    }
    if s.underline > 0 {
        out.push_str(ANSI_UNDERLINE);
    }
    if s.strikethrough > 0 {
        out.push_str(ANSI_STRIKETHROUGH);
    }
    if s.dim > 0 {
        out.push_str(ANSI_GREY);
    }
}

fn print_welcome(model: &AIModel, language: Language) -> io::Result<()> {
    let model_name = model.name(language);
    let mut stdout = stdout();
    write!(stdout, "{ANSI_GREEN}{model_name}{ANSI_RESET}  {ANSI_GREY}[Enter] send  [Alt+Enter] newline  [Esc] quit{ANSI_RESET}\n")?;
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
