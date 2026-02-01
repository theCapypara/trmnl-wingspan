mod auth;
mod bird;
mod config;

use crate::bird::{Bird, Locale, load_birds};
use crate::config::Config;
use auth::CheckToken;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, HeaderValue, header};
use axum::response::{IntoResponse, Redirect};
use axum::{Json, Router, routing::get};
use axum_extra::extract::Query;
use chrono::{Timelike, Utc};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::validate_request::ValidateRequestHeaderLayer;

#[derive(Debug)]
struct AppState {
    config: Config,
    birds_db: HashMap<String, Bird>,
}

impl AppState {
    fn new() -> Self {
        let config =
            toml::from_str(&read_to_string("./config.toml").expect("failed to read config"))
                .expect("failed to parse config");
        Self {
            birds_db: load_birds(&config),
            config,
        }
    }
}

#[derive(Deserialize)]
struct GetCurrentParams {
    locale: Option<String>,
    #[serde(default)]
    allowed_set: Vec<String>,
}

async fn get_current(
    State(state): State<Arc<AppState>>,
    query: Query<GetCurrentParams>,
) -> impl IntoResponse {
    let now = Utc::now().num_seconds_from_midnight();
    let time_idx = now / state.config.new_bird_interval;
    let remaining_time = state.config.new_bird_interval - (now % state.config.new_bird_interval);
    let locale: Option<Locale> = query
        .locale
        .as_ref()
        .and_then(|locale| locale.try_into().ok());

    let mut rng = ChaCha8Rng::seed_from_u64(time_idx as u64);

    let birds = state.birds_db.values().filter(|bird| {
        query.allowed_set.is_empty() || query.allowed_set.contains(&bird.master.set)
    });
    let Some(chosen) = birds.choose(&mut rng) else {
        return (HeaderMap::new(), Json(None));
    };

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CACHE_CONTROL,
        HeaderValue::from_str(&format!("public, max-age={remaining_time}")).unwrap(),
    );

    (headers, Json(Some(chosen.produce(locale))))
}

async fn redirect_default_images(
    State(state): State<Arc<AppState>>,
    Path(image_path): Path<String>,
) -> Redirect {
    Redirect::permanent(&format!(
        "/images/{}/{}",
        state.config.default_images, image_path
    ))
}

fn cache_layer_long() -> SetResponseHeaderLayer<HeaderValue> {
    SetResponseHeaderLayer::overriding(
        header::CACHE_CONTROL,
        HeaderValue::from_static("public, max-age=604800, immutable"),
    )
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState::new());

    let mut image_routers = Router::new();
    for (set_name, set_spec) in &state.config.images {
        if set_name == "_" {
            panic!("_ not allowed.");
        }
        if let Some(token) = &set_spec.token {
            image_routers = image_routers.nest_service(
                &format!("/images/{set_name}"),
                ServiceBuilder::new()
                    .layer(cache_layer_long())
                    .layer(ValidateRequestHeaderLayer::custom(CheckToken::new(token)))
                    .service(ServeDir::new(&set_spec.path)),
            );
        } else {
            image_routers = image_routers.nest_service(
                &format!("/images/{set_name}"),
                ServiceBuilder::new()
                    .layer(cache_layer_long())
                    .service(ServeDir::new(&set_spec.path)),
            );
        }
    }

    let app = Router::new()
        .route("/api/current", get(get_current))
        .route("/images/_/{key}", get(redirect_default_images))
        .merge(image_routers)
        .nest_service(
            "/icons",
            ServiceBuilder::new()
                .layer(cache_layer_long())
                .service(ServeDir::new(
                    state.config.wingsearch.join("src/assets/icons/png"),
                )),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
