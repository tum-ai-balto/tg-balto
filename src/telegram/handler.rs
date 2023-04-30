use teloxide::types::MediaKind;
use teloxide::utils::command::BotCommands;

pub mod callback;
pub mod employee;
pub mod help;
pub mod reset;
pub mod start;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "verify user.", parse_with = "split")]
    Start,
    #[command(description = "Reset user.")]
    Reset,
    #[command(description = "Send table to the employer")]
    Send,
    #[command(description = "Get all employers")]
    Employers,
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum State {
    #[default]
    Start,

    // Employee
    EmployeeWaitingEmployer,
    EmployeeWaitingMedia {
        employer: String,
        medias: Vec<MediaKind>,
    },
    EmployeePendingReport,

    // Employer
    EmployerWaitingLang,
    EmployerWaitingReport {
        lang: String,
        name: String,
    },
}
