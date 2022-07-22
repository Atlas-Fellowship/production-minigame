use crate::tournament_year_service;

use super::Db;
use auth_service_api::client::AuthService;
use auth_service_api::response::AuthError;
use auth_service_api::response::User;

use super::request;
use super::response;

use super::db_types::*;
use super::utils;

use super::tournament_data_service;
use super::tournament_membership_service;
use super::tournament_service;
use super::tournament_submission_service;

use std::collections::HashSet;
use std::error::Error;

use super::Config;

pub fn report_postgres_err(e: tokio_postgres::Error) -> response::AppError {
    utils::log(utils::Event {
        msg: e.to_string(),
        source: e.source().map(|e| e.to_string()),
        severity: utils::SeverityKind::Error,
    });
    response::AppError::InternalServerError
}

fn report_auth_err(e: AuthError) -> response::AppError {
    match e {
        AuthError::ApiKeyNonexistent => response::AppError::Unauthorized,
        AuthError::ApiKeyUnauthorized => response::AppError::Unauthorized,
        c => {
            let ae = match c {
                AuthError::InternalServerError => response::AppError::InternalServerError,
                AuthError::MethodNotAllowed => response::AppError::InternalServerError,
                AuthError::BadRequest => response::AppError::InternalServerError,
                AuthError::Network => response::AppError::InternalServerError,
                _ => response::AppError::Unknown,
            };

            utils::log(utils::Event {
                msg: ae.as_ref().to_owned(),
                source: Some(format!("auth service: {}", c.as_ref())),
                severity: utils::SeverityKind::Error,
            });

            ae
        }
    }
}

async fn fill_tournament(
    _con: &mut tokio_postgres::Client,
    tournament: Tournament,
) -> Result<response::Tournament, response::AppError> {
    Ok(response::Tournament {
        tournament_id: tournament.tournament_id,
        creation_time: tournament.creation_time,
        creator_user_id: tournament.creator_user_id,
        incentive_start_year: tournament.incentive_start_year,
        max_years: tournament.max_years,
    })
}

async fn fill_tournament_data(
    con: &mut tokio_postgres::Client,
    tournament_data: TournamentData,
) -> Result<response::TournamentData, response::AppError> {
    let tournament = tournament_service::get_by_tournament_id(con, tournament_data.tournament_id)
        .await
        .map_err(report_postgres_err)?
        .ok_or(response::AppError::TournamentNonexistent)?;

    Ok(response::TournamentData {
        tournament_data_id: tournament_data.tournament_data_id,
        creation_time: tournament_data.creation_time,
        creator_user_id: tournament_data.creator_user_id,
        tournament: fill_tournament(con, tournament).await?,
        title: tournament_data.title,
        active: tournament_data.active,
    })
}

async fn fill_tournament_year(
    con: &mut tokio_postgres::Client,
    tournament_year: TournamentYear,
) -> Result<response::TournamentYear, response::AppError> {
    let tournament = tournament_service::get_by_tournament_id(con, tournament_year.tournament_id)
        .await
        .map_err(report_postgres_err)?
        .ok_or(response::AppError::TournamentNonexistent)?;

    Ok(response::TournamentYear {
        tournament_year_id: tournament_year.tournament_year_id,
        creation_time: tournament_year.creation_time,
        creator_user_id: tournament_year.creator_user_id,
        tournament: fill_tournament(con, tournament).await?,
        current_year: tournament_year.current_year,
        incentive: tournament_year.incentive,
    })
}

async fn fill_tournament_membership(
    con: &mut tokio_postgres::Client,
    tournament_membership: TournamentMembership,
) -> Result<response::TournamentMembership, response::AppError> {
    let tournament =
        tournament_service::get_by_tournament_id(con, tournament_membership.tournament_id)
            .await
            .map_err(report_postgres_err)?
            .ok_or(response::AppError::TournamentNonexistent)?;

    Ok(response::TournamentMembership {
        tournament_membership_id: tournament_membership.tournament_membership_id,
        creation_time: tournament_membership.creation_time,
        creator_user_id: tournament_membership.creator_user_id,
        tournament: fill_tournament(con, tournament).await?,
        active: tournament_membership.active,
    })
}

