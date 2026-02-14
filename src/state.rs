use tokio::sync::OnceCell;

pub static POOL: OnceCell<sqlx::PgPool> = OnceCell::const_new();
pub static WEBHOOK_URL: OnceCell<String> = OnceCell::const_new();

pub fn init_pool(pool: sqlx::PgPool) {
    POOL.set(pool).expect("Failed to initialize pool");
}
pub fn get_pool() -> &'static sqlx::PgPool {
    POOL.get().expect("Pool not initialized")
}
pub fn init_webhook(url: String) {
    WEBHOOK_URL.set(url).expect("Failed to initialize webhook");
}
pub fn get_webhook() -> &'static String {
    WEBHOOK_URL.get().expect("Webhook not initialized")
}
