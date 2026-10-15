pub struct App {}

impl App {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        dotenvy::dotenv()?;

        let client_id = env::var("CLIENT_ID")
            .map(ClientId::new)
            .expect("CLIENT_ID should be provided.");
        let client_secret = env::var("CLIENT_SECRET")
            .map(ClientSecret::new)
            .expect("CLIENT_SECRET should be provided.");

        let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string())?;
        let token_url = TokenUrl::new("https://github.com/login/oauth/access_token".to_string())?;
        let client = BasicClient::new(client_id)
            .set_client_secret(client_secret)
            .set_auth_uri(auth_url)
            .set_token_uri(token_url);

        let db = SqlitePool::connect(":memory:").await?;
        sqlx::migrate!("./migrations").run(&db).await?;

        Ok(Self { db, client })
    }

    pub async fn serve(self) -> Result<(), Box<dyn Error>> {
        // Session layer.
        let session_store = MemoryStore::default();
        let session_layer = SessionManagerLayer::new(session_store)
            .with_secure(false)
            .with_same_site(SameSite::Lax)
            .with_expiry(Expiry::OnInactivity(Deration::days(1)));

        // Auth service.
        let backend = Backend::new(self.db, self.client);
        let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

        let require_login = Require::<Backend>::builder()
            .unaunthenticated(RedirectHandler::new().login_url("/login"))
            .build();

        let app =protected::router()
            .route_layer(require_login)
            .merge(auth::router())
            .merge(oauth::router())
            .layer(auth_layer);

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
        axum::serve(listener, app.into_make_service()).await?;

        Ok(())
    }
}
