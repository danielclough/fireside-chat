use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use common::database::{
    conversation::{
        ConversationForJson, ConversationForJsonVec, ConversationQuery, ConversationWithEngagements,
    },
    engagement::{EngagementForJson, EngagementForJsonVec, EngagementQuery},
};
use sqlx::{Pool, Sqlite};

use crate::{
    controllers::engagement::get::get_engagements_from_db,
    types::{
        conversation::{self, Conversation},
        errors::CustomError,
    },
};

pub async fn get_conversations(Extension(pool): Extension<Pool<Sqlite>>) -> impl IntoResponse {
    let sql = "SELECT * FROM conversations";

    let result = sqlx::query_as::<_, Conversation>(sql)
        .fetch_all(&pool)
        .await
        .unwrap();

    let mut final_result = vec![];

    for conversation in result.clone() {
        final_result.push(ConversationForJson {
            id: conversation.id,
            name: conversation.name,
            user_id: conversation.user_id,
            model_params: conversation.model_params,
            inference_params: conversation.inference_params,
        });
        println!(
            "\nid: {}\n user_id: {}\n",
            &conversation.id, &conversation.user_id
        );
    }

    (StatusCode::OK, Json(final_result))
}

pub async fn get_conversation_by_id(
    Extension(pool): Extension<Pool<Sqlite>>,
    Path(id): Path<i64>,
) -> Result<Json<ConversationForJson>, CustomError> {
    let conversation = ConversationQuery {
        id: Some(id),
        name: None,
        user_id: None,
        model_params: None,
        inference_params: None,
    };
    let result = get_conversation_from_db(pool, conversation).await?;
    let final_result = ConversationForJson {
        id: result.id,
        name: result.name,
        user_id: result.user_id,
        model_params: result.model_params,
        inference_params: result.inference_params,
    };

    Ok(Json(final_result))
}

pub async fn get_conversations_by_user_id(
    Extension(pool): Extension<Pool<Sqlite>>,
    Path(user_id): Path<i64>,
) -> Result<Json<Vec<ConversationWithEngagements>>, CustomError> {
    let mut conversations_with_engagements: Vec<ConversationWithEngagements> = vec![];
    let conversation: ConversationQuery = ConversationQuery {
        id: None,
        name: None,
        user_id: Some(user_id),
        model_params: None,
        inference_params: None,
    };

    // get conversations
    let result = get_conversations_from_db(pool.clone(), conversation).await?;

    // get engagements for each conversation
    for r in result.list {
        let engagement_query = EngagementQuery {
            id: None,
            conversation_id: Some(r.id),
            keyword: None,
        };
        let engagements = get_engagements_from_db(pool.clone(), engagement_query).await?;
        let c_w_e = ConversationWithEngagements {
            id: r.id,
            name: r.name,
            engagements: EngagementForJsonVec {
                list: engagements
                    .iter()
                    .map(|e| EngagementForJson {
                        id: e.id,
                        conversation_id: e.conversation_id,
                        query: e.query.clone(),
                        response: e.response.clone(),
                    })
                    .collect(),
            },
            user_id: r.user_id,
            model_params: r.model_params,
            inference_params: r.inference_params,
        };
        conversations_with_engagements.push(c_w_e);
    }

    Ok(Json(conversations_with_engagements))
}

pub async fn get_conversation_from_db(
    pool: Pool<Sqlite>,
    conversation: ConversationQuery,
) -> Result<conversation::Conversation, CustomError> {
    let result = if conversation.id.is_some() {
        let sql = "SELECT * FROM conversations WHERE id=$1";
        sqlx::query_as::<_, Conversation>(sql)
            .bind(conversation.id)
            .fetch_one(&pool)
            .await
            .map_err(|_| CustomError::NotFound)?
    } else {
        let sql = "SELECT * FROM conversations WHERE user_id=$1";
        sqlx::query_as::<_, Conversation>(sql)
            .bind(conversation.user_id)
            .fetch_one(&pool)
            .await
            .map_err(|_| CustomError::NotFound)?
    };

    Ok(result)
}

pub async fn get_conversations_from_db(
    pool: Pool<Sqlite>,
    conversation: ConversationQuery,
) -> Result<ConversationForJsonVec, CustomError> {
    let sql = if conversation.id.is_some() {
        format!(
            "SELECT * FROM conversations WHERE id={}",
            conversation.id.unwrap()
        )
    } else {
        format!(
            "SELECT * FROM conversations WHERE user_id={}",
            conversation.user_id.unwrap(),
        )
    };

    let result = sqlx::query_as::<_, Conversation>(&sql)
        .fetch_all(&pool)
        .await
        .unwrap();

    let mut final_result: ConversationForJsonVec = ConversationForJsonVec { list: vec![] };

    for conversation in result.clone() {
        final_result.list.push(ConversationForJson {
            id: conversation.id,
            name: conversation.name,
            user_id: conversation.user_id,
            model_params: conversation.model_params,
            inference_params: conversation.inference_params,
        });
        println!(
            "\nid: {}\n user_id: {}\n",
            &conversation.id, &conversation.user_id
        );
    }

    Ok(final_result)
}
