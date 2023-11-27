use axum::{
    body::Body as AxumBody,
    extract::{FromRef, Path, RawQuery, State},
    response::{IntoResponse, Response},
};
use birds_psy::app::{App, MaybeUser};
use http::{HeaderMap, Request};
use leptos::{logging::log, provide_context, LeptosOptions};
use leptos_axum::handle_server_fns_with_context;
use leptos_router::RouteListing;
use tower_cookies::Cookies;

#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub pool: sqlx::postgres::PgPool,
    pub routes: Vec<RouteListing>,
}

pub async fn server_fn_handler(
    State(app_state): State<AppState>,
    cookies: Cookies,
    maybeuser: MaybeUser,
    path: Path<String>,
    headers: HeaderMap,
    raw_query: RawQuery,
    request: Request<AxumBody>,
) -> impl IntoResponse {
    log!("{:?}", path);

    handle_server_fns_with_context(
        path,
        headers,
        raw_query,
        move || {
            provide_context(app_state.pool.clone());
            provide_context(app_state.leptos_options.clone());
            provide_context(maybeuser.clone());
            provide_context(cookies.clone());
        },
        request,
    )
    .await
}

pub async fn leptos_routes_handler(
    State(app_state): State<AppState>,
    cookies: Cookies,
    maybeuser: MaybeUser,
    req: Request<AxumBody>,
) -> Response {
    let handler = leptos_axum::render_route_with_context(
        app_state.leptos_options.clone(),
        app_state.routes.clone(),
        move || {
            provide_context(app_state.pool.clone());
            provide_context(app_state.leptos_options.clone());
            provide_context(maybeuser.clone());
            provide_context(cookies.clone());
        },
        App,
    );
    handler(req).await.into_response()
}
