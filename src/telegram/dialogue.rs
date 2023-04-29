use teloxide::types::MediaKind;
use unic_langid::LanguageIdentifier;

pub mod employee;
pub mod employer;
mod reset;

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum State {
    #[default]
    Start,

    // Employee
    EmployeeWaitingEmployer,
    EmployeeWaitingMedia { employer: String, content: Vec<MediaKind>},

    // Employer
    EmployerWaitingLang,
    EmployerWaitingReport { lang: String },
}