async fn fill_tournament_submission(
    con: &mut tokio_postgres::Client,
    tournament_submission: TournamentSubmission,
) -> Result<response::TournamentSubmission, response::AppError> {
    let tournament =
        tournament_service::get_by_tournament_id(con, tournament_submission.tournament_id)
            .await
            .map_err(report_postgres_err)?
            .ok_or(response::AppError::TournamentNonexistent)?;

    Ok(response::TournamentSubmission {
        tournament_submission_id: tournament_submission.tournament_submission_id,
        creation_time: tournament_submission.creation_time,
        creator_user_id: tournament_submission.creator_user_id,
        tournament: fill_tournament(con, tournament).await?,
        year: tournament_submission.year,
        amount: tournament_submission.amount,
        autogenerated: tournament_submission.autogenerated,
    })
}

pub async fn get_user_if_api_key_valid(
    auth_service: &auth_service_api::client::AuthService,
    api_key: String,
) -> Result<User, response::AppError> {
    auth_service
        .get_user_by_api_key_if_valid(api_key)
        .await
        .map_err(report_auth_err)
}

pub async fn tournament_new(
    _config: Config,
    db: Db,
    auth_service: AuthService,
    props: request::TournamentNewProps,
) -> Result<response::TournamentData, response::AppError> {
    // validate api key
    let user = get_user_if_api_key_valid(&auth_service, props.api_key).await?;

    let con = &mut *db.lock().await;

    if props.max_years <= 1 {
        return Err(response::AppError::TournamentMaxYearsInvalid);
    }

    if props.incentive_start_year <= 1 || props.incentive_start_year >= props.max_years {
        return Err(response::AppError::TournamentIncentiveStartYearInvalid);
    }

    let mut sp = con.transaction().await.map_err(report_postgres_err)?;

    // create tournament
    let tournament = tournament_service::add(
        &mut sp,
        user.user_id,
        props.incentive_start_year,
        props.max_years,
    )
    .await
    .map_err(report_postgres_err)?;

    // create tournament data
    let tournament_data = tournament_data_service::add(
        &mut sp,
        user.user_id,
        tournament.tournament_id,
        props.title,
        true,
    )
    .await
    .map_err(report_postgres_err)?;

    // create year
    let tournament_year =
        tournament_year_service::add(&mut sp, user.user_id, tournament.tournament_id, 0, 0)
            .await
            .map_err(report_postgres_err)?;

    sp.commit().await.map_err(report_postgres_err)?;

    // return json
    fill_tournament_data(con, tournament_data).await
}

pub async fn tournament_data_new(
    _config: Config,
    db: Db,
    auth_service: AuthService,
    props: request::TournamentDataNewProps,
) -> Result<response::TournamentData, response::AppError> {
    // validate api key
    let user = get_user_if_api_key_valid(&auth_service, props.api_key).await?;

    let con = &mut *db.lock().await;

    let mut sp = con.transaction().await.map_err(report_postgres_err)?;

    // ensure that tournament exists and belongs to you
    let tournament = tournament_service::get_by_tournament_id(&mut sp, props.tournament_id)
        .await
        .map_err(report_postgres_err)?
        .ok_or(response::AppError::TournamentNonexistent)?;
    // validate tournament is owned by correct user
    if tournament.creator_user_id != user.user_id {
        return Err(response::AppError::TournamentNonexistent);
    }

    // create tournament data
    let tournament_data = tournament_data_service::add(
        &mut sp,
        user.user_id,
        tournament.tournament_id,
        props.title,
        props.active,
    )
    .await
    .map_err(report_postgres_err)?;

    sp.commit().await.map_err(report_postgres_err)?;

    // return json
    fill_tournament_data(con, tournament_data).await
}

