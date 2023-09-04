use twilight_http::Client as HttpClient;
use songbird::Songbird;

pub struct AppContext {
    pub http: HttpClient,
    // TODO: make a wrapper around Songbird to prevent having multiple Call instances
    pub songbird: Songbird
}
