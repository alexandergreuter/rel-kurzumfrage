pub mod models;
pub mod schema;

use std::{convert::Infallible, env};

use deadpool::managed::{Object, Pool};
use diesel::{
    Connection, ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, Selectable,
    SelectableHelper,
};
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection, RunQueryDsl,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::{reply, Filter, Rejection};

type DBPool = Pool<AsyncDieselConnectionManager<diesel_async::AsyncPgConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::locations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Location {
    pub id: Uuid,
    pub title: String,
    pub prompt: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::votes)]
pub struct NewVote {
    pub user_agent: String,
    pub agrees: bool,
    pub comment: String,
    pub location_id: Uuid,
}

#[derive(Deserialize)]
pub struct AddVoteBody {
    pub agrees: bool,
    pub comment: String,
    pub location_id: Uuid,
}

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to read env");
    env_logger::init();

    let log = warp::log("warp_server");



    // create a new connection pool with the default config
    let database_url: String = expect_env_var("DATABASE_URL");

    run_migrations(&database_url);
    let config: AsyncDieselConnectionManager<diesel_async::AsyncPgConnection> =
        AsyncDieselConnectionManager::new(database_url);
    let pool: Pool<AsyncDieselConnectionManager<diesel_async::AsyncPgConnection>> =
        Pool::builder(config).build().expect("foo");
    log::info!("Database ready");

    log::info!("Listening on 127.0.0.1:8080");

    let routes = warp::any()
        .and(
            warp::get()
                .and(warp::path!("locations"))
                .and(with_db(pool.clone()))
                .and_then(get_locations)
                .map(|it| reply::json(&it)),
        )
        .or(warp::get()
            .and(warp::path!("locations" / Uuid))
            .and(with_db(pool.clone()))
            .and_then(get_location)
            .map(|it: Location| reply::json(&it)))
        .or(warp::post()
            .and(warp::path!("votes"))
            .and(with_db(pool.clone()))
            .and(warp::body::json())
            .and(warp::header("user-agent"))
            .and_then(add_vote)
            .map(|it| reply::json(&it)));

    warp::serve(
        warp::any()
            .and(routes)
            .with(warp::cors().allow_any_origin().allow_methods(vec!["POST", "GET"]).allow_headers(vec!["content-type"]))
            .with(log),
    )
    .run(([0, 0, 0, 0], 8080))
    .await
}

fn run_migrations(database_url: &str) {
    PgConnection::establish(database_url)
        .unwrap_or_else(|err| panic!("Failed to run migrations {}", err))
        .run_pending_migrations(MIGRATIONS)
        .unwrap_or_else(|err| panic!("Failed to run migrations {}", err));
}

fn with_db(
    pool: Pool<AsyncDieselConnectionManager<diesel_async::AsyncPgConnection>>,
) -> impl Filter<Extract = (DBPool,), Error = Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

fn expect_env_var(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| panic!("Env variable: {} must be set", name))
}

async fn get_conn(
    db: DBPool,
) -> Result<Object<AsyncDieselConnectionManager<AsyncPgConnection>>, Rejection> {
    db.get().await.map_err(|_err| warp::reject::not_found())
}

async fn get_locations(db: DBPool) -> Result<Vec<Location>, Rejection> {
    use self::schema::locations::dsl::*;

    locations
        .select(Location::as_select())
        .load(&mut get_conn(db).await?)
        .await
        .map_err(|_err| warp::reject::not_found())
}

async fn get_location(incoming_id: Uuid, db: DBPool) -> Result<Location, Rejection> {
    use self::schema::locations::dsl::*;

    locations
        .filter(id.eq(incoming_id))
        .select(Location::as_select())
        .first(&mut get_conn(db).await?)
        .await
        .map_err(|_err| warp::reject::not_found())
}

async fn add_vote(db: DBPool, body: AddVoteBody, user_agent: String) -> Result<(), Rejection> {
    diesel::insert_into(self::schema::votes::table)
        .values(NewVote {
            location_id: body.location_id,
            agrees: body.agrees,
            comment: body.comment,
            user_agent,
        })
        .execute(&mut get_conn(db).await?)
        .await
        .map_err(|_err| warp::reject::not_found())?;

    Ok(())
}
