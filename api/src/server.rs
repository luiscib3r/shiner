use axum::{
    Router,
    extract::DefaultBodyLimit,
    http::{Method, header::CONTENT_TYPE},
    serve,
};
use tokio::net::TcpListener;
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    trace::TraceLayer,
};

use crate::{health, images};

pub struct Server {
    router: Router,
}

impl Server {
    pub async fn new() -> Self {
        let health_router = health::router::new();
        let images_router = images::router::new();

        let mut router = Router::new()
            .merge(health_router)
            .merge(images_router)
            .layer(DefaultBodyLimit::disable())
            .layer(TraceLayer::new_for_http());

        let cors = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
            .allow_headers([CONTENT_TYPE])
            .allow_origin(AllowOrigin::exact("*".parse().unwrap()));

        router = router.layer(cors);

        Self { router }
    }

    pub async fn run(&self, addr: &str) {
        let listener = TcpListener::bind(addr).await.unwrap();
        println!(">>> {:<12} Listening on {addr}", "Server");
        serve(listener, self.router.clone()).await.unwrap()
    }
}
