use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use common::database::engagement::{EngagementForJson, EngagementQuery};
use sqlx::{Pool, Sqlite};

use crate::types::{
    engagement::{self, Engagement},
    errors::CustomError,
};

pub async fn get_engagements(Extension(pool): Extension<Pool<Sqlite>>) -> impl IntoResponse {
    let sql = "SELECT * FROM engagements";

    let result = sqlx::query_as::<_, Engagement>(sql)
        .fetch_all(&pool)
        .await
        .unwrap();

    let mut final_result = vec![];

    for engagement in result.clone() {
        final_result.push(EngagementForJson {
            id: engagement.id,
            conversation_id: engagement.conversation_id,
            query: engagement.query,
            response: engagement.response,
        });
        println!(
            "\nid: {}\n conversation_id: {}\n",
            &engagement.id, &engagement.conversation_id
        );
    }

    (StatusCode::OK, Json(final_result))
}

pub async fn get_engagement_from_db(
    pool: Pool<Sqlite>,
    engagement: EngagementQuery,
) -> Result<engagement::Engagement, CustomError> {
    let result = if engagement.id.is_some() {
        let sql = "SELECT * FROM engagements WHERE id=$1";
        sqlx::query_as::<_, Engagement>(sql)
            .bind(engagement.id)
            .fetch_one(&pool)
            .await
            .map_err(|_| CustomError::NotFound)?
    } else {
        let sql = "SELECT * FROM engagements WHERE conversation_id=$1";
        sqlx::query_as::<_, Engagement>(sql)
            .bind(engagement.conversation_id)
            .fetch_one(&pool)
            .await
            .map_err(|_| CustomError::NotFound)?
    };

    Ok(result)
}

pub async fn get_engagements_from_db(
    pool: Pool<Sqlite>,
    engagement: EngagementQuery,
) -> Result<Vec<engagement::Engagement>, CustomError> {
    let sql = "SELECT * FROM engagements WHERE conversation_id=$1";
    let result = sqlx::query_as::<_, Engagement>(sql)
        .bind(engagement.conversation_id)
        .fetch_all(&pool)
        .await
        .map_err(|_| CustomError::NotFound)?;

    Ok(result)
}

pub async fn get_engagement_by_id(
    Extension(pool): Extension<Pool<Sqlite>>,
    Path(id): Path<i64>,
) -> Result<Json<EngagementForJson>, CustomError> {
    let engagement = EngagementQuery {
        id: Some(id),
        conversation_id: None,
        keyword: None,
    };
    let result = get_engagement_from_db(pool, engagement).await?;
    let final_result = EngagementForJson {
        id: result.id,
        conversation_id: result.conversation_id,
        query: result.query,
        response: result.response,
    };

    Ok(Json(final_result))
}

pub async fn get_engagements_by_conversation_id(
    Extension(pool): Extension<Pool<Sqlite>>,
    Path(conversation_id): Path<i64>,
) -> Result<Json<Vec<EngagementForJson>>, CustomError> {
    let engagement = EngagementQuery {
        id: None,
        conversation_id: Some(conversation_id),
        keyword: None,
    };
    let result = get_engagements_from_db(pool, engagement).await?;

    let mut final_result = vec![];

    for r in result {
        final_result.push(EngagementForJson {
            id: r.id,
            conversation_id: r.conversation_id,
            query: r.query,
            response: r.response,
        });
    }

    Ok(Json(final_result))
}
