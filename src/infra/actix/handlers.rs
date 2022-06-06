use actix_web::{ get, post, web, web::Json, HttpResponse, Responder};
use serde::Serialize;
use crate::application::get_score_list_service::GetScoreListService;
use crate::application::get_stage_list_service::GetStageListService;
use crate::application::insert_stage_service::InsertStageService;
use crate::application::insert_score_service::InsertScoreService;
use crate::application::score_data::ScoreData;
use crate::infra::actix::request::{StageRequest, ScoreRequest};
use super::router::RequestContext;

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
    r#type: String,
}

#[get("/stage")]
async fn get_stage_list(
    data: web::Data<RequestContext>,
) -> impl Responder {
    let stage_application = GetStageListService::new(data.stage_repository());
    match stage_application.handle() {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => {
            let response = ErrorResponse {
                message: format!("FAILURE Get Stage List."),
                r#type: "get_stage_list_error".to_string(),
            };
            HttpResponse::InternalServerError().json(response)
        }
    }
}

#[post("/stage")]
async fn insert_new_stage(
  data: web::Data<RequestContext>,
  request: Json<StageRequest>,
) -> impl Responder {
  let stage_application = InsertStageService::new(data.stage_repository());
  let data = request.of();
  match stage_application.handle(data.clone()){
    Ok(_) => HttpResponse::Ok().body("success insert new stage."),
    Err(_) => {
      let response = ErrorResponse{
        message: format!("FAILURE insert new Stage."),
        r#type: "insert_stage_error".to_string(),
      };
      HttpResponse::InternalServerError().json(response)
    }
  }
}

#[get("/score/{stage_id}")]
async fn get_score_list(
  data: web::Data<RequestContext>,
  path_params: web::Path<(i32,)>,
) -> impl Responder {
  let score_application = GetScoreListService::new(data.score_repository());
  let stage_id = path_params.into_inner().0.into();
  match score_application.handle(stage_id) {
    Ok(score) => HttpResponse::Ok().json(score),
    Err(_) => {
      let response = ErrorResponse{
        message: format!("FAILURE Get score on stage {:?}", stage_id),
        r#type: "get_score_list_error".to_string(),
      };
      HttpResponse::InternalServerError().json(response)
    }
  }
}

#[post("/score")]
async fn insert_new_score(
  data: web::Data<RequestContext>,
  request: Json<ScoreRequest>,
) -> impl Responder {
  let score_application = InsertScoreService::new(data.score_repository());
  let data = ScoreData::new(&request.of());
  match score_application.handle(data){
    Ok(_) => HttpResponse::Ok().body("success insert new score."),
    Err(_) => {
      let response = ErrorResponse{
        message: format!("FAILURE insert new Score."),
        r#type: "insert_score_error".to_string(),
      };
      HttpResponse::InternalServerError().json(response)
    }
  }
}


#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}