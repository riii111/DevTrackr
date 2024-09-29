use actix::Addr;
use actix_redis::RedisActor;
use std::io::Result;

pub fn create_redis_actor(redis_url: &str) -> Result<Addr<RedisActor>> {
    Ok(RedisActor::start(redis_url))
}
