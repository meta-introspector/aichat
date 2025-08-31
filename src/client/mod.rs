mod access_token;
mod common;
mod message;
#[macro_use]
mod macros;
mod model;
mod stream;

pub use crate::function::ToolCall;
pub use common::*;
pub use message::*;
pub use model::*;
pub use stream::*;
use crate::auth::Authenticator;

register_client!(
    (openai, "openai", OpenAIConfig, OpenAIClient, [("api_key", "API Key", None)]),
    (
        openai_compatible,
        "openai-compatible",
        OpenAICompatibleConfig,
        OpenAICompatibleClient,
        []
    ),
    (gemini, "gemini", GeminiConfig, GeminiClient, []),
    (claude, "claude", ClaudeConfig, ClaudeClient, [("api_key", "API Key", None)]),
    (cohere, "cohere", CohereConfig, CohereClient, [("api_key", "API Key", None)]),
    (azure_openai, "azure-openai", AzureOpenAIConfig, AzureOpenAIClient, [
        ("api_base", "API Base", Some("e.g. https://{RESOURCE}.openai.azure.com")),
        ("api_key", "API Key", None)
    ]),
    (bedrock, "bedrock", BedrockConfig, BedrockClient, []),
);

impl_client_trait!(
    OpenAIClient,
    (
        openai::prepare_chat_completions,
        openai::openai_chat_completions,
        openai::openai_chat_completions_streaming
    ),
    (openai::prepare_embeddings, openai::openai_embeddings),
    (noop_prepare_rerank, noop_rerank),
);

impl_client_trait!(
    OpenAICompatibleClient,
    (
        openai_compatible::prepare_chat_completions,
        openai::openai_chat_completions,
        openai::openai_chat_completions_streaming
    ),
    (openai_compatible::prepare_embeddings, openai::openai_embeddings),
    (openai_compatible::prepare_rerank, openai_compatible::generic_rerank),
);

impl_client_trait!(
    GeminiClient,
    (
        gemini::prepare_chat_completions,
        vertexai::gemini_chat_completions,
        vertexai::gemini_chat_completions_streaming
    ),
    (gemini::prepare_embeddings, gemini::embeddings),
    (noop_prepare_rerank, noop_rerank),
);

impl_client_trait!(
    ClaudeClient,
    (
        claude::prepare_chat_completions,
        claude::claude_chat_completions,
        claude::claude_chat_completions_streaming
    ),
    (noop_prepare_embeddings, noop_embeddings),
    (noop_prepare_rerank, noop_rerank),
);

impl_client_trait!(
    CohereClient,
    (
        cohere::prepare_chat_completions,
        cohere::chat_completions,
        cohere::chat_completions_streaming
    ),
    (cohere::prepare_embeddings, cohere::embeddings),
    (cohere::prepare_rerank, openai_compatible::generic_rerank),
);

impl_client_trait!(
    AzureOpenAIClient,
    (
        azure_openai::prepare_chat_completions,
        openai::openai_chat_completions,
        openai::openai_chat_completions_streaming
    ),
    (azure_openai::prepare_embeddings, openai::openai_embeddings),
    (noop_prepare_rerank, noop_rerank),
);


pub const OPENAI_COMPATIBLE_PROVIDERS: [(&str, &str); 18] = [
    ("ai21", "https://api.ai21.com/studio/v1"),
    (
        "cloudflare",
        "https://api.cloudflare.com/client/v4/accounts/{ACCOUNT_ID}/ai/v1",
    ),
    ("deepinfra", "https://api.deepinfra.com/v1/openai"),
    ("deepseek", "https://api.deepseek.com"),
    ("ernie", "https://qianfan.baidubce.com/v2"),
    ("github", "https://models.inference.ai.azure.com"),
    ("groq", "https://api.groq.com/openai/v1"),
    ("hunyuan", "https://api.hunyuan.cloud.tencent.com/v1"),
    ("minimax", "https://api.minimax.chat/v1"),
    ("mistral", "https://api.mistral.ai/v1"),
    ("moonshot", "https://api.moonshot.cn/v1"),
    ("openrouter", "https://openrouter.ai/api/v1"),
    ("perplexity", "https://api.perplexity.ai"),
    (
        "qianwen",
        "https://dashscope.aliyuncs.com/compatible-mode/v1",
    ),
    ("xai", "https://api.x.ai/v1"),
    ("zhipuai", "https://open.bigmodel.cn/api/paas/v4"),
    // RAG-dedicated
    ("jina", "https://api.jina.ai/v1"),
    ("voyageai", "https://api.voyageai.com/v1"),
];
