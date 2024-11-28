use crate::message::reaction_list::DiscordReactionList;
use crate::snowflake::Snowflake;
use author::{DiscordMessageAuthor, DisplayName};
use chrono::{DateTime, Utc};
use content::DiscordMessageContent;
use gpui::{div, Element, IntoElement, ParentElement};
use scope_chat::message::MessageAuthor;
use scope_chat::reaction::ReactionList;
use scope_chat::{async_list::AsyncListItem, message::Message};
use serenity::all::Nonce;

pub mod author;
pub mod content;
pub mod reaction;
pub mod reaction_list;

#[derive(Clone, Debug)]
pub struct DiscordMessage {
  pub content: DiscordMessageContent,
  pub author: DiscordMessageAuthor,
  pub id: Snowflake,
  pub nonce: Option<String>,
  pub creation_time: serenity::model::Timestamp,
  pub reactions: DiscordReactionList,
}

impl DiscordMessage {
  pub fn from_serenity(msg: &serenity::all::Message) -> Self {
    let reactions = msg.reactions.iter().map(reaction::DiscordMessageReaction::from_message).collect::<Vec<_>>();
    if !reactions.is_empty() {
      println!("Reactions: {:?}", reactions);
    }
    DiscordMessage {
      id: Snowflake { content: msg.id.get() },
      author: DiscordMessageAuthor {
        display_name: DisplayName(msg.author.name.clone()),
        icon: msg.author.face(),
        id: msg.author.id.to_string(),
      },
      content: DiscordMessageContent {
        content: msg.content.clone(),
        is_pending: false,
      },
      nonce: msg.nonce.clone().map(|n| match n {
        Nonce::Number(n) => n.to_string(),
        Nonce::String(s) => s,
      }),
      creation_time: msg.timestamp,
      reactions: DiscordReactionList::new(reactions),
    }
  }
}

impl Message for DiscordMessage {
  fn get_author(&self) -> &impl MessageAuthor {
    &self.author
  }

  fn get_content(&self) -> impl Element {
    div()
        .child(self.content.clone().into_element())
        .child(self.reactions.clone())
  }

  fn get_identifier(&self) -> String {
    self.id.to_string()
  }

  fn get_nonce(&self) -> Option<&String> {
    self.nonce.as_ref()
  }

  fn should_group(&self, previous: &Self) -> bool {
    const MAX_DISCORD_MESSAGE_GAP_SECS_FOR_GROUP: i64 = 5 * 60;

    self.creation_time.signed_duration_since(*previous.creation_time).num_seconds() <= MAX_DISCORD_MESSAGE_GAP_SECS_FOR_GROUP
  }

  fn get_timestamp(&self) -> Option<DateTime<Utc>> {
    let ts = self.creation_time.timestamp_millis();

    DateTime::from_timestamp_millis(ts)
  }

  fn get_reactions(&mut self) -> &mut impl ReactionList {
    &mut self.reactions
  }
}

impl AsyncListItem for DiscordMessage {
  type Identifier = Snowflake;

  fn get_list_identifier(&self) -> Self::Identifier {
    self.id
  }
}
