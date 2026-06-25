use rig::{
    client::{CompletionClient, Nothing},
    completion::Prompt,
    providers::ollama::Client,
    tool::ToolError,
};
use serde::{Deserialize, Serialize};
use tokio::io::AsyncReadExt;
use void_log::log_info;

use crate::app::Config;

#[rig_derive::rig_tool(
    description = "通过Api获取标签的部落信息，标签以数字和字母组成，通常为3至10位数"
)]
async fn get_tag_info(tag: String) -> Result<serde_json::Value, ToolError> {
    let api = Config::get().await.get_api();
    println!("[Tool Call] 查询标签 {} 的数据", &tag);
    let tag = tag.trim_start_matches('#');
    let url = format!("https://api.clashofclans.com/v1/clans/%23{}", tag);
    let token = format!("Bearer {}", api.token.unwrap());
    let response = reqwest::Client::new()
        .get(url)
        .header(reqwest::header::AUTHORIZATION, token)
        .send()
        .await
        .unwrap();
    let result = response
        .json::<serde_json::Value>()
        .await
        .unwrap_or_default();
    Ok(result)
}

/// 查询智能体
pub async fn agent_run(prompt: &str) -> String {
    log_info!("调用智能体");
    let config_agent = Config::get().await.get_agent();
    let client = Client::builder()
        .api_key(Nothing)
        .base_url("http://localhost:11434")
        .build()
        .unwrap();
    let agent = client
        .agent(config_agent.model.unwrap_or("qwen3:0.6b".to_string()))
        .preamble(&Preamble::read().await.unwrap_or_default().0)
        .append_preamble("未指明语言时的一切回答必须为中文")
        .tool(GetTagInfo)
        .build();
    agent.prompt(prompt).await.unwrap_or("没有答案".to_string())
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct Preamble(String);

impl Preamble {
    async fn read() -> Option<Self> {
        let mut file = if let Ok(f) = tokio::fs::File::open("preamble.json").await {
            f
        } else {
            return None;
        };
        let mut json_str = String::new();
        if file.read_to_string(&mut json_str).await.is_err() {
            return None;
        };
        if let Ok(str) = serde_json::from_str(&json_str) {
            Some(str)
        } else {
            None
        }
    }
}

#[tokio::test]
async fn test() {
    let a = Preamble::read().await.unwrap_or_default().0;
    println!("{a}")
}
