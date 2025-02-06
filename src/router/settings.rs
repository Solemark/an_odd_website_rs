use super::common::{get_list, to_json_string, write_to_file, Helpers};
use axum::{
    body::Body,
    extract::Path,
    http::StatusCode,
    response::{Redirect, Response},
    Form,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Setting {
    pub name: String,
    pub status: bool,
}
impl Helpers for Setting {
    fn to_json(&self) -> String {
        format!(
            "{{\"name\": \"{}\",\"status\": {}}}",
            &self.name, &self.status
        )
    }
    fn to_csv(&self) -> String {
        format!("{},{}\n", &self.name, self.status)
    }
}

pub async fn setting_data_handler() -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(to_json_string(get_list(
            "settings",
            parse_setting,
        ))))
        .unwrap_or_default()
}

pub async fn setting_flag_handler(Path(name): Path<String>) -> Response {
    let data = get_list("settings", parse_setting);
    let mut check = Setting { name, status: true };
    if !data.contains(&check) {
        check.status = false;
    }
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(format!(
            "{{\"name\":\"{}\",\"status\":\"{}\"}}",
            check.name, check.status
        )))
        .unwrap_or_default()
}

pub async fn update_settings_handler(Form(set): Form<Setting>) -> Redirect {
    write_to_file(
        get_list("settings", parse_setting)
            .into_iter()
            .map(|s| {
                if s.name == set.name {
                    Setting {
                        name: s.name,
                        status: !s.status,
                    }
                } else {
                    s
                }
            })
            .collect(),
        "settings".to_string(),
    );
    Redirect::to("/settings")
}

pub fn parse_setting(s: &str) -> Setting {
    let str = s.split(',').collect::<Vec<&str>>();
    Setting {
        name: str[0].to_string(),
        status: str[1].parse::<bool>().unwrap_or_default(),
    }
}
