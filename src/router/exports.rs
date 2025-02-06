use super::common::{get_list, to_json_string, Helpers};
use axum::{body::Body, http::StatusCode, response::Response};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Export {
    id: usize,
    name: String,
}
impl Helpers for Export {
    fn to_json(&self) -> String {
        format!(
            "{{
                \"id\": \"{}\",
                \"name\": \"{}\"
            }}",
            &self.id, &self.name
        )
    }
    fn to_csv(&self) -> String {
        format!("{},{}\n", &self.id, &self.name)
    }
}

pub async fn get_exports_handler() -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(to_json_string(get_list(
            "exports",
            parse_exports,
        ))))
        .unwrap_or_default()
}

fn parse_exports(s: &str) -> Export {
    let e = s.split(',').collect::<Vec<&str>>();
    Export {
        id: e[0].parse().unwrap_or_default(),
        name: e[1].to_string(),
    }
}
