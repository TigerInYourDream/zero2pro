use actix_web::{web, HttpResponse, Responder};
use anyhow::Result;
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, DatabaseConnection, Set};
use serde::Deserialize;

use crate::entities;

#[derive(Debug, Deserialize)]
pub struct FromData {
    email: String,
    name: String,
}

#[tracing::instrument(skip(req, pool))]
pub async fn subscribe(
    req: web::Form<FromData>,
    pool: web::Data<DatabaseConnection>,
) -> impl Responder {
    tracing::info!("Saving new subscription: {:?}", req);
    let db = pool.as_ref();
    match insert_sub(db, &req).await {
        Ok(model) => {
            tracing::info!("Saved new subscription: {:?}", model)
        }
        Err(e) => {
            tracing::error!("Failed to save new subscription: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    }

    HttpResponse::Ok().finish()
}

#[tracing::instrument(skip(pool, data))]
pub async fn insert_sub(
    pool: &DatabaseConnection,
    data: &FromData,
) -> Result<entities::subscriptions::ActiveModel> {
    let news_sub = entities::subscriptions::ActiveModel {
        id: NotSet,
        email: Set(data.email.clone()),
        name: Set(data.name.clone()),
        ..Default::default()
    };

    news_sub
        .save(pool)
        .await
        .map_err(|e| anyhow::anyhow!(e.to_string()))
}