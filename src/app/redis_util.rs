use redis::{Commands, RedisResult};
use crate::app::{get_config, get_redis_conn};

pub async fn redis_set_access_token(access_token: String) -> RedisResult<()> {
    let mut con = get_redis_conn().await.expect("Redis链接失败");
    let config = get_config().await;
    con.set("snipe_wx_access_token", access_token)?;
    con.expire("snipe_wx_access_token", config.redis_expire)?;
    Ok(())
}

pub async fn redis_get_access_token() -> RedisResult<Option<String>> {
    let mut con = get_redis_conn().await.expect("Redis链接失败");
    let get = con.get::<_, Option<String>>("snipe_wx_access_token")?;
    Ok(get)
}