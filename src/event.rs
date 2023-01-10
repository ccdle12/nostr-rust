use crate::{error::Result, util::unix_u32_now};
use secp256k1::{
    schnorrsig::{KeyPair, PublicKey, Signature},
    Message, Secp256k1,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};

// TODO:
// Use as the source material: https://github.com/fiatjaf/go-nostr/blob/master/event/event.go
//
/// A single object used for all user posts/messages in nostr. Each Event contains
/// the content of the message with supporting metadata suchas the Schnorr PublicKey
/// of the publisher and the publishers Schnorr Signature over the Event.id. The Event.id
/// is a SHA256 hash of the serialized UTF-8 JSON String of the Event (with out the
/// signature field).
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Event {
    /// The sha256 hash of the serialized event data.
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

// TODO: Create constructors for each Kind:
// - new_set_meta_data
// - new_text_note
// - new_recommend_server
// - Validate the content structure
impl Event {
    pub fn new(keypair: &KeyPair, kind: Kind, tags: Vec<Tag>, content: String) -> Result<Event> {
        let secp = Secp256k1::new();
        let pubkey = PublicKey::from_keypair(&secp, keypair);

        let created_at = unix_u32_now()?;

        // event_json is the serialied UTF-8 JSON String that will be SHA256
        // hashed to create the Event.id.
        // TODO: pass tags to json! as an array of non-null strings
        let event_json = json!([0, pubkey, created_at, kind, [], content]).to_string();

        let id = Sha256::digest(&event_json.as_bytes());
        let message_hash = Message::from_slice(id.as_slice())?;
        let sig = secp.schnorrsig_sign(&message_hash, &keypair);

        Ok(Event {
            id: id.into(),
            pubkey,
            created_at,
            kind,
            tags,
            content,
            sig,
        })
    }

    /// Verify the signature and id (hash) of the message against the PublicKey of
    /// the publisher of the message.
    pub fn verify_signature(&self) -> Result<()> {
        Ok(Secp256k1::new().schnorrsig_verify(
            &self.sig,
            &Message::from_slice(&self.id)?,
            &self.pubkey,
        )?)
    }
}

/// Represents the differnet variants of the kind of content the Event message
/// contains.
#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
#[repr(u8)]
pub enum Kind {
    /// Represents a content that is a stringified JSON object describing the
    /// user who created the event. The JSON object should contain the following
    /// format:
    ///
    /// {name: String, about: String, picture: URL-String}
    SetMetaData = 0,

    /// Represents the content as a text, simply a note by the publisher with
    /// an arbitrary message.
    TextNote = 1,

    /// Represents the content as a URL address of a relay that the publisher
    /// wants its users to follow.
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
        let (keypair, _) = Secp256k1::new().generate_schnorrsig_keypair(&mut thread_rng());

        let event = Event::new(&keypair, Kind::SetMetaData, vec![], "some-content".into()).unwrap();
        assert!(event.verify_signature().is_ok());
    }
}
