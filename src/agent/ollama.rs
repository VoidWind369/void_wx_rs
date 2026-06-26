use rig::{
    client::{CompletionClient, Nothing},
    completion::{Chat, Prompt},
    message::Message,
    providers::ollama::Client,
    tool::{ToolDyn, ToolError},
};
use serde::{Deserialize, Serialize};
use tokio::io::AsyncReadExt;
use void_log::log_info;

use crate::{agent::Clan, app::Config};

#[rig_derive::rig_tool(
    description = "通过Api获取标签的部落信息，标签以数字和字母组成，通常为3至10位数"
)]
async fn get_tag_info(tag: String) -> Result<Clan, ToolError> {
    let api = Config::get().await.get_api();
    log_info!("[Tool Call] 查询标签 {} 的数据", &tag);
    let tag = tag.trim_start_matches('#');
    let url = format!("https://api.clashofclans.com/v1/clans/%23{}", tag);
    let token = format!("Bearer {}", api.token.unwrap());
    let response = reqwest::Client::new()
        .get(url)
        .header(reqwest::header::AUTHORIZATION, token)
        .send()
        .await
        .unwrap();
    let result = response.json::<Clan>().await.unwrap_or_default();
    log_info!("{:?}", &result);
    Ok(result)
}

fn boxed_tools() -> Vec<Box<dyn ToolDyn>> {
    vec![Box::new(GetTagInfo)]
}

/// 查询智能体
pub async fn agent_run(
    prompt: &str,
    mut messages: Vec<Message>,
) -> Result<String, rig::completion::PromptError> {
    log_info!("调用智能体");

    let preamble = Preamble::read().await.unwrap_or_default();
    let config_agent = Config::get().await.get_agent();
    let client = Client::builder()
        .api_key(Nothing)
        .base_url("http://localhost:11434")
        .build()
        .unwrap();
    let mut agent = client.agent(config_agent.model.unwrap_or("qwen3:0.6b".to_string()));
    if let Some(p) = preamble.first_preamble {
        agent = agent.preamble(&p);
    } else {
        agent = agent.preamble("你现在是一个负责查询和整理游戏《部落冲突》的数据助理，查询到的数据都是json格式，字段是英文的，当我以其他语言问你字段信息的时候，你需要将统计结果以询问的语言展示出来");
    };

    if let Some(ps) = preamble.preambles {
        for preamble in ps {
            agent = agent.append_preamble(&preamble);
        }
    } else {
        agent = agent
            .append_preamble("术语注解：Town Hall是主世界玩法；town_hall_level是大本营等级，又称大本等级；donations指的与捐赠部队有关的信息，玩家圈子俗称捐兵与收兵。对照表不要出现在结果里面")
            .append_preamble("数据整理需要包括部落名称，部落标签，部落首领标签和名称，部落位置及一些基础信息，大本营等级分布统计，联赛段位分布统计，不要展示非首领成员的信息，只关注部落情况")
    }
    let agent = agent.tool(GetTagInfo).max_tokens(20480).build();

    agent.chat(prompt, &mut messages).await
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct Preamble {
    first_preamble: Option<String>,
    preambles: Option<Vec<String>>,
}

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
async fn test2() {
    let messages = Vec::new();
    let a = agent_run("查询标签#q82u2qr9", messages).await.unwrap();
    log_info!("{a}")
}
