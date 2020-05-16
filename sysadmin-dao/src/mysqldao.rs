use sqlx::mysql::MySqlPool;

pub fn clientPool(conectStr: &String) -> Option<MySqlPool> {
    if conectStr.is_empty() {
        return None;
    }
    MySqlPool::new(conectStr).await?
}
