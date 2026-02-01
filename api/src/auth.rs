use axum::http::{Request, Response, StatusCode, header};
use std::marker::PhantomData;
use tower_http::auth::require_authorization::Bearer;
use tower_http::validate_request::ValidateRequest;

#[derive(Debug)]
pub struct CheckToken<ResBody>(String, PhantomData<fn() -> ResBody>);

impl<ResBody> Clone for CheckToken<ResBody> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl<ResBody> CheckToken<ResBody> {
    pub fn new(token: &str) -> Self {
        Self(format!("Token {token}"), PhantomData)
    }
}

impl<B, ResBody> ValidateRequest<B> for CheckToken<ResBody>
where
    ResBody: Default,
{
    type ResponseBody = ResBody;

    fn validate(&mut self, request: &mut Request<B>) -> Result<(), Response<Self::ResponseBody>> {
        match request.headers().get(header::AUTHORIZATION) {
            Some(actual) if actual == &self.0 => Ok(()),
            _ => {
                let mut res = Response::new(ResBody::default());
                *res.status_mut() = StatusCode::UNAUTHORIZED;
                Err(res)
            }
        }
    }
}
