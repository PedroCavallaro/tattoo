use std::sync::Arc;

use axum::{
    extract::{Path, State}, http::StatusCode, Extension, Json
};

use crate::{
    app::place::infra::repositories::place_repository::PlaceRepository,
    domain::{entities::{place::Place, user::JwtPayload}, error::ApiError},
};

pub async fn execute(
    Extension(user): Extension<JwtPayload>,
    Path(id): Path<i64>,
    State(place_repository): State<Arc<PlaceRepository>>,
)-> Result<Json<Place>, ApiError> {

    let place = place_repository.get_place_by_id(id)?;

    match place {
        Some(_place) => Ok(Json(_place)),
        None => Err(ApiError::new(StatusCode::NOT_FOUND.as_u16(), "Estabelecimento não encontrado"))
    }
}
