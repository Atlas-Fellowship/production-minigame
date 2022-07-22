use serde::{Deserialize, Serialize};
use strum::AsRefStr;

#[derive(Clone, Debug, Serialize, Deserialize, AsRefStr)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AppError {
    NoCapability,
    TournamentNonexistent,
    TournamentMaxYearsInvalid,
    TournamentIncentiveStartYearInvalid,
    TournamentSubmissionTestcaseIncomplete,
    TournamentSubmissionTestcaseFails,
    TournamentArchived,
    TournamentStarted,
    TournamentMembershipInvalid,
    TournamentMaxYearsAchieved,
    DecodeError,
    InternalServerError,
    MethodNotAllowed,
    Unauthorized,
    BadRequest,
    NotFound,
    Network,
    Unknown,
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl std::error::Error for AppError {}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tournament {
    pub tournament_id: i64,
    pub creation_time: i64,
    pub creator_user_id: i64,
    pub incentive_start_year: i64,
    pub max_years: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TournamentData {
    pub tournament_data_id: i64,
    pub creation_time: i64,
    pub creator_user_id: i64,
    pub tournament: Tournament,
    pub title: String,
    pub active: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TournamentYear {
    pub tournament_year_id: i64,
    pub creation_time: i64,
    pub creator_user_id: i64,
    pub tournament: Tournament,
    pub current_year: i64,
    pub incentive: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TournamentMembership {
    pub tournament_membership_id: i64,
    pub creation_time: i64,
    pub creator_user_id: i64,
    pub tournament: Tournament,
    pub active: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TournamentSubmission {
    pub tournament_submission_id: i64,
    pub creation_time: i64,
    pub creator_user_id: i64,
    pub tournament: Tournament,
    pub amount: i64,
    pub year: i64,
    pub autogenerated: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub service: String,
    pub version_major: i64,
    pub version_minor: i64,
    pub version_rev: i64,
}
