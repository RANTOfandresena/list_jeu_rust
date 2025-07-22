use actix_web::{body::MessageBody, dev::{ServiceRequest, ServiceResponse}, http::header::AUTHORIZATION, middleware::Next, Error, HttpMessage};

use crate::utils::{api_response::{self, ApiResponse}, jwt::decode_jwt};



pub async fn check_auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>
) -> Result<ServiceResponse<impl MessageBody>,Error> {
    let auth = req.headers().get(AUTHORIZATION);

    if auth.is_none() {
        return Err(Error::from(api_response::ApiResponse::new(401,"non authentifier".to_string())));
    }
    let token = auth.unwrap().to_str().unwrap().replace("Bearer ", "").to_owned();
    let cleam = decode_jwt(token).unwrap();
    req.extensions_mut().insert(cleam.claims);
    
    next.call(req).await
        .map_err(|error| 
            Error::from(ApiResponse::new(500, error.to_string()))
        )
}