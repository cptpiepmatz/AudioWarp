use async_trait::async_trait;
use songbird::error::JoinResult;
use songbird::id::GuildId;
use songbird::Songbird;

#[async_trait]
pub trait SongbirdUtil {
    async fn clear_calls(&self, guild_id: impl Into<GuildId> + Send) -> JoinResult<()>;
}

#[async_trait]
impl SongbirdUtil for Songbird {
    async fn clear_calls(&self, guild_id: impl Into<GuildId> + Send) -> JoinResult<()> {
        let guild_id = guild_id.into();
        let other_guild_ids: Vec<_> = self
            .iter()
            .filter(|(g_id, _)| *g_id != guild_id)
            .map(|(g_id, _)| g_id)
            .collect();

        for other_guild_id in other_guild_ids {
            self.remove(other_guild_id).await?;
        }

        Ok(())
    }
}
