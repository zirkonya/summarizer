use std::{collections::HashMap, path::PathBuf};

use ollama_rs::{
    Ollama,
    error::OllamaError,
    generation::chat::{
        ChatMessage, ChatMessageResponse, MessageRole, request::ChatMessageRequest,
    },
};

pub struct Synthesizer {
    ollama: Ollama,
}

impl Synthesizer {
    pub fn new() -> Self {
        Self {
            ollama: Ollama::default(),
        }
    }

    pub async fn synthesize(
        &self,
        model: &str,
        summaries: HashMap<PathBuf, String>,
    ) -> Result<ChatMessageResponse, OllamaError> {
        let mut batch = String::new();
        for (path, summary) in summaries {
            batch.push_str(&format!("--- {} ---\n{}", path.to_string_lossy(), summary));
        }
        let message = ChatMessage::new(MessageRole::User, batch);
        let request = ChatMessageRequest::new(model.to_string(), vec![message]);
        self.ollama.send_chat_messages(request).await
    }
}
