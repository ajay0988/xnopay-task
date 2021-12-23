use actix_web::{HttpResponse, web, Error, http};

use super::{Employee, service};

pub async fn save(
    employee: web::Json<Employee>
) -> Result<Result<HttpResponse, HttpResponse>, Error> {
    let employee: Employee = employee.into_inner();
    let res = web::block(move || service::db_create_Employee(employee))
        .await
        .map(|_result| HttpResponse::Ok().json(_result))
        .map_err(|_| HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR));
    Ok(res)
}

pub async fn get(
    id: web::Path<String>
) -> Result<Result<HttpResponse, HttpResponse>, Error> {
    let res = web::block(move || service::db_read_Employee(&id))
        .await
        .map(|_result| HttpResponse::Ok().json(_result))
        .map_err(|_| HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR));
    Ok(res)
}


pub async fn get_all() -> Result<Result<HttpResponse, HttpResponse>, Error> {
    let res = web::block(move || service::db_read_all_Employees())
        .await
        .map(|_result| HttpResponse::Ok().json(_result))
        .map_err(|_| HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR));
    Ok(res)
}

pub async fn update(
    id: web::Path<String>,
    employee: web::Json<Employee>
) -> Result<Result<HttpResponse, HttpResponse>, Error> {
    let employee = employee.into_inner();
    let res = web::block(move || service::db_update_Employee(&id, employee))
        .await
        .map(|_result| HttpResponse::Ok().json(_result))
        .map_err(|_| HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR));
    Ok(res)
}

pub async fn delete(
    id: web::Path<String>
) -> Result<Result<HttpResponse, HttpResponse>, Error> {
    let res = web::block(move || service::db_delete_Employee(&id))
        .await
        .map(|_result| HttpResponse::Ok().json(_result))
        .map_err(|_| HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR));
    Ok(res)
}