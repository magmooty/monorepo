#[cfg(test)]
mod tests {
    use crate::database::{Database, SigninCode};

    async fn setup() -> Database {
        Database::in_memory().await
    }

    #[tokio::test]
    async fn test_create_signin_code() {
        let db = setup().await;

        db.signin_code.create_signin_code(&"+201096707442".to_string(), 0).await;
        let codes: Vec<SigninCode> = db.surreal.select("signin_code").await.unwrap();
        assert_eq!(codes.len(), 1);

        db.signin_code.create_signin_code(&"+201096707442".to_string(), 0).await;
        let codes: Vec<SigninCode> = db.surreal.select("signin_code").await.unwrap();
        assert_eq!(codes.len(), 2);
    }

    #[tokio::test]
    async fn test_delete_previous_signin_codes() {
        let db = setup().await;

        db.signin_code.create_signin_code(&"+201096707442".to_string(), 0).await;
        let codes: Vec<SigninCode> = db.surreal.select("signin_code").await.unwrap();
        assert_eq!(codes.len(), 1);

        db.signin_code.create_signin_code(&"+201096707442".to_string(), 0).await;
        let codes: Vec<SigninCode> = db.surreal.select("signin_code").await.unwrap();
        assert_eq!(codes.len(), 2);

        db.signin_code.delete_previous_signin_codes(&"+201096707442".to_string()).await;
        let codes: Vec<SigninCode> = db.surreal.select("signin_code").await.unwrap();
        assert_eq!(codes.len(), 0);
    }
}
