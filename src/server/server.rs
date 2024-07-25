use core::str::FromStr;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;

use axum::{http::StatusCode, Json, Router, routing::get, routing::post};
use axum::extract::{Query, State};
use axum::http::header;
use axum::response::IntoResponse;
use log::info;
use serde::Serialize;
use crate::core::encoder::encoder::IMAGE_ENCODER;
use crate::core::errors::Error;
use crate::core::http::avatar_data_factory::create_avatar_data;
use crate::server::config::ServerConfig;
use crate::server::query_template::QueryParams;
use crate::server::service::petpet_service::PetpetService;
use crate::server::service::service_data::PetpetServiceData;

pub struct PetpetServer {
    addr: SocketAddr,
    pub(crate) service: PetpetService,
}

#[derive(Serialize)]
struct ServerInfo {
    version: String,
    templates: Vec<TemplateInfo>,
}

#[derive(Serialize)]
struct TemplateInfo {
    id: String,
    avatar: TemplateItemInfo,
    text: TemplateItemInfo,
    alias: Vec<String>,
}

#[derive(Serialize)]
struct TemplateItemInfo {
    types: Vec<String>,
    length: i32,
}


impl PetpetServer {
    pub fn new(config: ServerConfig) -> Result<Self, Error> {
        Ok(PetpetServer {
            addr: SocketAddr::from_str(&config.address).unwrap(),
            service: PetpetService::with_paths(&config.data_path)?,
        })
    }

    pub async fn run(self) {
        tracing_subscriber::fmt::init();

        let addr = self.addr.clone();

        let app = Router::new()
            .route("/", get(get_info))
            .route("/generate", post(generate_post))
            .route("/generate", get(generate_get))
            .with_state(Arc::new(self));

        info!("server run in {}", &addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await.unwrap();
    }
}

async fn get_info(
    State(server): State<Arc<PetpetServer>>,
) -> (StatusCode, Json<ServerInfo>) {
    let info = ServerInfo {
        //TODO
        version: "0.0.1-dev".to_string(),
        templates: server.service.builder_map.iter()
            .map(|(k, v)| {
                let mut avatar_types: Vec<String> = v.template.avatar.iter()
                    .map(|avatar| format!("{:?}", avatar._type))
                    .collect();
                avatar_types.sort();
                avatar_types.dedup();
                return TemplateInfo {
                    id: k.clone(),
                    avatar: TemplateItemInfo {
                        types: avatar_types,
                        length: 0, //TODO
                    },
                    text: TemplateItemInfo { //TODO
                        types: vec![],
                        length: 0,
                    },
                    alias: v.template.alias.clone(),
                };
            }).collect(),
    };
    (StatusCode::OK, Json(info))
}

async fn generate_post(
    State(server): State<Arc<PetpetServer>>,
    Json(payload): Json<PetpetServiceData>,
) -> impl IntoResponse {
    let avatar_data = create_avatar_data(&payload.avatar).unwrap();
    let builder = server.service.get_builder(&payload.key).unwrap();
    let start_time0 = Instant::now();
    let (images, delay) = builder.build(avatar_data, payload.text).await.unwrap();
    let start_time1 = Instant::now();
    let (blob, format) = IMAGE_ENCODER.encode(&images, delay).unwrap();
    info!("template: {}; download & draw: {:?}; encode: {:?}", &payload.key, start_time0.elapsed(), start_time1.elapsed());
    (StatusCode::OK, [(header::CONTENT_TYPE, format.to_format())], blob)
}

async fn generate_get(
    State(server): State<Arc<PetpetServer>>,
    Query(payload): Query<QueryParams>,
) -> impl IntoResponse {
    let data = payload.to_data();
    let avatar_data = create_avatar_data(&data.avatar).unwrap();
    let builder = server.service.get_builder(&data.key).unwrap();
    let start_time0 = Instant::now();
    let (images, delay) = builder.build(avatar_data, data.text).await.unwrap();
    let start_time1 = Instant::now();
    let (blob, format) = IMAGE_ENCODER.encode(&images, delay).unwrap();
    info!("template: {}; download & draw: {:?}; encode: {:?}", &data.key, start_time0.elapsed(), start_time1.elapsed());
    (StatusCode::OK, [(header::CONTENT_TYPE, format.to_format())], blob)
}
