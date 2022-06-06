use super::handlers;
use crate::{
    config::CONFIG,
    domain::models::{
        score::score_repository::ScoreRepository, stage::stage_repository::StageRepository,
    },
};
use actix_web::{middleware::Logger, App, HttpServer};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let port = std::env::var("PORT")
        .ok()
        .and_then(|val| val.parse::<u16>().ok())
        .unwrap_or(CONFIG.server_port);

    HttpServer::new(|| {
        App::new()
            .data(RequestContext::new())
            .wrap(Logger::default())
            .service(handlers::health)
            .service(handlers::get_stage_list)
            .service(handlers::insert_new_stage)
            .service(handlers::get_score_list)
            .service(handlers::insert_new_score)
    })
    .bind(format!("{}:{}", CONFIG.server_address, port))?
    .run()
    .await
}

#[derive(Clone)]
pub struct RequestContext {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl RequestContext {
    pub fn new() -> RequestContext {
        let manager = ConnectionManager::<PgConnection>::new(&CONFIG.database_url);
        let pool = Pool::builder()
            .max_size(4)
            .build(manager)
            .expect("Failed to create DB connection pool.");

        RequestContext { pool }
    }

    pub fn stage_repository(&self) -> impl StageRepository {
        use crate::infra::diesel::stage_repository::StageRepositoryImpl;

        StageRepositoryImpl {
            pool: Box::new(self.pool.to_owned()),
        }
    }

    pub fn score_repository(&self) -> impl ScoreRepository {
        use crate::infra::diesel::score_repository::ScoreRepositoryImpl;

        ScoreRepositoryImpl {
            pool: Box::new(self.pool.to_owned()),
        }
    }
}
