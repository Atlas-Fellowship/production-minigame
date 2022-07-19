use super::db_types::*;
use tokio_postgres::GenericClient;

impl From<tokio_postgres::row::Row> for Tournament {
    // select * from tournament order only, otherwise it will fail
    fn from(row: tokio_postgres::Row) -> Tournament {
        Tournament {
            tournament_id: row.get("tournament_id"),
            creation_time: row.get("creation_time"),
            creator_user_id: row.get("creator_user_id"),
            max_years: row.get("max_years"),
        }
    }
}

pub async fn add(
    con: &mut impl GenericClient,
    creator_user_id: i64,
    max_years: i64,
) -> Result<Tournament, tokio_postgres::Error> {
    let row = con
        .query_one(
            "INSERT INTO
       tournament(
           creator_user_id,
           max_years
       )
       VALUES($1, $2)
       RETURNING tournament_id, creation_time
      ",
            &[&creator_user_id, &max_years],
        )
        .await?;

    // return tournament
    Ok(Tournament {
        tournament_id: row.get(0),
        creation_time: row.get(1),
        creator_user_id,
        max_years,
    })
}

pub async fn get_by_tournament_id(
    con: &mut impl GenericClient,
    tournament_id: i64,
) -> Result<Option<Tournament>, tokio_postgres::Error> {
    let result = con
        .query_opt(
            "SELECT * FROM tournament WHERE tournament_id=$1",
            &[&tournament_id],
        )
        .await?
        .map(|x| x.into());
    Ok(result)
}
