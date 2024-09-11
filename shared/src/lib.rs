use serde::{Deserialize, Serialize};

const MODEL: &str = "llama3.1:latest";

#[derive(Serialize, Deserialize, Debug)]
struct GenerateResponse {
    model: String,
    created_at: String,
    response: String,
    done: bool,
    done_reason: Option<String>,
    context: Option<Vec<i32>>,
    total_duration: Option<u64>,
    load_duration: Option<u64>,
    prompt_eval_count: Option<u16>,
    prompt_eval_duration: Option<u64>,
    eval_count: Option<u16>,
    eval_duration: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GenereateRequest {
    model: String,
    prompt: String,
    stream: bool,
}

impl GenereateRequest {
    fn new(model: Option<&str>, prompt: &str) -> Self {
        Self {
            model: model.unwrap_or(MODEL).to_string(),
            prompt: prompt.to_string(),
            stream: false,
        }
    }
}

pub struct CompletionAPI {
    model: String,
    url: String,
}

impl CompletionAPI {
    pub fn new() -> Self {
        Self {
            model: String::from("llama3.1:latest"),
            url: String::from("http://localhost:11434/api/generate"),
        }
    }

    pub async fn fetch_completion(
        &self,
        prompt: String,
        text: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let context = String::from("Rewrite the following text while keeping the meaning and structure same. Keep it concise, casual and be natural. Only return the response without anything else\n");
        let prompt = format!("{}{}\n{}", context, prompt, text);
        let request = GenereateRequest::new(Some(&self.model), &prompt[..]);
        let serialized = serde_json::to_string(&request).map_err(|e| e.to_string())?;

        let response: GenerateResponse = reqwest::Client::new()
            .post(&self.url)
            .body(serialized)
            .send()
            .await?
            .json()
            .await?;

        Ok(response.response)
    }
}
