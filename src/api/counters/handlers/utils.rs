use std::str::FromStr;

use sqlx::postgres::PgRow;
use sqlx::{types::Uuid, Error, Pool, Postgres, Row};

use super::types::{Counter, MinesweeperResponse};
use crate::api::ErrorResp;

pub async fn increment_count(
    project: &str,
    entity: &str,
    pool: &Pool<Postgres>,
) -> Result<(), ErrorResp> {
    let count = get_count(project, entity, pool).await.map_err(|e| {
        println!("{:?}", e);
        ErrorResp::InternalError
    })?;
    let count_id = Uuid::from_str(&count.id).map_err(|e| {
        println!("{:?}", e);
        ErrorResp::InternalError
    })?;
    let query = "update counters set count = $1 where id = $2";
    sqlx::query(query)
        .bind(count.count + 1)
        .bind(count_id)
        .execute(pool)
        .await
        .map_err(|e| {
            println!("{:?}", e);
            ErrorResp::InternalError
        })?;
    Ok(())
}

pub async fn get_and_insert_count(
    project: &str,
    entity: &str,
    pool: &Pool<Postgres>,
    resp: &mut MinesweeperResponse,
) -> Result<(), ErrorResp> {
    let count = get_count(project, entity, pool).await.map_err(|e| {
        println!("{:?}", e);
        ErrorResp::InternalError
    })?;
    resp.insert(count.entity, count.count);
    Ok(())
}

pub async fn get_count(
    project: &str,
    entity: &str,
    pool: &Pool<Postgres>,
) -> Result<Counter, Error> {
    let count = get_or_insert_count(project, entity, pool).await;
    match count {
        Ok(row) => {
            let id: Uuid = row.get("id");
            let project: String = row.get("project");
            let entity: String = row.get("entity");
            let count: i32 = row.get("count");
            let counter = Counter {
                id: id.to_string(),
                project,
                entity,
                count,
            };
            Ok(counter)
        }
        Err(e) => Err(e),
    }
}

pub async fn get_or_insert_count(
    project: &str,
    entity: &str,
    pool: &Pool<Postgres>,
) -> Result<PgRow, Error> {
    match sqlx::query("select * from counters where project=$1 and entity=$2;")
        .bind(project)
        .bind(entity)
        .fetch_one(pool)
        .await
    {
        Ok(row) => Ok(row),
        Err(Error::RowNotFound) => {
            sqlx::query("insert into counters values ($1, $2, $3, 0) returning *;")
                .bind(uuid::Uuid::new_v4())
                .bind(project)
                .bind(entity)
                .fetch_one(pool)
                .await
        }
        Err(e) => Err(e),
    }
}
