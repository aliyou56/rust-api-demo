use axum::debug_handler;

#[debug_handler]
pub async fn hello_world() -> &'static str {
    println!("Got request: /hello");
    "Hello, world"
}

#[cfg(test)]
mod tests {
    use crate::handlers::*;

    #[tokio::test]
    async fn test_hello_world() {
        let actual = hello_world().await;
        let expected = "Hello, world";
        assert!(actual == expected);
    }
}
