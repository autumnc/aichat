use chrono::Local;
use ratatui::widgets::{ListState, ScrollbarState};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::ai::{call_real_aliyun_api, call_real_deepseek_api};
use crate::ai_models::AIModel;
use crate::i18n::{Language, Translations};

#[derive(Debug, Clone)]
pub struct Message {
    pub content: String,
    pub sender: Sender,
    pub timestamp: chrono::DateTime<Local>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Sender {
    User,
    AI(AIModel),
    Thinking(AIModel),
}

#[derive(Debug, PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
}

#[derive(Debug, PartialEq)]
pub enum AppState {
    Welcome,
    Chatting,
    Help,
}

pub struct App {
    pub ai_models: Vec<AIModel>,
    pub selected_model_index: usize,
    pub messages: Arc<Mutex<Vec<Message>>>,
    pub input: String,
    pub input_mode: InputMode,
    pub ai_list_state: ListState,
    pub user_list_state: ListState,
    pub ai_scrollbar_state: ScrollbarState,
    pub user_scrollbar_state: ScrollbarState,
    pub theme_index: usize,
    pub cursor_blink_state: bool,
    pub last_blink_time: Instant,
    pub show_help: bool,
    pub notification: Option<String>,
    pub last_update: Instant,
    pub language: Language,
    pub translations: Translations,
    pub app_state: AppState,
    pub thinking_message_index: Option<usize>,
    pub auto_scroll: bool,
    pub model_display_offset: usize,
}

impl App {
    pub fn new() -> Self {
        let ai_models = AIModel::all();
        let language = Language::English;
        let translations = Translations::new(language);
        App {
            ai_models,
            selected_model_index: 0,
            messages: Arc::new(Mutex::new(Vec::new())),
            input: String::new(),
            input_mode: InputMode::Normal,
            ai_list_state: ListState::default(),
            user_list_state: ListState::default(),
            ai_scrollbar_state: ScrollbarState::new(0),
            user_scrollbar_state: ScrollbarState::new(0),
            theme_index: 0,
            cursor_blink_state: true,
            last_blink_time: Instant::now(),
            show_help: false,
            notification: None,
            last_update: Instant::now(),
            language,
            translations,
            app_state: AppState::Welcome,
            thinking_message_index: None,
            auto_scroll: true,
            model_display_offset: 0,
        }
    }

    pub fn update_model_display_offset(&mut self, max_visible: usize) {
        let total_models = self.ai_models.len();
        let max_visible = max_visible.min(total_models);
        if self.selected_model_index < self.model_display_offset {
            self.model_display_offset = self.selected_model_index;
        } else if self.selected_model_index >= self.model_display_offset + max_visible {
            self.model_display_offset = self.selected_model_index - max_visible + 1;
        }
        if self.model_display_offset + max_visible > total_models {
            self.model_display_offset = total_models.saturating_sub(max_visible);
        }
    }

    pub fn current_model(&self) -> AIModel {
        self.ai_models[self.selected_model_index].clone()
    }

    pub fn select_previous_model(&mut self, max_visible: usize) {
        let total_models = self.ai_models.len();
        if self.selected_model_index > 0 {
            self.selected_model_index -= 1;
        } else {
            self.selected_model_index = total_models - 1;
        }
        self.ensure_selected_visible(max_visible);
    }

    pub fn select_next_model(&mut self, max_visible: usize) {
        let total_models = self.ai_models.len();
        if self.selected_model_index < total_models - 1 {
            self.selected_model_index += 1;
        } else {
            self.selected_model_index = 0;
        }
        self.ensure_selected_visible(max_visible);
    }

    pub fn ensure_selected_visible(&mut self, max_visible: usize) {
        let total_models = self.ai_models.len();
        if self.selected_model_index < self.model_display_offset {
            self.model_display_offset = self.selected_model_index;
        } else if self.selected_model_index >= self.model_display_offset + max_visible {
            self.model_display_offset = self.selected_model_index - max_visible + 1;
        }
        if self.model_display_offset + max_visible > total_models {
            self.model_display_offset = total_models.saturating_sub(max_visible);
        }
        if total_models <= max_visible {
            self.model_display_offset = 0;
        }
    }

    pub fn scroll_models_left(&mut self, max_visible: usize) {
        if self.model_display_offset > 0 {
            self.model_display_offset -= 1;
        }
        self.ensure_selected_visible(max_visible);
    }

    pub fn scroll_models_right(&mut self, max_visible: usize) {
        let total_models = self.ai_models.len();
        if self.model_display_offset + max_visible < total_models {
            self.model_display_offset += 1;
        }
        self.ensure_selected_visible(max_visible);
    }

    pub fn calculate_max_visible(&self, available_width: usize) -> usize {
        let avg_model_width = 12;
        let max_visible = (available_width.saturating_sub(4)) / avg_model_width;
        max_visible.max(1)
    }

