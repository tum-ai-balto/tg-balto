use crate::telegram::handler::State;
use std::sync::RwLock;
use teloxide::prelude::*;

use super::super::{BotType, HandlerResult, MyDialogue};
use crate::locale::LocaleManager;
use crate::{ModelMessage, UserActive, UserEntity};
use fluent::fluent_args;
use sea_orm::{ActiveModelTrait, DbConn, EntityTrait, Set};
use std::collections::HashMap;
use std::sync::Arc;
use teloxide::types::InputFile;
use uuid::Uuid;

pub async fn callback_handler(
    bot: BotType,
    query: CallbackQuery,
    dialogue: MyDialogue,
    locale: LocaleManager,
    conn: DbConn,
) -> HandlerResult {
    // Tell telegram that we've seen this query, to remove 🕑 icons from the clients.
    bot.answer_callback_query(query.id).await?;

    if query.data.is_none() {
        return Ok(());
    }

    match query.data.unwrap().as_str() {
        "employee" => {
            let user_active = UserActive {
                id: Set(query.from.id.to_string()),
                full_name: Set(query.from.full_name()),
                role: Set("employee".to_string()),
                lang: Set(query.from.language_code.unwrap()),
            };

            let _user = match UserEntity::find_by_id(query.from.id.to_string())
                .one(&conn)
                .await?
            {
                Some(_) => user_active.update(&conn).await?,
                None => user_active.insert(&conn).await?,
            };

            let text = locale.get_message("employee", "select-employer", fluent_args![])?;
            // Edit text of the message to which the buttons were attached
            if let Some(Message { id, chat, .. }) = query.message {
                bot.edit_message_text(chat.id, id, text).await?;
            } else if let Some(id) = query.inline_message_id {
                bot.edit_message_text_inline(id, text).await?;
            }
            dialogue.update(State::EmployeeWaitingEmployer).await?;
        }
        "employer" => {
            let user_active = UserActive {
                id: Set(query.from.id.to_string()),
                full_name: Set(query.from.full_name()),
                role: Set("employer".to_string()),
                lang: Set(query.from.language_code.clone().unwrap()),
            };

            let _user = match UserEntity::find_by_id(query.from.id.to_string())
                .one(&conn)
                .await?
            {
                Some(_) => user_active.update(&conn).await?,
                None => user_active.insert(&conn).await?,
            };

            let text = if let Some(lang) = query.from.language_code.clone() {
                dialogue
                    .update(State::EmployerWaitingReport {
                        lang,
                        name: query.from.full_name().clone(),
                    })
                    .await?;
                locale.get_message("employer", "new-employer", fluent_args![])?
            } else {
                dialogue.update(State::EmployerWaitingLang).await?;
                locale.get_message("employer", "new-employer-lang", fluent_args![])?
            };

            if let Some(Message { id, chat, .. }) = query.message {
                bot.edit_message_text(chat.id, id, text).await?;
            } else if let Some(id) = query.inline_message_id {
                bot.edit_message_text_inline(id, text).await?;
            }
        }
        _ => {}
    }

    Ok(())
}

pub async fn callback_document_handler(
    bot: BotType,
    query: CallbackQuery,
    dialogue: MyDialogue,
    locale: LocaleManager,
    documents_to_approve: Arc<RwLock<HashMap<Uuid, ModelMessage>>>,
) -> HandlerResult {
    bot.answer_callback_query(query.id).await?;

    if query.data.is_none() {
        return Ok(());
    }

    let query_data = query.data.unwrap().to_string();

    // TODO ask for the email, add to the dialogue, and send to the employer
    let uuid = Uuid::parse_str(query_data.as_str()).unwrap();
    let report = documents_to_approve.write().unwrap().remove(&uuid).unwrap();

    use base64::{engine::general_purpose, Engine as _};
    let pdf_bytes = general_purpose::STANDARD
        .decode(report.clone().pdf.as_str())
        .unwrap();
    let pdf = InputFile::memory(pdf_bytes);
    let pdf = pdf.file_name(format!("{}.pdf", report.translated_title));
    let employer_id = ChatId(report.employer.parse::<i64>().unwrap());

    bot.send_document(employer_id, pdf)
        .caption("New Report") // TODO
        .await;
    dialogue
        .update(State::EmployeeWaitingMedia {
            employer: report.employer,
            medias: Vec::new(),
        })
        .await?;

    let text = locale.get_message("employee", "sent-report", fluent_args![])?;
    if let Some(Message { id, chat, .. }) = query.message {
        bot.edit_message_text(chat.id, id, text).await?;
    } else if let Some(id) = query.inline_message_id {
        bot.edit_message_text_inline(id, text).await?;
    }

    Ok(())
}
