#[cfg(all(feature = "diesel_sqlite_pool", feature = "diesel_postgres_pool"))]
mod databases_tests {
    use rocket_contrib::databases::{database, diesel};

    #[database("foo")]
    struct TempStorage(diesel::SqliteConnection);

    #[database("bar")]
    struct PrimaryDb(diesel::PgConnection);
}

#[cfg(all(feature = "databases", feature = "sqlite_pool"))]
#[cfg(test)]
mod rusqlite_integration_test {
    use rocket::config::{Config, Environment, Value, Map};
    use rocket_contrib::databases::rusqlite;
    use rocket_contrib::database;

    use rusqlite::types::ToSql;

    #[database("test_db")]
    struct SqliteDb(pub rusqlite::Connection);

    #[rocket::async_test]
    async fn deref_mut_impl_present() {
        let mut test_db: Map<String, Value> = Map::new();
        let mut test_db_opts: Map<String, Value> = Map::new();
        test_db_opts.insert("url".into(), Value::String(":memory:".into()));
        test_db.insert("test_db".into(), Value::Table(test_db_opts));
        let config = Config::build(Environment::Development)
            .extra("databases", Value::Table(test_db))
            .finalize()
            .unwrap();

        let mut rocket = rocket::custom(config).attach(SqliteDb::fairing());
        let mut conn = SqliteDb::get_one(rocket.inspect().await).expect("unable to get connection");

        // Rusqlite's `transaction()` method takes `&mut self`; this tests the
        // presence of a `DerefMut` trait on the generated connection type.
        let tx = conn.transaction().unwrap();
        let _: i32 = tx.query_row("SELECT 1", &[] as &[&dyn ToSql], |row| row.get(0)).expect("get row");
        tx.commit().expect("committed transaction");
    }

    #[rocket::async_test]
    async fn deref_impl_present() {
        let mut test_db: Map<String, Value> = Map::new();
        let mut test_db_opts: Map<String, Value> = Map::new();
        test_db_opts.insert("url".into(), Value::String(":memory:".into()));
        test_db.insert("test_db".into(), Value::Table(test_db_opts));
        let config = Config::build(Environment::Development)
            .extra("databases", Value::Table(test_db))
            .finalize()
            .unwrap();

        let mut rocket = rocket::custom(config).attach(SqliteDb::fairing());
        let conn = SqliteDb::get_one(rocket.inspect().await).expect("unable to get connection");
        let _: i32 = conn.query_row("SELECT 1", &[] as &[&dyn ToSql], |row| row.get(0)).expect("get row");
    }
}
