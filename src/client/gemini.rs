use crate::client::vertexai::*;
use super::*;

use anyhow::{Context, Result};
use reqwest::RequestBuilder;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::auth::Authenticator;

const API_BASE: &str = "https://generativelanguage.googleapis.com/v1beta";

#[derive(Debug, Clone, Deserialize, Default)]
pub struct GeminiConfig {
    pub name: Option<String>,
    pub api_base: Option<String>,
    #[serde(default)]
    pub models: Vec<ModelData>,
    pub patch: Option<RequestPatch>,
    pub extra: Option<ExtraConfig>,
}

pub struct GeminiClient {
    pub model: Model,
    pub config: GeminiConfig,
    pub authenticator: Box<dyn Authenticator + Send + Sync>,
}

impl GeminiClient {
    pub const NAME: &'static str = "gemini";

    pub fn name(local_config: &GeminiConfig) -> &str {
        local_config.name.as_deref().unwrap_or(Self::NAME)
    }

    pub fn new(model: Model, config: GeminiConfig, authenticator: Box<dyn Authenticator + Send + Sync>) -> Self {
        Self { model, config, authenticator }
    }

    pub fn get_api_base(&self) -> anyhow::Result<String> {
        let env_prefix = Self::name(&self.config);
        let env_name =
            format!("{}_{}", env_prefix, stringify!(api_base)).to_ascii_uppercase();
        std::env::var(&env_name)
            .ok()
            .or_else(|| self.config.api_base.clone())
            .ok_or_else(|| anyhow::anyhow!("Miss '{}'", stringify!(api_base)))
    }

    
}


pub async fn prepare_chat_completions(
    self_: &crate::client::GeminiClient,
    data: ChatCompletionsData,
) -> Result<RequestData> {
    let api_key = self_.authenticator.as_ref().context("Authenticator not found")?.authenticate().await?;
    let api_base = self_.get_api_base()
        .unwrap_or_else(|_| API_BASE.to_string());

    let func = match data.stream {
        true => "streamGenerateContent",
        false => "generateContent",
    };

    let url = format!(
        "{}/models/{}:{}",
        api_base.trim_end_matches('/'),
        self_.model.real_name(),
        func
    );

    let body = gemini_build_chat_completions_body(data, &self_.model)?;

    let mut request_data = RequestData::new(url, body);

    request_data.header("x-goog-api-key", api_key);

    Ok(request_data)
}

pub async fn prepare_embeddings(
    self_: &crate::client::GeminiClient,
    data: &EmbeddingsData,
) -> Result<RequestData> {
    let api_key = self_.authenticator.as_ref().context("Authenticator not found")?.authenticate().await?;
    let api_base = self_.get_api_base()
        .unwrap_or_else(|_| API_BASE.to_string());

    let url = format!(
        "{}/models/{}:batchEmbedContents?key={}",
        api_base.trim_end_matches('/'),
        self_.model.real_name(),
        api_key
    );

    let model_id = format!("models/{}", self_.model.real_name());

    let requests: Vec<_> = data
        .texts
        .iter()
        .map(|text| {
            json!({
                "model": model_id,
                "content": {
                    "parts": [
                        {
                            "text": text
                        }
                    ]
                },
            })
        })
        .collect();

    let body = json!({
        "requests": requests,
    });

    let request_data = RequestData::new(url, body);

    Ok(request_data)
}

pub async fn embeddings(builder: RequestBuilder, _model: &Model) -> Result<EmbeddingsOutput> {
    let res = builder.send().await?;
    let status = res.status();
    let data: Value = res.json().await?;
    if !status.is_success() {
        catch_error(&data, status.as_u16())?;
    }
    let res_body: EmbeddingsResBody =
        serde_json::from_value(data).context("Invalid embeddings data")?;
    let output = res_body
        .embeddings
        .into_iter()
        .map(|embedding| embedding.values)
        .collect();
    Ok(output)
}

#[derive(Deserialize)]
struct EmbeddingsResBody {
    embeddings: Vec<EmbeddingsResBodyEmbedding>,
}

#[derive(Deserialize)]
struct EmbeddingsResBodyEmbedding {
    values: Vec<f32>,
}