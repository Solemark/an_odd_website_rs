use super::common::{get_list, to_json_string, write_to_file, Helpers};
use axum::{
    body::Body,
    extract::Form,
    http::StatusCode,
    response::{Redirect, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Client {
    id: usize,
    first_name: String,
    last_name: String,
    email: String,
    visible: bool,
}
impl Helpers for Client {
    fn to_json(&self) -> String {
        format!(
            "{{\"id\": \"{}\",\"first_name\": \"{}\",\"last_name\": \"{}\",\"email\": \"{}\",\"visible\": \"{}\"}}",
            &self.id, &self.first_name, &self.last_name, &self.email, &self.visible
        )
    }

    fn to_csv(&self) -> String {
        format!(
            "{},{},{},{},{}\n",
            &self.id, &self.first_name, &self.last_name, &self.email, &self.visible
        )
    }
}

pub async fn client_data_handler() -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(to_json_string(
            get_list("clients", parse_client)
                .into_iter()
                .filter(|c| c.visible)
                .collect(),
        )))
        .unwrap_or_default()
}

pub async fn new_client_handler(Form(cli): Form<Client>) -> Redirect {
    let client_list = get_list("clients", parse_client);
    let cll = client_list.len();
    write_to_file(
        client_list
            .into_iter()
            .chain(vec![Client {
                id: cll,
                first_name: cli.first_name,
                last_name: cli.last_name,
                email: cli.email,
                visible: true,
            }])
            .collect(),
        "clients".to_string(),
    );
    Redirect::to("/clients")
}

pub async fn remove_client_handler(Form(cli): Form<Client>) -> Redirect {
    write_to_file(
        get_list("clients", parse_client)
            .into_iter()
            .map(|c| {
                if c.id == cli.id {
                    Client {
                        id: c.id,
                        first_name: c.first_name,
                        last_name: c.last_name,
                        email: c.email,
                        visible: false,
                    }
                } else {
                    c
                }
            })
            .collect(),
        "clients".to_string(),
    );
    Redirect::to("/clients")
}

pub async fn update_client_handler(Form(cli): Form<Client>) -> Redirect {
    write_to_file(
        get_list("clients", parse_client)
            .into_iter()
            .map(|c| if c.id == cli.id { cli.clone() } else { c })
            .collect(),
        "clients".to_string(),
    );
    Redirect::to("/clients")
}

fn parse_client(s: &str) -> Client {
    let c: Vec<&str> = s.split(',').collect();
    Client {
        id: c[0].parse::<usize>().unwrap_or_default(),
        first_name: String::from(c[1]),
        last_name: String::from(c[2]),
        email: String::from(c[3]),
        visible: c[4].parse::<bool>().unwrap_or_default(),
    }
}
