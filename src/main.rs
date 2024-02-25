use axum::Router;

mod routes;
mod handlers;
mod models;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .merge(routes::day0::routes())
        .merge(routes::day1::routes())
        .merge(routes::day4::routes());

    Ok(router.into())
}
