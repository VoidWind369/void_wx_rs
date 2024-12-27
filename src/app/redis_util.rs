use crate::app::{Config, ConfigRedis};
use redis::{Commands, Connection, RedisResult};
use void_log::log_info;

impl ConfigRedis {
    pub async fn get_redis_conn(&self) -> RedisResult<Connection> {
        let password = urlencoding::encode("Z#2nTt98ve!Q#*RY");
        let client = redis::Client::open(format!(
            "redis://{}:{}@{}",
            self.username.clone().unwrap_or_default(),
            password,
            self.url.clone().unwrap_or_default()
        ))?;
        let con = client.get_connection()?;
        Ok(con)
    }
}

pub async fn redis_set_access_token(access_token: String) {
    let config = Config::get().await.redis.unwrap_or_default();
    let mut con = config.get_redis_conn().await.unwrap();
    let set = con.set::<&str, String, usize>("snipe_wx_access_token", access_token).unwrap();
    let expire = con.expire::<&str, usize>(
        "snipe_wx_access_token",
        config.expire.unwrap_or_default(),
    ).unwrap();
    log_info!("SET {set} | EXPIRE {expire}");
}

pub async fn redis_get_access_token() -> RedisResult<Option<String>> {
    let mut con = Config::get()
        .await
        .get_redis_conn()
        .await
        .expect("Redis链接失败");
    let get = con.get::<_, Option<String>>("snipe_wx_access_token")?;
    Ok(get)
}
