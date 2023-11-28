use core::str::FromStr;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;

use axum::{http::StatusCode, Json, Router, routing::post};
use axum::extract::State;
use axum::http::header;
use axum::response::IntoResponse;

use crate::core::encoder::encoder::IMAGE_ENCODER;
use crate::core::errors::Error;
use crate::core::http::avatar_data_factory::create_avatar_data;
use crate::core::http::template_data::PetpetData;
use crate::server::config::ServerConfig;
use crate::server::service::petpet_service::PetpetService;

pub struct PetpetServer {
    addr: SocketAddr,
    pub(crate) service: PetpetService,
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
            // .route("/", get(root))
            .route("/generate", post(
                generate
            ))
            .with_state(Arc::new(self));

        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await.unwrap()
    }
}

async fn generate(
    State(logic): State<Arc<PetpetServer>>,
    Json(payload): Json<PetpetData>,
) -> impl IntoResponse {
    let avatar_data = create_avatar_data(&payload.avatar).unwrap();
    let builder = logic.service.get_builder(&payload.key).unwrap();
    let start_time0 = Instant::now();
    let (images, delay) = builder.build(avatar_data).await.unwrap();
    println!("download & draw: {:?}", start_time0.elapsed());
    let start_time1 = Instant::now();
    let blob = IMAGE_ENCODER.encode(&images, delay).unwrap();
    println!("encode: {:?}", start_time1.elapsed());
    (StatusCode::OK, [(header::CONTENT_TYPE, "image/png")], blob)
}
