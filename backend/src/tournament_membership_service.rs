use super::db_types::*;
use std::convert::From;
use tokio_postgres::GenericClient;

impl From<tokio_postgres::row::Row> for TournamentMembership {
    // select * from tournament_membership order only, otherwise it will fail
    fn from(row: tokio_postgres::Row) -> TournamentMembership {
        TournamentMembership {
            tournament_membership_id: row.get("tournament_membership_id"),
            creation_time: row.get("creation_time"),
            creator_user_id: row.get("creator_user_id"),
            tournament_id: row.get("tournament_id"),
            active: row.get("active"),
        }
    }
}

// TODO we need to figure out a way to make scheduled and unscheduled articles work better
pub async fn add(
    con: &mut impl GenericClient,
    creator_user_id: i64,
    tournament_id: i64,
    active: bool,
) -> Result<TournamentMembership, tokio_postgres::Error> {
    let row = con
        .query_one(
            "INSERT INTO
             tournament_membership(
                 creator_user_id,
                 tournament_id,
                 active
             )
             VALUES ($1, $2, $3)
             RETURNING tournament_membership_id, creation_time
            ",
            &[
                &creator_user_id,
                &tournament_id,
                &active
            ],
        )
        .await?;

    Ok(TournamentMembership {
        tournament_membership_id: row.get(0),
        creation_time: row.get(1),
        creator_user_id,
        tournament_id,
        active,
    })
}

pub async fn get_recent_by_tournament(
    con: &mut impl GenericClient,
    tournament_id: i64,
) -> Result<Vec<TournamentMembership>, tokio_postgres::Error> {
    let sql = [
        "SELECT ts.* FROM recent_tournament_membership ts",
        " WHERE 1 = 1",
        " AND ts.tournament_id = $1",
        " ORDER BY ts.tournament_membership_id",
    ]
    .join("\n");

    let stmnt = con.prepare(&sql).await?;

    let results = con
        .query(&stmnt, &[&tournament_id])
        .await?
        .into_iter()
        .map(|row| row.into())
        .collect();

    Ok(results)
}

pub async fn get_recent_by_tournament_user(
    con: &mut impl GenericClient,
    tournament_id: i64,
    user_id: i64,
) -> Result<Option<TournamentMembership>, tokio_postgres::Error> {
    let sql = [
        "SELECT ts.* FROM recent_tournament_membership ts",
        " WHERE 1 = 1",
        " AND ts.tournament_id = $1",
        " AND ts.creator_user_id = $2",
        " ORDER BY ts.tournament_membership_id",
    ]
    .join("\n");

    let stmnt = con.prepare(&sql).await?;

    let results = con
        .query_opt(&stmnt, &[&tournament_id, &user_id])
        .await?
        .map(|row| row.into());

    Ok(results)
}


pub async fn query(
    con: &mut impl GenericClient,
    props: super::request::TournamentMembershipViewProps,
) -> Result<Vec<TournamentMembership>, tokio_postgres::Error> {
    let sql = [
        if props.only_recent {
            "SELECT ts.* FROM recent_tournament_membership ts"
        } else {
            "SELECT ts.* FROM tournament_membership ts"
        },
        " WHERE 1 = 1",
        " AND ($1::bigint[] IS NULL OR ts.tournament_membership_id = ANY($1))",
        " AND ($2::bigint   IS NULL OR ts.creation_time >= $2)",
        " AND ($3::bigint   IS NULL OR ts.creation_time <= $3)",
        " AND ($4::bigint[] IS NULL OR ts.creator_user_id = ANY($4))",
        " AND ($5::bigint[] IS NULL OR ts.tournament_id = ANY($5))",
        " AND ($6::bool     IS NULL OR ts.active = $6)",
        " ORDER BY ts.tournament_membership_id",
    ]
    .join("\n");

    let stmnt = con.prepare(&sql).await?;

    let results = con
        .query(
            &stmnt,
            &[
                &props.tournament_membership_id,
                &props.min_creation_time,
                &props.max_creation_time,
                &props.creator_user_id,
                &props.tournament_id,
                &props.active,
            ],
        )
        .await?
        .into_iter()
        .map(|row| row.into())
        .collect();

    Ok(results)
}
