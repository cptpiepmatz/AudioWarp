use twilight_gateway::error::ReceiveMessageError;
use twilight_gateway::Shard;
use twilight_model::gateway::event::{Event, Event as TwilightGatewayEvent};
use twilight_model::gateway::payload::incoming::{
    GuildDelete, GuildUpdate, InteractionCreate, Ready, VoiceStateUpdate
};

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

pub async fn handle_event(event: TwilightGatewayEvent, ctx: impl AsRef<AppContext>) {
    let ctx = ctx.as_ref();

    use twilight_model::gateway::event::Event as E;
    match event {
        E::GuildDelete(event) => handle_guild_delete(event, ctx).await,
        E::GuildUpdate(event) => handle_guild_update(*event, ctx).await,
        E::VoiceStateUpdate(event) => handle_voice_state_update(*event, ctx).await,
        E::InteractionCreate(event) => handle_interaction_create(*event, ctx).await,
        _ => ()
    }
}

async fn handle_guild_delete(event: GuildDelete, ctx: &AppContext) {
    // TODO: disconnect from that voice, if connected
    todo!()
}

async fn handle_guild_update(event: GuildUpdate, ctx: &AppContext) {
    // TODO: update ui indicator
    todo!()
}

async fn handle_voice_state_update(event: VoiceStateUpdate, ctx: &AppContext) {
    // TODO: update ui indicator and songbird connection
    todo!()
}

async fn handle_interaction_create(event: InteractionCreate, ctx: &AppContext) {
    // TODO: listen to commands
    todo!()
}