pub async fn tournament_year_new(
    _config: Config,
    db: Db,
    auth_service: AuthService,
    props: request::TournamentYearNewProps,
) -> Result<response::TournamentYear, response::AppError> {
    // validate api key
    let user = get_user_if_api_key_valid(&auth_service, props.api_key).await?;

    let con = &mut *db.lock().await;

    let mut sp = con.transaction().await.map_err(report_postgres_err)?;

    // ensure that tournament exists and belongs to you
    let tournament = tournament_service::get_by_tournament_id(&mut sp, props.tournament_id)
        .await
        .map_err(report_postgres_err)?
        .ok_or(response::AppError::TournamentNonexistent)?;

    // validate tournament is owned by correct user
    if tournament.creator_user_id != user.user_id {
        return Err(response::AppError::TournamentNonexistent);
    }

    // validate tournament is still active
    let tournament_data =
        tournament_data_service::get_recent_by_tournament_id(&mut sp, props.tournament_id)
            .await
            .map_err(report_postgres_err)?
            .ok_or(response::AppError::TournamentNonexistent)?;

    if !tournament_data.active {
        return Err(response::AppError::TournamentArchived);
    }

    // get old tournament year
    let tournament_year =
        tournament_year_service::get_recent_by_tournament_id(&mut sp, props.tournament_id)
            .await
            .map_err(report_postgres_err)?
            .ok_or(response::AppError::TournamentNonexistent)?;

    // if max years hit
    if tournament_year.current_year >= tournament.max_years {
        return Err(response::AppError::TournamentMaxYearsAchieved);
    }

    // TODO: add other tournament submissions if they're not created yet

    let mut users_who_didnt_submit = HashSet::new();

    // add all members to a hashmap marked false for now
    for membership in tournament_membership_service::get_recent_by_tournament(
        &mut sp,
        tournament_data.tournament_id,
    )
    .await
    .map_err(report_postgres_err)?
    {
        users_who_didnt_submit.insert(membership.creator_user_id);
    }

    for submission in tournament_submission_service::get_recent_by_tournament(
        &mut sp,
        tournament_data.tournament_id,
    )
    .await
    .map_err(report_postgres_err)?
    {
        if submission.year == tournament_year.current_year {
            users_who_didnt_submit.remove(&submission.creator_user_id);
        }
    }

    for user_id in users_who_didnt_submit {
        // create tournament submission
        tournament_submission_service::add(
            &mut sp,
            user_id,
            tournament.tournament_id,
            tournament_year.current_year,
            0,
            true,
        )
        .await
        .map_err(report_postgres_err)?;
    }

    // create tournament data
    let tournament_year = tournament_year_service::add(
        &mut sp,
        user.user_id,
        tournament.tournament_id,
        tournament_year.current_year + 1,
        0,
    )
    .await
    .map_err(report_postgres_err)?;

    sp.commit().await.map_err(report_postgres_err)?;

    // return json
    fill_tournament_year(con, tournament_year).await
}

pub async fn tournament_membership_new(
    _config: Config,
    db: Db,
    auth_service: AuthService,
    props: request::TournamentMembershipNewProps,
) -> Result<response::TournamentMembership, response::AppError> {
    // validate api key
    let user = get_user_if_api_key_valid(&auth_service, props.api_key).await?;

    let con = &mut *db.lock().await;

    let mut sp = con.transaction().await.map_err(report_postgres_err)?;

    // ensure that tournament exists and belongs to you
    let tournament = tournament_service::get_by_tournament_id(&mut sp, props.tournament_id)
        .await
        .map_err(report_postgres_err)?
        .ok_or(response::AppError::TournamentNonexistent)?;

    // cannot create a membership if you created the tournament
    if tournament.creator_user_id == user.user_id {
        return Err(response::AppError::TournamentMembershipInvalid);
    }

    // validate that the tournament isn't archived
    let tournament_data =
        tournament_data_service::get_recent_by_tournament_id(&mut sp, props.tournament_id)
            .await
            .map_err(report_postgres_err)?
            .ok_or(response::AppError::TournamentNonexistent)?;

    if !tournament_data.active {
        return Err(response::AppError::TournamentArchived);
    }

    // also validate that we haven't started the game yet
    let tournament_year =
        tournament_year_service::get_recent_by_tournament_id(&mut sp, props.tournament_id)
            .await
            .map_err(report_postgres_err)?
            .ok_or(response::AppError::TournamentNonexistent)?;

    if !tournament_year.current_year > 0 {
        return Err(response::AppError::TournamentStarted);
    }

    // create tournament membership
    let tournament_membership = tournament_membership_service::add(
        &mut sp,
        user.user_id,
        tournament.tournament_id,
        props.active,
    )
    .await
    .map_err(report_postgres_err)?;

    sp.commit().await.map_err(report_postgres_err)?;

    // return json
    fill_tournament_membership(con, tournament_membership).await
}

