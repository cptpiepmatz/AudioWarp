use twilight_gateway::error::ReceiveMessageError;
use twilight_gateway::Shard;
use twilight_model::gateway::event::Event;
use twilight_model::gateway::payload::incoming::Ready;

pub async fn wait_for_ready(shard: &mut Shard) -> Result<Ready, ReceiveMessageError> {
    loop {
        match shard.next_event().await {
            Ok(Event::Ready(event)) => return Ok(*event),
            Ok(_) => (),
            Err(e) => return Err(e)
        }
    }
}
