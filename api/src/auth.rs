use axum::http::{Request, Response, StatusCode};
use std::marker::PhantomData;
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
        Self(token.to_string(), PhantomData)
    }
}

impl<B, ResBody> ValidateRequest<B> for CheckToken<ResBody>
where
    ResBody: Default,
{
    type ResponseBody = ResBody;

    fn validate(&mut self, request: &mut Request<B>) -> Result<(), Response<Self::ResponseBody>> {
        let query_pairs =
            form_urlencoded::parse(request.uri().query().unwrap_or_default().as_bytes());
        for (k, v) in query_pairs {
            if k == "token" && v == self.0 {
                return Ok(());
            }
        }
        let mut res = Response::new(ResBody::default());
        *res.status_mut() = StatusCode::UNAUTHORIZED;
        Err(res)
    }
}
