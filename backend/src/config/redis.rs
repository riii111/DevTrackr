use actix::Addr;
use actix_redis::RedisActor;

pub fn create_redis_actor(redis_url: &str) -> Addr<RedisActor> {
    RedisActor::start(redis_url)
}
