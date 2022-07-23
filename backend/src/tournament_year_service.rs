use super::db_types::*;
use super::request;
use tokio_postgres::GenericClient;

impl From<tokio_postgres::row::Row> for TournamentYear {
    // select * from tournament_year order only, otherwise it will fail
    fn from(row: tokio_postgres::Row) -> TournamentYear {
        TournamentYear {
            tournament_year_id: row.get("tournament_year_id"),
            creation_time: row.get("creation_time"),
            creator_user_id: row.get("creator_user_id"),
            tournament_id: row.get("tournament_id"),
            current_year: row.get("current_year"),
        }
    }
}

pub async fn add(
    con: &mut impl GenericClient,
    creator_user_id: i64,
    tournament_id: i64,
    current_year: i64,
) -> Result<TournamentYear, tokio_postgres::Error> {
    let row = con
        .query_one(
            "INSERT INTO
             tournament_year(
                 creator_user_id,
                 tournament_id,
                 current_year
             )
             VALUES ($1, $2, $3)
             RETURNING tournament_year_id, creation_time
            ",
            &[
                &creator_user_id,
                &tournament_id,
                &current_year,
            ],
        )
        .await?;

    // return tournament_year
    Ok(TournamentYear {
        tournament_year_id: row.get(0),
        creation_time: row.get(1),
        creator_user_id,
        tournament_id,
        current_year,
    })
}

pub async fn get_recent_by_tournament_id(
    con: &mut impl GenericClient,
    tournament_id: i64,
) -> Result<Option<TournamentYear>, tokio_postgres::Error> {
    let sql = [
        "SELECT td.* FROM recent_tournament_year td",
        "WHERE 1 = 1",
        "AND td.tournament_id = $1",
        "ORDER BY td.tournament_year_id",
    ]
    .join("\n");

    let stmnt = con.prepare(&sql).await?;

    let results = con
        .query_opt(&stmnt, &[&tournament_id])
        .await?
        .map(|row| row.into());

    Ok(results)
}

pub async fn query(
    con: &mut impl GenericClient,
    props: request::TournamentYearViewProps,
) -> Result<Vec<TournamentYear>, tokio_postgres::Error> {
    let sql = [
        if props.only_recent {
            "SELECT td.* FROM recent_tournament_year td"
        } else {
            "SELECT td.* FROM tournament_year td"
        },
        "WHERE 1 = 1",
        "AND ($1::bigint[]  IS NULL OR td.tournament_year_id = ANY($1))",
        "AND ($2::bigint    IS NULL OR td.creation_time >= $2)",
        "AND ($3::bigint    IS NULL OR td.creation_time <= $3)",
        "AND ($4::bigint[]  IS NULL OR td.creator_user_id = ANY($4))",
        "AND ($5::bigint[]  IS NULL OR td.tournament_id = ANY($5))",
        "ORDER BY td.tournament_year_id",
    ]
    .join("\n");

    let stmnt = con.prepare(&sql).await?;

    let results = con
        .query(
            &stmnt,
            &[
                &props.tournament_year_id,
                &props.min_creation_time,
                &props.max_creation_time,
                &props.creator_user_id,
                &props.tournament_id,
            ],
        )
        .await?
        .into_iter()
        .map(|row| row.into())
        .collect();

    Ok(results)
}
