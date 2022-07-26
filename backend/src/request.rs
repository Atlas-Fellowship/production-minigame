use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TournamentNewProps {
    pub api_key: String,
    pub title: String,
    pub cost_per_unit: i64,
    pub demand_xintercept: i64,
    pub demand_yintercept: i64,
    pub incentive_multiplier: i64,
    pub incentive_start_year: i64,
    pub max_years: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TournamentDataNewProps {
    pub tournament_id: i64,
    pub title: String,
    pub active: bool,
    pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TournamentYearNewProps {
    pub tournament_id: i64,
    pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TournamentMembershipNewProps {
    pub tournament_id: i64,
    pub active: bool,
    pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TournamentSubmissionNewProps {
    pub tournament_id: i64,
    pub amount: i64,
    pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TournamentDataViewProps {
    pub tournament_data_id: Option<Vec<i64>>,
    pub min_creation_time: Option<i64>,
    pub max_creation_time: Option<i64>,
    pub creator_user_id: Option<Vec<i64>>,
    pub tournament_id: Option<Vec<i64>>,
    pub active: Option<bool>,
    pub only_recent: bool,
    pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TournamentYearViewProps {
    pub tournament_year_id: Option<Vec<i64>>,
    pub min_creation_time: Option<i64>,
    pub max_creation_time: Option<i64>,
    pub creator_user_id: Option<Vec<i64>>,
    pub tournament_id: Option<Vec<i64>>,
    pub only_recent: bool,
    pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TournamentMembershipViewProps {
    pub tournament_membership_id: Option<Vec<i64>>,
    pub min_creation_time: Option<i64>,
    pub max_creation_time: Option<i64>,
    pub creator_user_id: Option<Vec<i64>>,
    pub tournament_id: Option<Vec<i64>>,
    pub active: Option<bool>,
    pub only_recent: bool,
    pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TournamentSubmissionViewProps {
    pub tournament_submission_id: Option<Vec<i64>>,
    pub min_creation_time: Option<i64>,
    pub max_creation_time: Option<i64>,
    pub creator_user_id: Option<Vec<i64>>,
    pub tournament_id: Option<Vec<i64>>,
    pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TournamentYearDemandViewProps {
    pub tournament_year_demand_id: Option<Vec<i64>>,
    pub min_creation_time: Option<i64>,
    pub max_creation_time: Option<i64>,
    pub user_id: Option<Vec<i64>>,
    pub tournament_id: Option<Vec<i64>>,
    pub api_key: String,
}
