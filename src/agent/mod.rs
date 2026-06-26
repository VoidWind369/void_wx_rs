use serde::{Deserialize, Serialize};
use serde_json::Value;

/// # Ollama后段Api
pub mod ollama;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clan {
    pub reason: Option<String>,
    pub message: Option<String>,
    pub detail: Option<Value>,
    pub r#type: Option<String>,
    pub tag: Option<String>,            // 标签
    pub name: Option<String>,           // 名称
    pub description: Option<String>,    // 公告
    pub location: Option<ClanLocation>, // 位置
    // pub is_family_friendly: Option<bool>,
    // pub badge_urls: Option<ClanIconUrls>,
    pub clan_level: Option<i64>, // 等级
    // pub clan_points: Option<i64>, // 积分
    // pub clan_builder_base_points: Option<i64>,
    // pub clan_capital_points: Option<i64>,
    pub capital_league: Option<ClanInfo>,
    pub required_trophies: Option<i64>,
    pub war_frequency: Option<String>, // 部落战频率
    pub war_win_streak: Option<i64>,   // 连胜
    pub war_wins: Option<i64>,
    pub war_ties: Option<i64>,
    pub war_losses: Option<i64>,
    // pub is_war_log_public: Option<bool>,
    pub war_league: Option<ClanInfo>,
    pub members: Option<i64>,
    pub member_list: Option<ClanMemberLists>,
    // pub labels: Option<ClanInfoIcons>,
    // pub required_builder_base_trophies: Option<i64>,
    // #[serde(rename = "requiredTownhallLevel")]
    // pub required_town_hall_level: Option<i64>, // 加入所需大本等级
    pub clan_capital: Option<ClanCapital>, // 都城
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanLocation {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub is_country: Option<bool>,
    pub country_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClanIconUrls {
    pub small: Option<String>,
    pub large: Option<String>,
    pub medium: Option<String>,
    pub tiny: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClanInfo {
    pub id: Option<i64>,
    pub name: Option<String>,
}

pub type ClanMemberLists = Vec<ClanMemberList>;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ClanMemberList {
    pub tag: Option<String>,
    pub name: Option<String>,
    pub role: Option<String>,
    pub town_hall_level: Option<i64>,
    // pub exp_level: Option<i64>,
    pub league: Option<ClanInfo>,
    // pub trophies: Option<i64>,
    // pub builder_base_trophies: Option<i64>,
    // pub clan_rank: Option<i64>,
    // pub previous_clan_rank: Option<i64>,
    // pub donations: Option<i64>,
    // pub donations_received: Option<i64>,
    // pub player_house: Option<ClanMemberListPlayerHouse>,
    // pub builder_base_league: Option<ClanInfo>,
    // pub chat_language: Option<ClanChatLanguage>,
}

pub type ClanInfoIcons = Vec<ClanInfoIcon>;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanInfoIcon {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub icon_urls: Option<ClanIconUrls>,
}

pub type ClanMemberListPlayerHouseElements = Vec<ClanMemberListPlayerHouseElement>;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClanMemberListPlayerHouse {
    pub elements: Option<ClanMemberListPlayerHouseElements>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClanMemberListPlayerHouseElement {
    pub r#type: Option<String>,
    pub id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanCapital {
    pub capital_hall_level: Option<i64>,
    pub districts: Option<ClanCapitalDistricts>,
}

pub type ClanCapitalDistricts = Vec<ClanCapitalDistrict>;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanCapitalDistrict {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub district_hall_level: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClanChatLanguage {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub language_code: Option<String>,
}
