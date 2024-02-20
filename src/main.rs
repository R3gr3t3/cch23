use axum::Router;

mod routes;
mod handlers;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().merge(routes::day0::routes());

    Ok(router.into())
}
