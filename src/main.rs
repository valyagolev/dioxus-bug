use axum::{extract::WebSocketUpgrade, response::Html, routing::get, Router};

pub mod app;

#[tokio::main]
async fn main() {
    let addr: std::net::SocketAddr = ([0, 0, 0, 0], 33030).into();

    let view = dioxus_liveview::LiveViewPool::new();

    let app = Router::new()
        .fallback(
            // "/*an    ything",
            get(move || async move {
                Html(format!(
                    r#"
            <!DOCTYPE html>
            <html>
            <head>
                <title>Cookbot Admin UI</title>
                <link rel="stylesheet" href="/public/tailwind.css" />
            </head>
            <body> <div id="main"></div> </body>
            {glue}
            </html>
            "#,
                    // Create the glue code to connect to the WebSocket on the "/ws" route
                    glue = dioxus_liveview::interpreter_glue(&format!(
                        "ws://0.0.0.0:{}/ws",
                        addr.port()
                    ))
                ))
            }),
        )
        // .nest("/public", axum_static::static_router("public"))
        // The root route contains the glue code to connect to the WebSocket
        // The WebSocket route is what Dioxus uses to communicate with the browser
        .route(
            "/ws",
            get(move |ws: WebSocketUpgrade| async move {
                ws.on_upgrade(move |socket| async move {
                    // When the WebSocket is upgraded, launch the LiveView with the app component
                    _ = view
                        .launch(dioxus_liveview::axum_socket(socket), app::app)
                        .await;
                })
            }),
        );

    println!("Listening on http://{addr}");

    axum::Server::bind(&addr.to_string().parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
