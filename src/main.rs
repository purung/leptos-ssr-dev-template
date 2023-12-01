#[cfg(feature = "ssr")]
mod lpt;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use std::{env, time::Duration};

    use axum::routing::get;
    use axum::Router;
    use birds_psy::app::*;
    use birds_psy::fileserv::file_and_error_handler;
    use http::{HeaderValue, Method};
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use sqlx::{migrate, migrate::Migrator, postgres::PgPoolOptions};
    use tower_cookies::CookieManagerLayer;
    use tower_http::cors::CorsLayer;
    use tracing::info;

    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    static MIGRATOR: Migrator = migrate!();

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let pg_addr = env::var("DATABASE_URL").unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(3)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(90))
        .connect_lazy(&pg_addr)
        .expect("can't connect to database");

    MIGRATOR
        .run(&pool)
        .await
        .expect("migrations to run smoothly");

    let app_state = lpt::AppState {
        leptos_options,
        pool: pool.clone(),
        routes: routes.clone(),
    };
    let cors_layer = CorsLayer::new()
        .allow_origin(
            env::var("HOMEPAGE")
                .expect("homepage to be set for cors")
                .parse::<HeaderValue>()
                .expect("homepage to be a valid url"),
        )
        .allow_methods([Method::POST, Method::GET]);

    info!("{:?}", cors_layer);

    // build our application with a route
    let app = Router::new()
        .route(
            "/api/*fn_name",
            get(lpt::server_fn_handler).post(lpt::server_fn_handler),
        )
        // .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        // .leptos_routes(&leptos_options, routes, || view! { <App /> })
        .leptos_routes_with_handler(routes, get(lpt::leptos_routes_handler))
        .fallback(file_and_error_handler)
        .layer(CookieManagerLayer::new())
        // .layer(TraceLayer::new_for_http())
        .layer(cors_layer)
        .with_state(app_state);
    // .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log::info!("listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
