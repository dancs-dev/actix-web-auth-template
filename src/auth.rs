use std::future::{ready, Ready};

use actix_web::{
    body::EitherBody,
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    web::Data,
    Error, HttpResponse,
};
use futures_util::future::LocalBoxFuture;

use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub struct CheckAuth;

impl<S, B> Transform<S, ServiceRequest> for CheckAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = CheckAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CheckAuthMiddleware { service }))
    }
}


pub struct CheckAuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for CheckAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        // Check the JWT here:
        let mut authorised = false;
        use crate::actions::validate_token;
        if let Some(auth_header) = request.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer") {
                    let token = auth_str[6..auth_str.len()].trim();
                    if let Some(conn) = request.app_data::<Data<DbPool>>() {
                        authorised = validate_token(token, &mut conn.get().unwrap())
                    }
                }
            }
        }

        // If not authorised, return NotAuthorized
        if !authorised && !vec!["/signup","/signin"].contains(&request.path()) {
            println!("{}", request.path());
            let (request, _pl) = request.into_parts();

            let response = HttpResponse::Unauthorized().body("Invalid").map_into_right_body();

            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
        }

        let res = self.service.call(request);

        Box::pin(async move {
            res.await.map(ServiceResponse::map_into_left_body)
        })
    }
}
