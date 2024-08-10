use syn_ack::{
    rocket::{
        http::{ContentType, Header, Method, Status},
        local::asynchronous::LocalRequest,
    },
    services::TokenPair,
};

pub struct TestFramework {
    client: syn_ack::rocket::local::asynchronous::Client,
    token_pair: Option<TokenPair>,
}

impl TestFramework {
    pub async fn new() -> Result<TestFramework, syn_ack::rocket::Error> {
        let client =
            syn_ack::rocket::local::asynchronous::Client::tracked(syn_ack::start().await).await?;

        Ok(TestFramework {
            client,
            token_pair: None,
        })
    }

    pub fn tokens(&self) -> Option<TokenPair> {
        self.token_pair.clone()
    }

    /// Try to log in and obtain tokens for the API.
    /// Note: This function panics if the credentials are wrong.
    pub async fn login(&mut self, username: impl ToString, password: impl ToString) {
        let response = self
            .post(
                "/api/v1/auth/login",
                Some(format!(
                    r#"{{
                                        "username": "{username}",
                                        "password": "{password}"
                                    }}"#,
                    username = username.to_string(),
                    password = password.to_string()
                )),
            )
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Accepted);
        let Some(token_pair) = response.into_json::<TokenPair>().await else {
            panic!("login response did not match");
        };

        self.token_pair = Some(token_pair);
    }

    /// Try to refresh the current sesseion.
    /// Doing that refreshes the currently stored token pair.
    pub async fn refresh(&mut self) {
        let mut req = self.post("/api/v1/auth/refresh", Option::<&str>::None);

        if let Some(TokenPair { refresh_token, .. }) = &self.token_pair {
            req.replace_header(Header::new(
                "Authorization",
                format!("Bearer {refresh_token}"),
            ));
        };

        let response = req.dispatch().await;

        assert_eq!(response.status(), Status::Accepted);
        let Some(new_tokens) = response.into_json::<TokenPair>().await else {
            panic!("refresh response did not match");
        };

        self.token_pair.replace(new_tokens);
    }

    /// Log out of the currently registered session.
    pub async fn logout(&mut self) {
        {
            let response = self.get("/api/v1/auth/logout").dispatch().await;

            assert_eq!(response.status(), Status::Ok);
        }
        self.token_pair = None;
    }

    fn req(&self, method: Method, url: impl ToString) -> LocalRequest {
        let mut req = self
            .client
            .req(method, url.to_string())
            .header(ContentType::JSON);

        if let Some(TokenPair { access_token, .. }) = &self.token_pair {
            req = req.header(Header::new(
                "Authorization",
                format!("Bearer {access_token}"),
            ));
        };

        req
    }

    /// Perform a get request against against the API. If present, the Authorization header will be
    /// set.
    pub fn get(&self, url: impl ToString) -> LocalRequest {
        self.req(Method::Get, url).clone()
    }

    /// Perform a post request against against the API. If present, the Authorization header will be
    /// set.
    pub fn post(&self, url: impl ToString, body: Option<impl ToString>) -> LocalRequest {
        let mut req = self.req(Method::Post, url);

        if let Some(body) = body {
            req = req.body(body.to_string());
        };

        req.clone()
    }
}
