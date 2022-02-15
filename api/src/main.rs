mod graph;

#[macro_use]
extern crate juniper;

use crate::graph::*;
use actix_web::http::header::HeaderMap;
use actix_web::web::Data;
use app::domain::user::AuthUser;
use juniper_actix::{graphql_handler, playground_handler};
use lambda_web::actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use lambda_web::{is_running_on_lambda, run_actix_on_lambda, LambdaError};
use std::env;
use std::str::FromStr;

#[actix_web::main]
async fn main() -> Result<(), LambdaError> {
    app::aws::ssm::load_env().await;

    let app = move || {
        let schema = create_schema();

        // ApiGatewayにドメインを設定しないのでステージ名がパスに入る
        App::new()
            .app_data(Data::new(schema))
            .service(
                web::resource("/default/graphql")
                    .route(web::post().to(graphql_route))
                    .route(web::get().to(graphql_route)),
            )
            .service(web::resource("/default/playground").route(web::get().to(playground_route)))
            .service(
                web::resource("/default/health_check").route(web::get().to(health_check_route)),
            )
    };

    if is_running_on_lambda() {
        run_actix_on_lambda(app).await?;
    } else {
        let port = env::var("PORT").unwrap_or("3000".to_string());
        HttpServer::new(app)
            .bind(format!("127.0.0.1:{}", port))?
            .run()
            .await?;
    }

    Ok(())
}

async fn health_check_route() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("ok"))
}

async fn playground_route() -> actix_web::Result<HttpResponse> {
    playground_handler("/default/graphql", None).await
}

async fn graphql_route(
    req: HttpRequest,
    payload: web::Payload,
    schema: web::Data<Schema>,
) -> actix_web::Result<HttpResponse> {
    let auth_user = authenticate(&req).await;
    let context = Context::new(auth_user).await;
    graphql_handler(&schema, &context, req, payload).await
}

async fn authenticate(req: &HttpRequest) -> AuthUser {
    if let Some(id) = get_into(req.headers(), "x-admin-user-id") {
        return AuthUser::Admin(id);
    }

    if let Some(id) = get_into(req.headers(), "x-service-user-id") {
        return AuthUser::Service(id);
    }

    let token: &str = get(req.headers(), "authorization").unwrap_or_default();
    if token.len() < 7 {
        return AuthUser::None;
    }

    let result = app::aws::cognite::verify_token(&token[7..]).await;
    if let Err(_e) = result {
        return AuthUser::None;
    }

    result.ok().unwrap()
}

fn get_into<T>(headers: &HeaderMap, key: &str) -> Option<T>
where
    T: FromStr,
{
    headers.get(key)?.to_str().ok()?.parse::<T>().ok()
}

fn get<'a, 'b>(headers: &'a HeaderMap, key: &'b str) -> Option<&'a str> {
    headers.get(key)?.to_str().ok()
}
