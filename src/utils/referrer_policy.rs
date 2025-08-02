use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpMessage};
use futures_util::future::{ok, Ready, LocalBoxFuture};
use actix_web::http::{header::HeaderName, header::HeaderValue};
use std::rc::Rc;
use std::task::{Context, Poll};

pub struct ReferrerPolicy;

impl<S, B> Transform<S, ServiceRequest> for ReferrerPolicy
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = ReferrerPolicyMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ReferrerPolicyMiddleware {
            service: Rc::new(service),
        })
    }
}

pub struct ReferrerPolicyMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for ReferrerPolicyMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();

        Box::pin(async move {
            let mut res = srv.call(req).await?;

            res.headers_mut().insert(
                HeaderName::from_static("referrer-policy"),
                HeaderValue::from_static("no-referrer"),
            );
            Ok(res)
        })
    }
}