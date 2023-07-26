pub mod schema;

use std::env;

use anyhow::Context;
use deadpool::managed::{Object, Pool};
use diesel::{
    Connection, ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, Selectable,
    SelectableHelper, ConnectionResult, ConnectionError,
};
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection, RunQueryDsl,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use futures_util::{future::BoxFuture, FutureExt};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::{
    reject::{self},
    reply, Filter, Rejection,
};

type DBPool = Pool<AsyncDieselConnectionManager<diesel_async::AsyncPgConnection>>;
type DBConn = Object<AsyncDieselConnectionManager<AsyncPgConnection>>;

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
    pub comment: Option<String>,
    pub location_id: Uuid,
}

#[derive(Deserialize)]
pub struct AddVoteBody {
    pub agrees: bool,
    pub comment: Option<String>,
    pub location_id: Uuid,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().expect("Failed to read env");
    env_logger::init();

    // create a new connection pool with the default config
    let database_url: String = expect_env_var("DATABASE_URL");

    log::info!("Setting up DB {}", database_url);
    let pool = setup_db(&database_url)?;
    log::info!("Database set up successfully");

    warp::serve(
        warp::any()
            .and(
                warp::any()
                    .and(
                        warp::path!("locations")
                            .and(warp::get())
                            .and(with_db(pool.clone()))
                            .and_then(get_locations)
                            .map(|it| reply::json(&it)),
                    )
                    .or(warp::path!("locations" / Uuid)
                        .and(warp::get())
                        .and(with_db(pool.clone()))
                        .and_then(get_location)
                        .map(|it: Location| reply::json(&it)))
                    .or(warp::path!("votes")
                        .and(warp::post())
                        .and(with_db(pool.clone()))
                        .and(warp::body::json())
                        .and(warp::header("user-agent"))
                        .and_then(add_vote)
                        .map(|it| reply::json(&it))),
            )
            .recover(handle_rejection)
            .with(
                warp::cors()
                    .allow_any_origin()
                    .allow_methods(vec!["POST", "GET"])
                    .allow_headers(vec!["content-type"]),
            )
            .with(warp::log("warp_server")),
    )
    .run(([0, 0, 0, 0], 8080))
    .await;

    Ok(())
}

async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(anyhow_err) = err.find::<AnyhowError>() {
        log::error!("Internal server error: {:?}", anyhow_err.0);
        return Ok(warp::reply::with_status(
            "Internal server error",
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    // If the custom rejection isn't found, return the original rejection
    Err(err)
}

fn setup_db(database_url: &str) -> anyhow::Result<DBPool> {
    let mut conn =
        PgConnection::establish(database_url).context("Failed to establish database connection")?;

    match conn.run_pending_migrations(MIGRATIONS) {
        Ok(_) => log::info!("Database migrations ran successfully"),
        Err(err) => return Err(anyhow::anyhow!("Failed to run migrations: {}", err)),
    }

    Pool::builder(
        AsyncDieselConnectionManager::<AsyncPgConnection>::new_with_setup(
            database_url,
            establish_connection,
        )
    )
        .build()
        .context("Failed to init DB connection pool")
}

fn establish_connection(config: &str) -> BoxFuture<ConnectionResult<AsyncPgConnection>> {
    let fut = async {
        // We first set up the way we want rustls to work.
        let rustls_config = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(root_certs())
            .with_no_client_auth();
        let tls = tokio_postgres_rustls::MakeRustlsConnect::new(rustls_config);
        let (client, conn) = tokio_postgres::connect(config, tls)
            .await
            .map_err(|e| ConnectionError::BadConnection(e.to_string()))?;
        tokio::spawn(async move {
            if let Err(e) = conn.await {
                eprintln!("Database connection: {e}");
            }
        });
        AsyncPgConnection::try_from(client).await
    };
    fut.boxed()
}

fn root_certs() -> rustls::RootCertStore {
    let mut roots = rustls::RootCertStore::empty();
    let certs = rustls_native_certs::load_native_certs().expect("Certs not loadable!");
    let certs: Vec<_> = certs.into_iter().map(|cert: rustls_native_certs::Certificate| cert.0).collect();
    roots.add_parsable_certificates(&certs);
    roots
}

fn with_db(
    pool: Pool<AsyncDieselConnectionManager<diesel_async::AsyncPgConnection>>,
) -> impl Filter<Extract = (DBConn,), Error = Rejection> + Clone {
    warp::any().and_then(move || {
        let pool = pool.clone();
        async move {
            pool.get()
                .await
                .context("Failed to retrieve db connection from pool")
                .err_as_reject()
        }
    })
}

fn expect_env_var(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| panic!("Env variable: {} must be set", name))
}

async fn get_locations(mut conn: DBConn) -> Result<Vec<Location>, Rejection> {
    use self::schema::locations::dsl::*;

    locations
        .select(Location::as_select())
        .load(&mut conn)
        .await
        .err_as_reject()
}

async fn get_location(incoming_id: Uuid, mut conn: DBConn) -> Result<Location, Rejection> {
    use self::schema::locations::dsl::*;

    locations
        .filter(id.eq(incoming_id))
        .select(Location::as_select())
        .first(&mut conn)
        .await
        .err_as_reject()
}

async fn add_vote(
    mut conn: DBConn,
    body: AddVoteBody,
    user_agent: String,
) -> Result<(), Rejection> {
    diesel::insert_into(self::schema::votes::table)
        .values(NewVote {
            location_id: body.location_id,
            agrees: body.agrees,
            comment: body.comment,
            user_agent,
        })
        .execute(&mut conn)
        .await
        .err_as_reject()?;

    Ok(())
}

#[derive(Debug)]
struct AnyhowError(anyhow::Error);

impl reject::Reject for AnyhowError {}

trait IntoReject<T, E>
where
    E: Into<anyhow::Error>,
{
    fn err_as_reject(self) -> Result<T, Rejection>;
}

impl<T, E> IntoReject<T, E> for Result<T, E>
where
    E: Into<anyhow::Error>,
{
    fn err_as_reject(self) -> Result<T, Rejection> {
        self.map_err(|e| reject::custom(AnyhowError(e.into())))
    }
}
