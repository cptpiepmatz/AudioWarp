use twilight_gateway::error::ReceiveMessageError;
use twilight_gateway::Shard;
use twilight_model::gateway::event::{Event, Event as TwilightGatewayEvent};
use twilight_model::gateway::payload::incoming::{GuildDelete, GuildUpdate, InteractionCreate, Ready, VoiceStateUpdate};
use std::sync::Arc;
use crate::AppContext;

pub async fn wait_for_ready(shard: &mut Shard) -> Result<Ready, ReceiveMessageError> {
    loop {
        match shard.next_event().await {
            Ok(Event::Ready(event)) => return Ok(*event),
            Ok(_) => (),
            Err(e) => return Err(e)
        }
    }
}

pub async fn handle_event(event: TwilightGatewayEvent, ctx: Arc<AppContext>) {
    use twilight_model::gateway::event::Event as E;
    match event {
        E::GuildDelete(event) => handle_guild_delete(event).await,
        E::GuildUpdate(event) => handle_guild_update(*event).await,
        E::VoiceStateUpdate(event) => handle_voice_state_update(*event).await,
        E::InteractionCreate(event) => handle_interaction_create(*event).await,
        _ => ()
    }
}

async fn handle_guild_delete(event: GuildDelete) {
    // TODO: disconnect from that voice, if connected
    todo!()
}

async fn handle_guild_update(event: GuildUpdate) {
    // TODO: update ui indicator
    todo!()
}

async fn handle_voice_state_update(event: VoiceStateUpdate) {
    // TODO: update ui indicator and songbird connection
    todo!()
}

async fn handle_interaction_create(event: InteractionCreate) {
    // TODO: listen to commands
    todo!()
}
