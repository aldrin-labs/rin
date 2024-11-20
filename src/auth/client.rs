use crate::auth::Supabase;
use reqwest::Client;
use dotenvy_macro::dotenv;

impl Supabase {
    pub fn new() -> Self {
        let client = Client::new();
        let url = dotenv!("SUPABASE_URL");
        let api_key = dotenv!("SUPABASE_API_KEY");

        Supabase {
            client,
            url: url.to_string(),
            api_key: api_key.to_string(),
            user: None,
        }
    }
}
