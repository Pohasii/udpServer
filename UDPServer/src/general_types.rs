use std::sync::{Arc, Mutex};
use crate::client;

pub type ConnectionType = Arc<Mutex<Vec<client::Client>>>;
