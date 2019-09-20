use crate::proto::{error::Error, health::HealthEndpoint, lease::LeaseEndpoint};
use http::{
    method::Method,
    uri::{Authority, Scheme, Uri},
};
use hyper::{Body, Request};
// use lazy_static::lazy_static;
use serde_json::json;
use std::time::Duration;

#[derive(Debug)]
pub struct Protocol {
    version: String,
    scheme: Scheme,
    authority: Authority,
}

impl HealthEndpoint for Protocol {
    #[inline]
    fn health(&self) -> Result<Request<Body>, Error> {
        let uri = Uri::builder()
            .scheme(self.scheme.clone())
            .authority(self.authority.clone())
            .path_and_query(format!("{}{}", self.version, Self::HEALTH_ENDPOINT).as_str())
            .build()?;

        Request::builder()
            .uri(uri)
            .body(Body::empty())
            .map_err(Error::HttpError)
    }
}

impl LeaseEndpoint for Protocol {
    //
    //
    //
    #[inline]
    fn read_lease(&self, id: &str) -> Result<Request<Body>, Error> {
        let uri = Uri::builder()
            .scheme(self.scheme.clone())
            .authority(self.authority.clone())
            .path_and_query(format!("{}{}", self.version, Self::READ_LEASE_ENDPOINT).as_str())
            .build()?;

        let payload = {
            let inner = json!({ "lease_id": id });
            serde_json::to_string(&inner)
        }
        .map_err(Error::JsonError)?;

        Request::builder()
            .method(Method::PUT)
            .uri(uri)
            .body(Body::from(payload))
            .map_err(Error::HttpError)
    }

    #[inline]
    fn list_lease(&self, prefix: Option<&str>) -> Result<Request<Body>, Error> {
        let query = if let Some(p) = prefix {
            format!("{}{}/{}", self.version, Self::READ_LEASE_ENDPOINT, p)
        } else {
            format!("{}{}", self.version, Self::READ_LEASE_ENDPOINT)
        };

        let uri = Uri::builder()
            .scheme(self.scheme.clone())
            .authority(self.authority.clone())
            .path_and_query(query.as_str())
            .build()?;

        Request::builder()
            .method("LIST")
            .uri(uri)
            .body(Body::empty())
            .map_err(Error::HttpError)
    }

    #[inline]
    fn renew_lease(&self, id: &str, duration: Duration) -> Result<Request<Body>, Error> {
        let uri = Uri::builder()
            .scheme(self.scheme.clone())
            .authority(self.authority.clone())
            .path_and_query(format!("{}{}", self.version, Self::RENEW_LEASE_ENDPOINT).as_str())
            .build()?;

        let payload = {
            let inner = json!({ "lease_id": id, "increment": duration.as_secs() });
            serde_json::to_string(&inner)
        }
        .map_err(Error::JsonError)?;

        Request::builder()
            .method(Method::PUT)
            .uri(uri)
            .body(Body::from(payload))
            .map_err(Error::HttpError)
    }

    // revoke_lease
    #[inline]
    fn revoke_lease(&self, id: &str) -> Result<Request<Body>, Error> {
        let uri = Uri::builder()
            .scheme(self.scheme.clone())
            .authority(self.authority.clone())
            .path_and_query(format!("{}{}", self.version, Self::REVOKE_LEASE_ENDPOINT).as_str())
            .build()?;

        let payload = {
            let inner = json!({ "lease_id": id });
            serde_json::to_string(&inner)
        }
        .map_err(Error::JsonError)?;

        Request::builder()
            .method(Method::PUT)
            .uri(uri)
            .body(Body::from(payload))
            .map_err(Error::HttpError)
    }

    #[inline]
    fn revoke_prefix(&self, prefix: &str, forced: bool) -> Result<Request<Body>, Error> {
        let endpoint = if forced {
            Self::REVOKE_LEASE_FORCE_ENDPOINT
        } else {
            Self::REVOKE_LEASE_PREFIX_ENDPOINT
        };

        let uri = Uri::builder()
            .scheme(self.scheme.clone())
            .authority(self.authority.clone())
            .path_and_query(format!("{}{}/{}", self.version, endpoint, prefix).as_str())
            .build()?;

        Request::builder()
            .method(Method::PUT)
            .uri(uri)
            .body(Body::empty())
            .map_err(Error::HttpError)
    }
}
