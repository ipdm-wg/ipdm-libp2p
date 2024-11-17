use crate::types::message::IpdmMessage;
use libp2p::PeerId;

#[derive(Debug)]
pub enum IpdmEvent {
    MessageReceived {
        peer_id: PeerId,
        message: IpdmMessage,
    },
    PeerDiscovered(PeerId),
    PeerExpired(PeerId),
}