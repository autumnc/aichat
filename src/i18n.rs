use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Chinese,
    English,
}

impl Default for Language {
    fn default() -> Self {
        Language::Chinese
    }
}

impl Language {
    pub fn name(&self) -> &str {
        match self {
            Language::Chinese => "中文",
            Language::English => "English",
        }
    }

    pub fn code(&self) -> &str {
        match self {
            Language::Chinese => "zh",
            Language::English => "en",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Translations {
    pub strings: HashMap<String, String>,
}

impl Translations {
    pub fn new(lang: Language) -> Self {
        match lang {
            Language::Chinese => Self::chinese(),
            Language::English => Self::english(),
        }
    }

    fn chinese() -> Self {
        let mut strings = HashMap::new();
        strings.insert("app_title".to_string(), "✨ AI 聊天终端".to_string());
        strings.insert(
            "app_subtitle".to_string(),
            "选择 AI 模型并开始智能对话".to_string(),
        );
        strings.insert(
            "model_selector_title".to_string(),
            "🎯 AI 模型 (← →)".to_string(),
        );
        strings.insert("theme_selector_title".to_string(), "主题 (1-4)".to_string());
        strings.insert(
            "theme_names".to_string(),
            "(蓝色/绿色/橙色/霓虹)".to_string(),
        );
        strings.insert(
            "input_hint_normal".to_string(),
            "💬 按 'i' 开始输入，Enter 发送".to_string(),
        );
        strings.insert(
            "input_hint_editing".to_string(),
            "✏️ 编辑中 (按 Enter 发送，Esc 取消)".to_string(),
        );
        strings.insert(
            "chat_empty".to_string(),
            "还没有消息。开始对话吧！".to_string(),
        );
        strings.insert("language_selector".to_string(), "语言 (C/E)".to_string());

        strings.insert(
            "welcome_title".to_string(),
            "✨ 欢迎使用 AI 聊天终端 ✨".to_string(),
        );
        strings.insert(
            "welcome_subtitle".to_string(),
            "一个多模型、多主题的 TUI 聊天应用".to_string(),
        );
        strings.insert(
            "welcome_feature1".to_string(),
            "🎯 支持 7 种 AI 模型切换".to_string(),
        );
        strings.insert(
            "welcome_feature2".to_string(),
            "🌈 4 种精美主题配色".to_string(),
        );
        strings.insert(
            "welcome_feature3".to_string(),
            "🌍 中英文双语界面".to_string(),
        );
        strings.insert(
            "welcome_feature4".to_string(),
            "💬 实时模拟对话体验".to_string(),
        );
        strings.insert(
            "welcome_feature5".to_string(),
            "⌨️ 快捷键操作，高效便捷".to_string(),
        );
        strings.insert(
            "welcome_start_hint".to_string(),
            "按 Enter 键开始聊天，按 F1 查看帮助".to_string(),
        );
        strings.insert(
            "welcome_press_enter".to_string(),
            "按下 Enter 键开始 →".to_string(),
        );
        strings.insert(
            "help_title".to_string(),
            "📖 帮助 - AI 聊天终端".to_string(),
        );
        strings.insert("help_nav_title".to_string(), "🎯 导航：".to_string());
        strings.insert("help_edit_title".to_string(), "✏️ 编辑模式：".to_string());
        strings.insert("help_theme_title".to_string(), "🌈 主题：".to_string());
        strings.insert("help_tips_title".to_string(), "💡 提示：".to_string());
        strings.insert(
            "help_nav_line1".to_string(),
            "  ← →          切换 AI 模型".to_string(),
        );
        strings.insert(
            "help_nav_line2".to_string(),
            "  ↑ ↓          滚动聊天历史".to_string(),
        );
        strings.insert(
            "help_nav_line3".to_string(),
            "  PageUp/Down  快速滚动".to_string(),
        );
        strings.insert(
            "help_nav_line4".to_string(),
            "  Home/End     跳至顶部/底部".to_string(),
        );
        strings.insert(
            "help_nav_line5".to_string(),
            "  i            进入编辑模式".to_string(),
        );
        strings.insert(
            "help_nav_line6".to_string(),
            "  Enter        发送消息".to_string(),
        );
        strings.insert(
            "help_nav_line7".to_string(),
            "  Esc          取消/退出编辑模式".to_string(),
        );
        strings.insert(
            "help_nav_line8".to_string(),
            "  F1           显示/隐藏帮助".to_string(),
        );
        strings.insert(
            "help_nav_line9".to_string(),
            "  C/E          切换中英文".to_string(),
        );
        strings.insert(
            "help_nav_line10".to_string(),
            "  q            退出应用".to_string(),
        );
        strings.insert(
            "help_edit_line1".to_string(),
            "  输入消息后按 Enter 发送".to_string(),
        );
        strings.insert(
            "help_edit_line2".to_string(),
            "  Esc 取消编辑并清空输入".to_string(),
        );
        strings.insert(
            "help_edit_line3".to_string(),
            "  Delete 清空整行".to_string(),
        );
        strings.insert(
            "help_edit_line4".to_string(),
            "  Backspace 删除上一个字符".to_string(),
        );
        strings.insert("help_theme_line1".to_string(), "  1 - 深蓝海洋".to_string());
        strings.insert("help_theme_line2".to_string(), "  2 - 森林绿".to_string());
        strings.insert("help_theme_line3".to_string(), "  3 - 日落橙".to_string());
        strings.insert("help_theme_line4".to_string(), "  4 - 霓虹赛博".to_string());
        strings.insert(
            "help_tips_line1".to_string(),
            "  • 每个 AI 模型都有独特的响应风格".to_string(),
        );
        strings.insert(
            "help_tips_line2".to_string(),
            "  • 消息仅保存在内存中".to_string(),
        );
        strings.insert(
            "help_tips_line3".to_string(),
            "  • 切换主题改变界面氛围".to_string(),
        );
        strings.insert(
            "help_tips_line4".to_string(),
            "  • 按 C/E 键切换中英文界面".to_string(),
        );

        strings.insert(
            "help_close_hint".to_string(),
            "按任意键关闭帮助".to_string(),
        );
        strings.insert("notification_title".to_string(), "💡 通知".to_string());
        strings.insert(
            "notification_continue".to_string(),
            "按任意键继续...".to_string(),
        );
        strings.insert("welcome_message".to_string(), "欢迎使用 AI 聊天终端！使用左右键切换 AI 模型，输入消息后按 Enter 发送。按 F1 显示帮助，按 C/E 切换中英文。".to_string());
        strings.insert(
            "notification_language_changed".to_string(),
            "语言已切换为中文".to_string(),
        );
        strings.insert(
            "notification_theme_changed".to_string(),
            "主题已更改".to_string(),
        );
        Self { strings }
    }

    fn english() -> Self {
        let mut strings = HashMap::new();
        strings.insert(
            "app_title".to_string(),
            "✨ AI Chat Terminal".to_string(),
        );
        strings.insert(
            "app_subtitle".to_string(),
            "Select AI Model and Start Intelligent Conversation".to_string(),
        );
        strings.insert(
            "model_selector_title".to_string(),
            "🎯 AI Models (← →)".to_string(),
        );
        strings.insert(
            "theme_selector_title".to_string(),
            "Themes (1-4)".to_string(),
        );
        strings.insert(
            "theme_names".to_string(),
            "(Blue/Green/Orange/Neon)".to_string(),
        );
        strings.insert(
            "input_hint_normal".to_string(),
            "💬 Type 'i' to start typing, then press Enter to send".to_string(),
        );
        strings.insert(
            "input_hint_editing".to_string(),
            "✏️ Editing (Press Enter to send, Esc to cancel)".to_string(),
        );
        strings.insert(
            "chat_empty".to_string(),
            "No messages yet. Start a conversation!".to_string(),
        );
        strings.insert(
            "language_selector".to_string(),
            "Language (C/E)".to_string(),
        );
        strings.insert(
            "welcome_title".to_string(),
            "✨ Welcome to AI Chat Terminal ✨".to_string(),
        );
        strings.insert(
            "welcome_subtitle".to_string(),
            "A Multi-Model, Multi-Theme TUI Chat Application".to_string(),
        );
        strings.insert(
            "welcome_feature1".to_string(),
            "🎯 Support 7 AI Model Switching".to_string(),
        );
        strings.insert(
            "welcome_feature2".to_string(),
            "🌈 4 Beautiful Theme Colors".to_string(),
        );
        strings.insert(
            "welcome_feature3".to_string(),
            "🌍 Chinese/English Bilingual Interface".to_string(),
        );
        strings.insert(
            "welcome_feature4".to_string(),
            "💬 Real-time Simulated Chat Experience".to_string(),
        );
        strings.insert(
            "welcome_feature5".to_string(),
            "⌨️ Hotkey Operation, Efficient and Convenient".to_string(),
        );
        strings.insert(
            "welcome_start_hint".to_string(),
            "Press Enter to start chatting, F1 for help".to_string(),
        );
        strings.insert(
            "welcome_press_enter".to_string(),
            "Press Enter to Start →".to_string(),
        );
        strings.insert(
            "help_title".to_string(),
            "📖 Help - AI Chat Terminal".to_string(),
        );
        strings.insert("help_nav_title".to_string(), "🎯 Navigation:".to_string());
        strings.insert("help_edit_title".to_string(), "✏️ Edit Mode:".to_string());
        strings.insert("help_theme_title".to_string(), "🌈 Themes:".to_string());
        strings.insert("help_tips_title".to_string(), "💡 Tips:".to_string());
        strings.insert(
            "help_nav_line1".to_string(),
            "  ← →          Switch AI Models".to_string(),
        );
        strings.insert(
            "help_nav_line2".to_string(),
            "  ↑ ↓          Scroll chat history".to_string(),
        );
        strings.insert(
            "help_nav_line3".to_string(),
            "  PageUp/Down  Fast scroll".to_string(),
        );
        strings.insert(
            "help_nav_line4".to_string(),
            "  Home/End     Jump to top/bottom".to_string(),
        );
        strings.insert(
            "help_nav_line5".to_string(),
            "  i            Enter edit mode".to_string(),
        );
        strings.insert(
            "help_nav_line6".to_string(),
            "  Enter        Send message".to_string(),
        );
        strings.insert(
            "help_nav_line7".to_string(),
            "  Esc          Cancel/Exit edit mode".to_string(),
        );
        strings.insert(
            "help_nav_line8".to_string(),
            "  F1           Show/Hide help".to_string(),
        );
        strings.insert(
            "help_nav_line9".to_string(),
            "  C/E          Switch Chinese/English".to_string(),
        );
        strings.insert(
            "help_nav_line10".to_string(),
            "  q            Quit application".to_string(),
        );
        strings.insert(
            "help_edit_line1".to_string(),
            "  Type your message and press Enter to send".to_string(),
        );
        strings.insert(
            "help_edit_line2".to_string(),
            "  Esc cancels editing and clears input".to_string(),
        );
        strings.insert(
            "help_edit_line3".to_string(),
            "  Delete clears the entire input line".to_string(),
        );
        strings.insert(
            "help_edit_line4".to_string(),
            "  Backspace deletes the last character".to_string(),
        );
        strings.insert(
            "help_theme_line1".to_string(),
            "  1 - Deep Blue Ocean".to_string(),
        );
        strings.insert(
            "help_theme_line2".to_string(),
            "  2 - Forest Green".to_string(),
        );
        strings.insert(
            "help_theme_line3".to_string(),
            "  3 - Sunset Orange".to_string(),
        );
        strings.insert(
            "help_theme_line4".to_string(),
            "  4 - Neon Cyber".to_string(),
        );
        strings.insert(
            "help_tips_line1".to_string(),
            "  • Each AI model has unique response style".to_string(),
        );
        strings.insert(
            "help_tips_line2".to_string(),
            "  • Messages are saved in memory only".to_string(),
        );
        strings.insert(
            "help_tips_line3".to_string(),
            "  • Switch themes to change the mood".to_string(),
        );
        strings.insert(
            "help_tips_line4".to_string(),
            "  • Press C/E to switch Chinese/English".to_string(),
        );
        strings.insert(
            "help_close_hint".to_string(),
            "Press any key to close help".to_string(),
        );
        strings.insert(
            "notification_title".to_string(),
            "💡 Notification".to_string(),
        );
        strings.insert(
            "notification_continue".to_string(),
            "Press any key to continue...".to_string(),
        );
        strings.insert("welcome_message".to_string(), "Welcome to AI Chat Terminal! Use left/right arrows to switch AI models, press Enter to send messages. Press F1 for help, C/E to switch languages.".to_string());
        strings.insert(
            "notification_language_changed".to_string(),
            "Language changed to English".to_string(),
        );
        strings.insert(
            "notification_theme_changed".to_string(),
            "Theme changed".to_string(),
        );
        Self { strings }
    }

    pub fn get(&self, key: &str) -> String {
        self.strings
            .get(key)
            .cloned()
            .unwrap_or_else(|| format!("[MISSING: {}]", key))
    }
}
