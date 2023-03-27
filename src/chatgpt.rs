use anyhow::Result;
use leptos::*;
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
        if line.starts_with("- ") {
            ingredients.push(line[2..].to_string());
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

#[component]
pub fn Test(cx: Scope) -> impl IntoView {
    let (signal, _) = create_signal(cx, String::from("pizza"));
    let res = create_local_resource(cx, signal, fetch);

    let fallback = move |cx, errors: RwSignal<Errors>| {
        let error_list = move || {
            errors.with(|errors| {
                errors
                    .iter()
                    .map(|(_, e)| view! { cx, <li>{e.to_string()}</li>})
                    .collect::<Vec<_>>()
            })
        };

        view! { cx,
            <div class="error">
                <h2>"Error"</h2>
                <ul>{error_list}</ul>
            </div>
        }
    };

    let fetch_view = move || {
        res.with(cx, |data| {
            view! {
                cx,
                <p>{format!("{:#?}", transform(data))}</p>
            }
        })
    };

    view! {
        cx,
        <div>
            <ErrorBoundary fallback>
                <Transition fallback=move || view! { cx, <div>"Loading..."</div>}>
                    {fetch_view}
                </Transition>
            </ErrorBoundary>
        </div>
    }
}

pub async fn send_message(message: &str) -> Result<CompletionResponse> {
    Ok(
        reqwasm::http::Request::post(&format!("https://api.openai.com/v1/chat/completions",))
            .header(
                "Authorization",
                &format!("Bearer {}", dotenvy_macro::dotenv!("OPENAI_API_KEY")),
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
    Ok(send_message(&format!("{} {}", context, message)).await?)
}

pub async fn send_message_with_context_mock(
    _context: &str,
    _message: &str,
) -> Result<CompletionResponse> {
    #[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
    struct Resp {
        message: String,
        status: String,
    }
    let res: Resp = reqwasm::http::Request::get("https://dog.ceo/api/breeds/image/random")
        .send()
        .await?
        .json()
        .await?;

    Ok(CompletionResponse {
        message_id: String::from("1"),
        created_timestamp: 1,
        model: String::from("gpt-3.5-turbo"),
        usage: TokenUsage {
            prompt_tokens: 10,
            completion_tokens: 10,
            total_tokens: 20,
        },
        message_choices: vec![MessageChoice {
            index: 1,
            finish_reason: String::from("test"),
            message: ChatMessage {
                role: Role::User,
                content: res.message,
            },
        }],
    })
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
