use fluent::fluent_args;
use itertools::Itertools;
use rabbit::sender::Sender;
use sea_orm::{ColumnTrait, DbConn, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use teloxide::prelude::*;
use teloxide::types::{MediaKind, MediaPhoto, MediaText, MediaVoice};
use unic_langid::LanguageIdentifier;

use super::super::{BotType, HandlerResult, MyDialogue};
use crate::locale::LocaleManager;
use crate::telegram::handler::State;
use crate::User;
use crate::UserEntity;

#[derive(Serialize, Deserialize)]
struct EmployeeMessage {
    kind: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct GenerateReportRequest {
    employee: String,
    employer: String,
    employee_language: String,
    employer_language: String,
    chat_messages: Vec<EmployeeMessage>,
}

pub(crate) async fn waiting_employer(
    bot: BotType,
    message: Message,
    locale: LocaleManager,
    dialogue: MyDialogue,
) -> HandlerResult {
    let text = locale.get_message("employee", "received-employer", fluent_args![])?;

    // TODO check the employer
    bot.send_message(message.chat.id, text).await?;

    dialogue
        .update(State::EmployeeWaitingMedia {
            employer: message.text().unwrap().to_string(),
            medias: vec![],
        })
        .await?;
    Ok(())
}

pub(crate) async fn waiting_text(message: Message, dialogue: MyDialogue) -> HandlerResult {
    if let Some((employer, mut medias)) = extract_state_from_dialogue(&dialogue).await {
        let media = MediaKind::Text(MediaText {
            text: message.text().unwrap().to_string(),
            entities: Vec::new(),
        });
        medias.push(media);
        dialogue
            .update(State::EmployeeWaitingMedia { employer, medias })
            .await?;
    }

    Ok(())
}

pub(crate) async fn waiting_photo(message: Message, dialogue: MyDialogue) -> HandlerResult {
    if let Some((employer, mut medias)) = extract_state_from_dialogue(&dialogue).await {
        let media = MediaKind::Photo(MediaPhoto {
            photo: message.photo().unwrap().to_vec(),
            caption: None,
            caption_entities: vec![],
            has_media_spoiler: false,
            media_group_id: None,
        });
        medias.push(media);
        dialogue
            .update(State::EmployeeWaitingMedia { employer, medias })
            .await?;
    }

    Ok(())
}

pub(crate) async fn waiting_voice(message: Message, dialogue: MyDialogue) -> HandlerResult {
    if let Some((employer, mut medias)) = extract_state_from_dialogue(&dialogue).await {
        let media = MediaKind::Voice(MediaVoice {
            voice: message.voice().unwrap().clone(),
            caption: None,
            caption_entities: vec![],
        });
        medias.push(media);
        dialogue
            .update(State::EmployeeWaitingMedia { employer, medias })
            .await?;
    }

    Ok(())
}

pub(crate) async fn send_media(
    bot: BotType,
    message: Message,
    locale: LocaleManager,
    dialogue: MyDialogue,
    conn: DbConn,
    sender: Sender,
) -> HandlerResult {
    let state = dialogue.get().await;

    if let State::EmployeeWaitingMedia { employer, medias } = state.unwrap().unwrap() {
        let messages = medias.iter().map(|media| match media {
            MediaKind::Voice(voice) => EmployeeMessage {
                kind: "audio".to_string(),
                content: voice.voice.file.id.clone(),
            },
            MediaKind::Text(content) => EmployeeMessage {
                kind: "text".to_string(),
                content: content.clone().text,
            },
            MediaKind::Photo(photo) => EmployeeMessage {
                kind: "image".to_string(),
                content: photo.photo.last().unwrap().file.id.clone(),
            },
            _ => unreachable!(),
        });

        let employer_user: User = UserEntity::find()
            .filter(crate::user::Column::FullName.contains(employer.as_str()))
            .one(&conn)
            .await?
            .unwrap(); // TODO employer not found case

        let employee_language: LanguageIdentifier = locale.local_locale.into();
        let req = GenerateReportRequest {
            employee: message.chat.id.to_string(),
            employer,
            employee_language: employee_language.to_string(),
            employer_language: employer_user.lang,
            chat_messages: messages.collect_vec(),
        };

        let text = locale.get_message("employee", "sent-media", fluent_args![])?;
        bot.send_message(message.chat.id, text)
            .await?;
        sender
            .push(serde_json::to_string(&req).unwrap().into_bytes())
            .await;
        dialogue.update(State::EmployeePendingReport).await?;
    }

    Ok(())
}

pub(crate) async fn extract_state_from_dialogue(
    dialogue: &MyDialogue,
) -> Option<(String, Vec<MediaKind>)> {
    if let State::EmployeeWaitingMedia { employer, medias } = dialogue.get().await.ok()?? {
        Some((employer, medias))
    } else {
        None
    }
}

pub(crate) async fn get_employers_list(
    bot: BotType,
    message: Message,
    conn: DbConn,
    locale: LocaleManager,
) -> HandlerResult {
    let employers = UserEntity::find()
        .filter(crate::user::Column::Role.contains("employer"))
        .all(&conn)
        .await?;

    let text = locale.get_message("general", "get-employers", fluent_args![
        "employers" => employers.into_iter().map(|x| format!("<code>{}</code>", x.full_name)).join("\n")
    ])?;

    bot.send_message(message.chat.id, text).await;

    Ok(())
}
