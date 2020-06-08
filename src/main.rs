#[macro_use]
extern crate diesel;
use diesel::prelude::*;
use diesel::sql_types::*;

// Define the tunnel table and struct
table! {
    #[allow(unused_imports)]
    use diesel::sql_types::*;
    tunnels (id) {
        id -> BigInt,
        name -> Text,
    }
}
#[derive(Queryable, Identifiable, Clone, Debug, PartialEq, Eq)]
pub struct Tunnel {
    pub id: i64,
    pub name: String,
}

table! {
    #[allow(unused_imports)]
    dead_tunnels (id) {
        id -> BigInt,
        name -> Text,
        close_reason -> BigInt,
        new_col -> Text,
    }
}
#[derive(Queryable, Identifiable, Clone, Debug, PartialEq, Eq)]
pub struct DeadTunnel {
    pub id: i64,
    pub name: String,
    pub new_col: String,
}

fn kill_tunnel(
    conn: &diesel::PgConnection,
    name: &'static str,
) -> Result<(), Box<dyn std::error::Error>> {
    tunnels::table
        .select((tunnels::all_columns, 14.into_sql::<BigInt>()))
        .insert_into(dead_tunnels::table)
        .into_columns((
            (dead_tunnels::id, dead_tunnels::name),
            dead_tunnels::close_reason,
        ))
        .execute(conn)?;
    Ok(())
}

fn main() {}
