use sqlx::mysql::MySqlPoolOptions;

#[derive(sqlx::FromRow)]
struct User {
    id: u64,
}

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("")
        .await?;
    let rows = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&pool)
        .await?;
    println!("{}", rows[0].id);
    Ok(())
}
