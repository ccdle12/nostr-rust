use secp256k1::schnorrsig::{KeyPair, PublicKey, Signature};
use secp256k1::{Message, Secp256k1};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::time::SystemTime;

// TODO:
// Use as the source material: https://github.com/fiatjaf/go-nostr/blob/master/event/event.go
//
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Event {
    /// The sha256 hash of the serialized event data,
    pub id: [u8; 32],
    /// The Schnorr Public Key of the creator of the event.
    pub pubkey: PublicKey,
    /// Unix time stamp in seconds of the created event.
    pub created_at: u32,
    /// The type of event represented by pre-defined Enums.
    pub kind: Kind,
    /// A collection of tags for the event.
    pub tags: Vec<Tag>,
    /// Arbitrary content for the event.
    pub content: String,
    /// The Schnorr Signature over the sha256 hash of the event data (the "id" field).
    pub sig: Signature,
}

impl Event {
    pub fn new(keypair: &KeyPair, kind: Kind, tags: Vec<Tag>, content: String) -> Event {
        let secp = Secp256k1::new();
        let pubkey = PublicKey::from_keypair(&secp, keypair);

        // TODO: Move into an internal macro.
        let created_at = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|duration| duration.as_secs() as u32)
            .unwrap();

        let event_json = json!([0, pubkey, created_at, kind, [], content]).to_string();

        let id = Sha256::digest(&event_json.as_bytes());
        let message = Message::from_slice(id.as_slice()).unwrap();
        let sig = secp.schnorrsig_sign(&message, &keypair);

        Event {
            id: id.into(),
            pubkey,
            created_at,
            kind,
            tags,
            content,
            sig,
        }
    }
}

// TODO:
#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[repr(u8)]
pub enum Kind {
    SetMetaData = 0,
    TextNote = 1,
    RecommendServer = 2,
}

// TODO:
#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub struct Tag {}

#[cfg(test)]
mod test {
    use super::*;
    use secp256k1::rand::thread_rng;

    #[test]
    fn init_event() {
        let secp = Secp256k1::new();
        let (keypair, _) = secp.generate_schnorrsig_keypair(&mut thread_rng());

        let event = Event::new(&keypair, Kind::SetMetaData, vec![], "some-content".into());

        let message = Message::from_slice(&event.id).unwrap();
        assert!(secp
            .schnorrsig_verify(&event.sig, &message, &event.pubkey)
            .is_ok());
    }
}