pub async fn tournament_submission_new(
    _config: Config,
    db: Db,
    auth_service: AuthService,
    props: request::TournamentSubmissionNewProps,
) -> Result<response::TournamentSubmission, response::AppError> {
    // validate api key
    let user = get_user_if_api_key_valid(&auth_service, props.api_key).await?;

    let con = &mut *db.lock().await;

    let mut sp = con.transaction().await.map_err(report_postgres_err)?;

    // ensure that tournament exists
    let tournament = tournament_service::get_by_tournament_id(&mut sp, props.tournament_id)
        .await
        .map_err(report_postgres_err)?
        .ok_or(response::AppError::TournamentNonexistent)?;

    // validate user is member
    let tournament_membership = tournament_membership_service::get_recent_by_tournament_user(
        &mut sp,
        props.tournament_id,
        user.user_id,
    )
    .await
    .map_err(report_postgres_err)?
    .ok_or(response::AppError::Unauthorized)?;

    // validate that the tournament isn't archived
    let tournament_data =
        tournament_data_service::get_recent_by_tournament_id(&mut sp, props.tournament_id)
            .await
            .map_err(report_postgres_err)?
            .ok_or(response::AppError::TournamentNonexistent)?;

    if !tournament_data.active {
        return Err(response::AppError::TournamentArchived);
    }

    // get current year from tournament_data
    let tournament_year =
        tournament_year_service::get_recent_by_tournament_id(&mut sp, props.tournament_id)
            .await
            .map_err(report_postgres_err)?
            .ok_or(response::AppError::TournamentNonexistent)?;

    // create tournament submission
    let tournament_submission = tournament_submission_service::add(
        &mut sp,
        user.user_id,
        tournament.tournament_id,
        tournament_year.current_year,
        props.amount,
        false,
    )
    .await
    .map_err(report_postgres_err)?;

    sp.commit().await.map_err(report_postgres_err)?;

    // return json
    fill_tournament_submission(con, tournament_submission).await
}

pub async fn tournament_data_view(
    _config: Config,
    db: Db,
    _auth_service: AuthService,
    props: request::TournamentDataViewProps,
) -> Result<Vec<response::TournamentData>, response::AppError> {
    let con = &mut *db.lock().await;
    // get users
    let tournament_data = tournament_data_service::query(con, props)
        .await
        .map_err(report_postgres_err)?;

    // return tournament_datas
    let mut resp_tournament_datas = vec![];
    for u in tournament_data.into_iter() {
        resp_tournament_datas.push(fill_tournament_data(con, u).await?);
    }

    Ok(resp_tournament_datas)
}

pub async fn tournament_membership_view(
    _config: Config,
    db: Db,
    _auth_service: AuthService,
    props: request::TournamentMembershipViewProps,
) -> Result<Vec<response::TournamentMembership>, response::AppError> {
    let con = &mut *db.lock().await;
    // get users
    let tournament_membership = tournament_membership_service::query(con, props)
        .await
        .map_err(report_postgres_err)?;

    // return tournament_memberships
    let mut resp_tournament_memberships = vec![];
    for u in tournament_membership.into_iter() {
        resp_tournament_memberships.push(fill_tournament_membership(con, u).await?);
    }

    Ok(resp_tournament_memberships)
}

pub async fn tournament_submission_view(
    _config: Config,
    db: Db,
    _auth_service: AuthService,
    props: request::TournamentSubmissionViewProps,
) -> Result<Vec<response::TournamentSubmission>, response::AppError> {
    let con = &mut *db.lock().await;
    // get users
    let tournament_submission = tournament_submission_service::query(con, props)
        .await
        .map_err(report_postgres_err)?;

    // return tournament_submissions
    let mut resp_tournament_submissions = vec![];
    for u in tournament_submission.into_iter() {
        resp_tournament_submissions.push(fill_tournament_submission(con, u).await?);
    }

    Ok(resp_tournament_submissions)
}

pub async fn tournament_year_view(
    _config: Config,
    db: Db,
    _auth_service: AuthService,
    props: request::TournamentYearViewProps,
) -> Result<Vec<response::TournamentYear>, response::AppError> {
    let con = &mut *db.lock().await;
    // get users
    let tournament_year = tournament_year_service::query(con, props)
        .await
        .map_err(report_postgres_err)?;

    // return tournament_years
    let mut resp_tournament_years = vec![];
    for u in tournament_year.into_iter() {
        resp_tournament_years.push(fill_tournament_year(con, u).await?);
    }

    Ok(resp_tournament_years)
}
