use anyhow::Result;
use serde::{Deserialize, Serialize};

pub async fn fetch(recipe: String) -> String {
    let context = "You are a chef, when asked to write a recipe you only
        provide the ingredients and instructions. Only use metric for measurements.";
    let fetch = crate::chatgpt::send_message_with_context(
        context,
        &format!("Write a recipe for {}", recipe),
    )
    .await;

    match fetch {
        Ok(res) => res.content().clone(),
        Err(e) => {
            log::error!("Error: {}", e);
            String::from("Error")
        }
    }
}

pub fn transform(data: &str) -> (Vec<String>, Vec<String>) {
    let content = data.lines().filter(|l| !l.is_empty()).collect::<Vec<_>>();

    let mut ingredients: Vec<String> = vec![];
    for line in &content {
        if line.starts_with("Instructions") {
            break;
        }
        if line.starts_with("Ingredients") {
            continue;
        }
        if let Some(stripped) = line.strip_prefix("- ") {
            ingredients.push(stripped.to_string());
            continue;
        }
        ingredients.push(line.to_string());
    }

    let mut instructions: Vec<String> = vec![];
    let mut skip = true;
    for line in &content {
        if line.starts_with("Instructions") {
            skip = false;
            continue;
        }
        if skip {
            continue;
        }
        instructions.push(line.to_string());
    }

    (ingredients, instructions)
}

pub async fn send_message(message: &str) -> Result<CompletionResponse> {
    cfg_if::cfg_if! {
        if #[cfg(debug_assertions)] {
            let api_key = dotenvy_macro::dotenv!("OPENAI_API_KEY");
        } else {
            let api_key = std::env!("OPENAI_API_KEY");
        }
    }
    Ok(
        reqwasm::http::Request::post("https://api.openai.com/v1/chat/completions")
            .header(
                "Authorization",
                &format!("Bearer {}", api_key),
            )
            .header("content-type", "application/json")
            .body(serde_json::to_string(&CompletionRequest {
                model: "gpt-3.5-turbo",
                messages: &vec![ChatMessage {
                    role: Role::User,
                    content: message.into(),
                }],
                max_tokens: 500,
            })?)
            .send()
            .await?
            .json()
            .await?,
    )
}

pub async fn send_message_with_context(context: &str, message: &str) -> Result<CompletionResponse> {
    send_message(&format!("{} {}", context, message)).await
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    Assistant,
    User,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct CompletionRequest<'a> {
    pub model: &'a str,
    pub messages: &'a Vec<ChatMessage>,
    pub max_tokens: u32,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct CompletionResponse {
    #[serde(rename = "id")]
    pub message_id: String,
    #[serde(rename = "created")]
    pub created_timestamp: u64,
    pub model: String,
    pub usage: TokenUsage,
    #[serde(rename = "choices")]
    pub message_choices: Vec<MessageChoice>,
}

impl CompletionResponse {
    pub fn content(&self) -> &String {
        &self.message_choices.first().unwrap().message.content
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct MessageChoice {
    pub message: ChatMessage,
    pub finish_reason: String,
    pub index: u32,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}
