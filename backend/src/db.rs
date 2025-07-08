// Bring in Diesel's Postgres connection type
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
// Bring in r2d2 connection pooling support
// `self` means you're also bringing in the `r2d2` module itself
// `ConnectionManager<PgConnection>` tells r2d2 how to manage Diesel Postgres connections
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>; // A pool that manages PostgreSQL connections using Diesel's PgConnection, with help from r2d2
// ConnectionManager<PgConnection>
// This is a specific "connection manager" that knows how to create and manage
//PostgreSQL connections

pub fn establish_connection_pool(database_url: &str)-> DbPool{
    // Create a connection manager for Postgres
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    // Build a new connection pool from the manager
    // `.build(manager)` returns a `Result`
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB connection pool")
}

