fn f(
    pool: &diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>>,
) -> Result<(), diesel::r2d2::Error> {
    let mut _conn = pool.get().map_err(|e| -> diesel::r2d2::Error { e })?;
    Ok(())
}

fn main() {
    println!("Hello, world!");
}
