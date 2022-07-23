use super::db_types::*;
use super::request;
use tokio_postgres::GenericClient;

impl From<tokio_postgres::row::Row> for TournamentYearDemand {
    // select * from tournament_year_demand order only, otherwise it will fail
    fn from(row: tokio_postgres::Row) -> TournamentYearDemand {
        TournamentYearDemand {
            tournament_year_demand_id: row.get("tournament_year_demand_id"),
            creation_time: row.get("creation_time"),
            tournament_id: row.get("tournament_id"),
            user_id: row.get("user_id"),
            year: row.get("year"),
            demand: row.get("demand"),
        }
    }
}

pub async fn add(
    con: &mut impl GenericClient,
    user_id: i64,
    tournament_id: i64,
    year: i64,
    demand: i64,
) -> Result<TournamentYearDemand, tokio_postgres::Error> {
    let row = con
        .query_one(
            "INSERT INTO
             tournament_year_demand(
                 user_id,
                 tournament_id,
                 year,
                 demand
             )
             VALUES ($1, $2, $3, $4)
             RETURNING tournament_year_demand_id, creation_time
            ",
            &[&user_id, &tournament_id, &year, &demand],
        )
        .await?;

    // return tournament_year_demand
    Ok(TournamentYearDemand {
        tournament_year_demand_id: row.get(0),
        creation_time: row.get(1),
        user_id,
        tournament_id,
        year,
        demand,
    })
}

pub async fn get_recent_by_tournament_id(
    con: &mut impl GenericClient,
    tournament_id: i64,
) -> Result<Vec<TournamentYearDemand>, tokio_postgres::Error> {
    let sql = [
        "SELECT td.* FROM recent_tournament_year_demand td",
        " WHERE 1 = 1",
        " AND td.tournament_id = $1",
        " ORDER BY td.tournament_year_demand_id",
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

pub async fn query(
    con: &mut impl GenericClient,
    props: request::TournamentYearDemandViewProps,
) -> Result<Vec<TournamentYearDemand>, tokio_postgres::Error> {
    let sql = [
        "SELECT td.* FROM tournament_year_demand td",
        " WHERE 1 = 1",
        " AND ($1::bigint[]  IS NULL OR td.tournament_year_demand_id = ANY($1))",
        " AND ($2::bigint    IS NULL OR td.creation_time >= $2)",
        " AND ($3::bigint    IS NULL OR td.creation_time <= $3)",
        " AND ($4::bigint[]  IS NULL OR td.user_id = ANY($4))",
        " AND ($5::bigint[]  IS NULL OR td.tournament_id = ANY($5))",
        " ORDER BY td.tournament_year_demand_id",
    ]
    .join("\n");

    let stmnt = con.prepare(&sql).await?;

    let results = con
        .query(
            &stmnt,
            &[
                &props.tournament_year_demand_id,
                &props.min_creation_time,
                &props.max_creation_time,
                &props.user_id,
                &props.tournament_id,
            ],
        )
        .await?
        .into_iter()
        .map(|row| row.into())
        .collect();

    Ok(results)
}
