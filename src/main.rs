#[macro_use]
extern crate diesel;

use diesel::{
    expression::BoxableExpression,
    pg::Pg,
    query_dsl::{QueryDsl, RunQueryDsl},
    sql_types::Bool,
    ExpressionMethods,
};

table! {
    #[allow(unused_imports)]
    use diesel::sql_types::*;
    connection(id) {
        id -> BigInt,
        tunnel_id -> BigInt,
    }
}

table! {
    #[allow(unused_imports)]
    use diesel::sql_types::*;
    tunnel (id) {
        id -> BigInt,
        name -> Text,
    }
}

#[derive(Debug, Associations, Identifiable, Queryable)]
#[table_name = "connection"]
#[primary_key(id)]
#[belongs_to(Tunnel)]
pub struct Connection {
    pub id: i64,
    pub tunnel_id: i64,
}

#[derive(Queryable, Identifiable, Clone, Debug, PartialEq, Eq)]
#[table_name = "tunnel"]
pub struct Tunnel {
    pub id: i64,
    pub name: String,
}

joinable!(connection -> tunnel(tunnel_id));
allow_tables_to_appear_in_same_query!(connection, tunnel);

fn _filter(
    conn: &diesel::PgConnection,
) -> Result<Vec<(Tunnel, Option<Connection>)>, Box<dyn std::error::Error>> {
    let mut query = tunnel::table.left_join(connection::table).into_boxed();
    for filter in filters() {
        query = query.filter(filter);
    }
    Ok(query.get_results(conn)?)
}

fn filters() -> Vec<Box<dyn BoxableExpression<tunnel::table, Pg, SqlType = Bool>>> {
    let mut wheres: Vec<Box<dyn BoxableExpression<tunnel::table, Pg, SqlType = Bool>>> = Vec::new();
    wheres.push(Box::new(tunnel::name.eq("adam")));
    wheres
}

fn main() {}
