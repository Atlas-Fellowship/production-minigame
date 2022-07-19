#[derive(Clone, Debug)]
pub struct Tournament {
    pub tournament_id: i64,
    pub creation_time: i64,
    pub creator_user_id: i64,
    pub max_years: i64,
}

#[derive(Clone, Debug)]
pub struct TournamentData {
    pub tournament_data_id: i64,
    pub creation_time: i64,
    pub creator_user_id: i64,
    pub tournament_id: i64,
    pub title: String,
    pub description: String,
    pub active: bool,
}

#[derive(Clone, Debug)]
pub struct TournamentMembership {
    pub tournament_membership_id: i64,
    pub creation_time: i64,
    pub creator_user_id: i64,
    pub tournament_id: i64,
    pub active: bool,
}

#[derive(Clone, Debug)]
pub struct TournamentSubmission {
    pub tournament_submission_id: i64,
    pub creation_time: i64,
    pub creator_user_id: i64,
    pub tournament_id: i64,
    pub year: i64,
    pub amount: i64,
    pub autogenerated: bool,
}

