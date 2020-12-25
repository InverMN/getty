use lazy_static::lazy_static;
use prometheus::{HistogramOpts, HistogramVec, IntCounter, IntCounterVec, IntGauge, Opts, Registry};

lazy_static! {
  pub static ref REGISTRY: Registry = Registry::new();

  pub static ref HTTP_REQUESTS_TOTAL: IntCounter = 
    IntCounter::new("http_requests_total", "Total Amount of HTTP Requests")
    .expect("http_requests_total metric can't be initialized");

  pub static ref INCOMING_REQUESTS: IntCounterVec = IntCounterVec::new(
    Opts::new("incoming_requests", "Incoming Requests"),
    &["target"],
  ).expect("incoming_requests metric can't be initialized");

  pub static ref REFRESH_ATTEMPTS: IntCounterVec = IntCounterVec::new(
    Opts::new("refresh_attempts", "Refresh Attempts"),
    &["status"],
  ).expect("refresh_attempts metric can't be initialized");

  pub static ref LOGOUT_ATTEMPTS: IntCounterVec = IntCounterVec::new(
    Opts::new("logout_attempts", "Logout Attempts"),
    &["status"],
  ).expect("logout_attempts metric can't be initialized");
}

pub fn register_metrics() {
  REGISTRY
    .register(Box::new(HTTP_REQUESTS_TOTAL.clone()))
    .expect("http_requests_total metric can't be registered");

  REGISTRY
    .register(Box::new(INCOMING_REQUESTS.clone()))
    .expect("incoming_requests metric can't be registered");

  REGISTRY
    .register(Box::new(REFRESH_ATTEMPTS.clone()))
    .expect("refresh_attempts metric can't be registered");

  REGISTRY
    .register(Box::new(LOGOUT_ATTEMPTS.clone()))
    .expect("logout_attempts metric can't be registered");

  println!("Registered metrics successfully");
}