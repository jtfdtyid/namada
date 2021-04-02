use anoma::types::Topic;
use libp2p::gossipsub::MessageId;
use libp2p::PeerId;

#[derive(Debug)]
pub struct InternMessage {
    pub peer: PeerId,
    pub topic: Topic,
    pub message_id: MessageId,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub enum NetworkEvent {
    Message(InternMessage),
}
