use actix_web::{dev::ServiceRequest, error::ErrorUnauthorized, Error as ActixError};
use actix_web_httpauth::extractors::basic::BasicAuth;
use std::hash::{DefaultHasher, Hash, Hasher};

pub async fn do_hello_auth(
    req: ServiceRequest,
    creds: BasicAuth,
) -> Result<ServiceRequest, (ActixError, ServiceRequest)> {
    if calculate_hash(&creds.password()) == calculate_hash(&Some("pass")) {
        return Ok(req);
    }
    return Err((
        ErrorUnauthorized(
            "401 Unauthorized: Sorry this page is protected and you are a dumbass lol",
        ),
        req,
    ));
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
