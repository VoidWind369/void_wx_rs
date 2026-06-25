use rig::{
    client::{CompletionClient, Nothing, ProviderClient},
    completion::Prompt,
    providers::ollama::Client,
    tool::ToolError,
};
use void_log::log_info;

use crate::app::Config;

#[rig_derive::rig_tool(description = "获取#开头标签的部落信息")]
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
    let agent = client.agent(config_agent.model.unwrap_or("qwen3:0.6b".to_string()))
        .preamble("你现在是一个负责查询和整理游戏《部落冲突》的数据助理，查询到的数据都是json格式，字段是英文的，当我以其他语言问你字段信息的时候，你需要将统计结果以询问的语言展示出来")
        .append_preamble("术语注解：Town Hall是主世界玩法；town_hall_level是大本营等级，又称大本等级；donations指的与捐赠部队有关的信息，玩家圈子俗称捐兵与收兵。对照表不要出现在结果里面")
        .append_preamble("数据整理需要包括部落名称，部落标签，部落首领标签和名称，部落位置及一些基础信息，大本营等级分布统计，联赛段位分布统计，不要展示非首领成员的信息，只关注部落情况")
        .tool(GetTagInfo)
        .build();
    agent.prompt(prompt).await.unwrap_or("没有答案".to_string())
}
