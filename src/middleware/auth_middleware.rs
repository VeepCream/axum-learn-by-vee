use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::config;
use crate::utils::jwt_authentication::verify_token;


pub async fn token_authorization(
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if let Some(jwt) = req.headers().get(header::AUTHORIZATION) {
        if let Ok(jwt_header) = jwt.to_str() {
            let jwt_header = jwt_header.replace("Bearer ", "");
            if let Ok(claims) = verify_token(config::SECRET_ENV.to_string(), jwt_header.to_owned())
            {
                req.extensions_mut().insert(claims.id);
                return Ok(next.run(req).await);
            };
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}