    pub fn send_message(&mut self) {
        if self.input.trim().is_empty() {
            return;
        }
        self.auto_scroll = true;
        let user_input = self.input.clone();
        let current_model = self.current_model().clone();
        let language = self.language;
        let mut messages = self.messages.lock().unwrap();
        if messages.is_empty() {
            let welcome_message = Message {
                content: self.t("welcome_message"),
                sender: Sender::AI(current_model.clone()),
                timestamp: Local::now(),
            };
            messages.push(welcome_message);
        }
        let user_message = Message {
            content: user_input.clone(),
            sender: Sender::User,
            timestamp: Local::now(),
        };
        messages.push(user_message);
        let thinking_message = Message {
            content: match language {
                Language::Chinese => format!("🤔 {} 正在思考中...", current_model.name(language)),
                Language::English => format!("🤔 {} is thinking...", current_model.name(language)),
            },
            sender: Sender::Thinking(current_model.clone()),
            timestamp: Local::now(),
        };
        messages.push(thinking_message);
        let ai_messages_count = messages
            .iter()
            .filter(|msg| matches!(msg.sender, Sender::AI(_) | Sender::Thinking(_)))
            .count();
        let user_messages_count = messages.len() - ai_messages_count;
        self.ai_scrollbar_state = ScrollbarState::new(ai_messages_count);
        self.user_scrollbar_state = ScrollbarState::new(user_messages_count);
        drop(messages);
        let messages_ref = Arc::clone(&self.messages);
        let model = current_model.clone();
        match model {
            AIModel::DeepSeek => {
                let current_language = self.language;
                tokio::spawn(async move {
                    let response = call_real_deepseek_api(&user_input, current_language).await;
                    Self::process_ai_response(messages_ref, model, response).await;
                });
            }
            AIModel::AliYun(aliyun_model_type) => {
                let current_language = self.language;
                tokio::spawn(async move {
                    let response =
                        call_real_aliyun_api(&user_input, current_language, aliyun_model_type)
                            .await;
                    Self::process_ai_response(messages_ref, model, response).await;
                });
            }
            _ => {
                let language_for_sim = self.language;
                tokio::spawn(async move {
                    tokio::time::sleep(Duration::from_millis(500)).await;
                    let response = model.simulate_response(&user_input, language_for_sim);
                    Self::process_ai_response(messages_ref, model, response).await;
                });
            }
        }
        self.input.clear();
    }

    async fn process_ai_response(
        messages_ref: Arc<Mutex<Vec<Message>>>,
        model: AIModel,
        response: String,
    ) {
        let mut messages = messages_ref.lock().unwrap();
        if let Some(pos) = messages
            .iter()
            .position(|msg| matches!(msg.sender, Sender::Thinking(_)))
        {
            messages.remove(pos);
        }
        let ai_message = Message {
            content: response,
            sender: Sender::AI(model),
            timestamp: Local::now(),
        };
        messages.push(ai_message);
    }

    pub fn get_max_scroll_offset(&self) -> usize {
        let messages = self.messages.lock().unwrap();
        if messages.is_empty() {
            return 0;
        }
        let mut total_lines = 0;
        for msg in messages.iter() {
            total_lines += 1;
            total_lines += msg.content.lines().count();
            total_lines += 1;
        }
        total_lines
    }

    pub fn scroll_up(&mut self) {
        let current = self.ai_list_state.selected().unwrap_or(0);
        if current > 0 {
            self.ai_list_state.select(Some(current - 1));
            self.auto_scroll = false;
        }
    }

    pub fn scroll_down(&mut self) {
        let current = self.ai_list_state.selected().unwrap_or(0);
        self.ai_list_state.select(Some(current + 1));
        self.auto_scroll = false;
    }

    pub fn scroll_to_home(&mut self) {
        self.ai_list_state.select(Some(0));
        self.auto_scroll = false;
    }

    pub fn scroll_to_end(&mut self) {
        let messages = self.messages.lock().unwrap();
        let ai_messages_count = messages
            .iter()
            .filter(|msg| matches!(msg.sender, Sender::AI(_) | Sender::Thinking(_)))
            .count();
        self.ai_list_state
            .select(Some(ai_messages_count.saturating_sub(1)));
        self.auto_scroll = true;
    }

    pub fn toggle_help(&mut self) {
        if self.app_state == AppState::Chatting {
            self.show_help = !self.show_help;
        }
    }

    pub fn change_theme(&mut self, index: usize) {
        if index < 4 {
            self.theme_index = index;
            self.set_notification(self.translations.get("notification_theme_changed"));
        }
    }

    pub fn next_theme(&mut self) {
        self.theme_index = (self.theme_index + 1) % 4;
        self.set_notification(self.translations.get("notification_theme_changed"));
    }

    pub fn previous_theme(&mut self) {
        if self.theme_index == 0 {
            self.theme_index = 3;
        } else {
            self.theme_index -= 1;
        }
        self.set_notification(self.translations.get("notification_theme_changed"));
    }

    pub fn update_cursor_blink(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_blink_time) >= Duration::from_millis(500) {
            self.cursor_blink_state = !self.cursor_blink_state;
            self.last_blink_time = now;
        }
    }

    pub fn set_notification(&mut self, message: String) {
        self.notification = Some(message);
    }

    pub fn clear_notification(&mut self) {
        self.notification = None;
    }

    pub fn clear_input(&mut self) {
        self.input.clear();
    }

    pub fn switch_language(&mut self, lang: Language) {
        self.language = lang;
        self.translations = Translations::new(lang);
        {
            let mut messages = self.messages.lock().unwrap();
            if !messages.is_empty() {
                if let Some(first_msg) = messages.first_mut() {
                    first_msg.content = self.translations.get("welcome_message");
                }
            }
        }
        self.set_notification(self.translations.get("notification_language_changed"));
    }

    pub fn switch_to_chinese(&mut self) {
        self.switch_language(Language::Chinese);
    }

    pub fn switch_to_english(&mut self) {
        self.switch_language(Language::English);
    }

    pub fn t(&self, key: &str) -> String {
        self.translations.get(key)
    }

    pub fn start_chatting(&mut self) {
        self.app_state = AppState::Chatting;
    }

    pub fn show_welcome(&mut self) {
        self.app_state = AppState::Welcome;
    }
}
