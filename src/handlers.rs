use crate::models::{User, UserRequest, Response};
use crate::repository::UserRepository;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use actix_web::http::StatusCode;
use log::info;
use sqlx::PgPool;
use utoipa::OpenApi;
use uuid::Uuid;

#[derive(OpenApi)]
#[openapi(
    paths(
        create_user,
        get_all_user,
        get_user_by_id,
        delete_user_by_id,
        calculate_possible_combinations
    ),
    components(schemas(User)),
    tags(
        (name = "User Management", description = "Operations related to user management")
    )
)]
pub struct ApiDoc;

#[utoipa::path(
    post,
    path = "/users",
    request_body = UserRequest,
    responses(
        (status = 201, description = "User created successfully", body = User)
    )
)]
#[post("/users")]
async fn create_user(pool: web::Data<PgPool>, new_user: web::Json<UserRequest>) -> impl Responder {
    let user_request = new_user.into_inner();
    let id = user_request.id.unwrap_or_else(|| Uuid::new_v4().to_string()); // Use UUID v4 if no ID provided
    let user = User {
        id,
        email: user_request.email,
        name: user_request.name,
        role: user_request.role,
        password: user_request.password,
    };
    match UserRepository::create(&pool, user).await {
        Ok(user) => HttpResponse::Created().json(
            Response {
                status: "success".to_string(),
                status_code: StatusCode::CREATED.to_string(),
                data: Some(user),
            }),
        Err(_) => HttpResponse::InternalServerError().json(
            Response::<User> {
            status: "Internal Server Error".to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR.to_string(),
            data: None,
        }),
    }
}

#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, description = "List all users", body = [User])
    )
)]
#[get("/users")]
async fn get_all_user(pool: web::Data<PgPool>) -> impl Responder {
    match UserRepository::get_all_users(&pool).await {
        Ok(user) => HttpResponse::Ok().json(Response {
            status: "Success".to_string(),
            status_code: StatusCode::OK.to_string(),
            data: Some(user),
        }),
        Err(_) => HttpResponse::NotFound().json(Response::<User>{
            status: "Data not found".to_string(),
            status_code: StatusCode::NOT_FOUND.to_string(),
            data: None,
        }),
    }
}

#[utoipa::path(
    get,
    path = "/users/{id}",
    responses(
        (status = 200, description = "Get user by ID", body = User),
        (status = 404, description = "User not found")
    ),
    params(
        ("id" = String, Path, description = "User ID") // Use String here for UUID in URL
    )
)]
#[get("/users/{id}")]
async fn get_user_by_id(pool: web::Data<PgPool>, id: web::Path<String>) -> impl Responder {
    match UserRepository::get_user_by_id(&pool, id.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(Response{
            status: "Success".to_string(),
            status_code: StatusCode::OK.to_string(),
            data: Some(user),
        }),
        Err(_) => HttpResponse::NotFound().json(
            Response::<User> {
                status: "Data not found".to_string(),
                status_code: StatusCode::NOT_FOUND.to_string(),
                data: None,
            }),
    }
}

#[utoipa::path(
    delete,
    path = "/users/{id}",
    responses(
        (status = 204, description = "User deleted successfully"),
        (status = 404, description = "User not found")
    ),
    params(
        ("id" = String, Path, description = "User ID") // Use String here for UUID in URL
    )
)]
#[delete("/users/{id}")]
async fn delete_user_by_id(pool: web::Data<PgPool>, id: web::Path<String>) -> impl Responder {
    let id = id.into_inner();
    match UserRepository::delete_user_by_id(&pool, id).await {
        Ok(rows_affected) if rows_affected > 0 => HttpResponse::NoContent().json(
            Response::<User> {
                status: "Success".to_string(),
                status_code: StatusCode::OK.to_string(),
                data: None,
            }
        ),
        Ok(_) => HttpResponse::NotFound().json(
            Response::<User> {
                status: "Data not found".to_string(),
                status_code: StatusCode::NOT_FOUND.to_string(),
                data: None,
            }
        ),
        Err(_) => HttpResponse::InternalServerError().json(
            Response::<User> {
                status: "Internal Server Error".to_string(),
                status_code: StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                data: None,
            }
        ),
    }
}

#[utoipa::path(
    get,
    path = "/calculate-combinations/{word}",
    responses(
        (status = 200, description = "Number of possible combinations", body = String)
    ),
    params(
        ("word" = String, Path, description = "Word to calculate combinations for")
    )
)]
#[get("/calculate-combinations/{word}")]
pub async fn calculate_possible_combinations(word: web::Path<String>) -> impl Responder {
    let word = word.into_inner();
    let n = word.chars().count();
    let base: i32 = 2;
    let result = base.pow((n - 1) as u32);
    info!("Number of possible combinations: {}", result);
    HttpResponse::Ok().json(Response {
        status: "Success".to_string(),
        status_code: StatusCode::OK.to_string(),
        data: Some(result.to_string()),
    })
}