use std::{fs, path::PathBuf};

use ollama_rs::{
    Ollama,
    error::OllamaError,
    generation::chat::{
        ChatMessage, ChatMessageResponse, MessageRole, request::ChatMessageRequest,
    },
};

pub struct FileSummarizer {
    ollama: Ollama,
}

impl FileSummarizer {
    pub fn new() -> Self {
        Self {
            ollama: Ollama::default(),
        }
    }

    pub async fn summarize_file(
        &self,
        model: &str,
        path: PathBuf,
    ) -> Result<ChatMessageResponse, OllamaError> {
        let text = fs::read_to_string(&path).unwrap_or(
            "Something went wrong with this file. Please share this error in your summary"
                .to_string(),
        );
        let content = format!("--- {} ---\n{}", path.to_string_lossy(), text);
        let message = ChatMessage::new(MessageRole::User, content);
        let request = ChatMessageRequest::new(model.to_string(), vec![message]);
        self.ollama.send_chat_messages(request).await
    }
}
