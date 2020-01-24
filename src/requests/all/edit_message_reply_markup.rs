use serde::Serialize;

use super::BotWrapper;
use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{ChatOrInlineMessage, InlineKeyboardMarkup, Message},
    Bot,
};

/// Use this method to edit only the reply markup of messages.
///
/// On success, if edited message is sent by the bot, the edited [`Message`] is
/// returned, otherwise [`True`] is returned.
///
/// [The official docs](https://core.telegram.org/bots/api#editmessagereplymarkup).
///
/// [`Message`]: crate::types::Message
/// [`True`]: crate::types::True
#[serde_with_macros::skip_serializing_none]
#[derive(Eq, PartialEq, Debug, Clone, Serialize)]
pub struct EditMessageReplyMarkup<'a> {
    #[serde(skip_serializing)]
    bot: BotWrapper<'a>,
    #[serde(flatten)]
    chat_or_inline_message: ChatOrInlineMessage,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[async_trait::async_trait]
impl Request for EditMessageReplyMarkup<'_> {
    type Output = Message;

    async fn send(&self) -> ResponseResult<Message> {
        net::request_json(
            self.bot.client(),
            self.bot.token(),
            "editMessageReplyMarkup",
            &self,
        )
        .await
    }
}

impl<'a> EditMessageReplyMarkup<'a> {
    pub(crate) fn new(
        bot: &'a Bot,
        chat_or_inline_message: ChatOrInlineMessage,
    ) -> Self {
        Self {
            bot: BotWrapper(bot),
            chat_or_inline_message,
            reply_markup: None,
        }
    }

    pub fn chat_or_inline_message(mut self, val: ChatOrInlineMessage) -> Self {
        self.chat_or_inline_message = val;
        self
    }

    /// A JSON-serialized object for an [inline keyboard].
    ///
    /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }
}
