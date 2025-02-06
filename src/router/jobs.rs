use super::common::{get_list, to_json_string, write_to_file, Helpers};
use axum::{
    body::Body,
    extract::Form,
    http::StatusCode,
    response::{Redirect, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Job {
    id: usize,
    name: String,
    created: String,
    status: String,
}
impl Helpers for Job {
    fn to_json(&self) -> String {
        format!(
            "{{\"id\": \"{}\",\"name\": \"{}\",\"created\": \"{}\",\"status\": \"{}\"}}",
            &self.id, &self.name, &self.created, &self.status
        )
    }
    fn to_csv(&self) -> String {
        format!(
            "{},{},{},{}\n",
            &self.id, &self.name, &self.created, &self.status
        )
    }
}

pub async fn get_jobs_handler() -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(to_json_string(get_list("jobs", parse_job))))
        .unwrap_or_default()
}

pub async fn new_jobs_handler(Form(job): Form<Job>) -> Redirect {
    let jobs_list = get_list("jobs", parse_job);
    let jll = jobs_list.len();
    write_to_file(
        jobs_list
            .clone()
            .into_iter()
            .chain(vec![Job {
                id: jll,
                name: job.name,
                created: job.created,
                status: job.status,
            }])
            .collect(),
        "jobs".to_string(),
    );
    Redirect::to("/accounting")
}

fn parse_job(s: &str) -> Job {
    let j: Vec<&str> = s.split(',').collect();
    Job {
        id: j[0].parse::<usize>().unwrap_or_default(),
        name: String::from(j[1]),
        created: String::from(j[2]),
        status: String::from(j[3]),
    }
}
