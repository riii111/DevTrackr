use actix::Addr;
use actix_redis::{resp_array, Command, RedisActor, RespValue};
use actix_web::Error;
use std::time::Duration;
use tokio::time::timeout;

pub struct RedisClient {
    actor: Addr<RedisActor>,
}

impl RedisClient {
    pub fn new(actor: Addr<RedisActor>) -> Self {
        Self { actor }
    }

    pub async fn increment_and_get(&self, key: &str, expiry: u64) -> Result<u32, Error> {
        // INCRコマンドを送信し、カウンターをインクリメント
        let incr_result = timeout(
            Duration::from_secs(5), // 5秒のタイムアウトを設定
            self.actor.send(Command(resp_array!["INCR", key])),
        )
        .await
        .map_err(|_| {
            Error::from(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "Redis operation timed out",
            ))
        })?
        .map_err(|e| {
            Error::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Mailbox error: {}", e),
            ))
        })?
        .map_err(|e| {
            Error::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Redis error: {}", e),
            ))
        })?;

        if let RespValue::Integer(count) = incr_result {
            // EXPIREコマンドを送信し、キーの有効期限を設定
            timeout(
                Duration::from_secs(5),
                self.actor
                    .send(Command(resp_array!["EXPIRE", key, expiry.to_string()])),
            )
            .await
            .map_err(|_| {
                Error::from(std::io::Error::new(
                    std::io::ErrorKind::TimedOut,
                    "Redis operation timed out",
                ))
            })?
            .map_err(|e| {
                Error::from(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Mailbox error: {}", e),
                ))
            })?
            .map_err(|e| {
                Error::from(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Redis error: {}", e),
                ))
            })?;

            Ok(count as u32)
        } else {
            Err(Error::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Unexpected response from Redis",
            )))
        }
    }
}
