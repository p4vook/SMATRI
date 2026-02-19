use tower::{Service, Layer};
use std::{
    sync::{
        Arc,
        Mutex
    },
    task::{
        Poll,
        Context
    }
};
use http::{Request, Response};

#[derive(Clone)]
pub struct OhttpLayer {
    ohttp_server: Arc<Mutex<i32>>
}

impl OhttpLayer {
    pub fn new(ohttp_server: Arc<Mutex<i32>>) -> Self {
        Self { ohttp_server }
    }
}

impl<S> Layer<S> for OhttpLayer
{
    type Service = OhttpService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        OhttpService {
            inner,
            ohttp_server: self.ohttp_server.clone()
        }
    }
}

#[derive(Clone)]
pub struct OhttpService<S> {
    inner: S,
    ohttp_server: Arc<Mutex<i32>>
}

impl<ReqBody, ResBody, S> Service<Request<ReqBody>> for OhttpService<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>>
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        assert!(req.headers().contains_key("X-Abracadabra"));
        self.inner.call(req)
    }
}
