use super::common::{get_list, to_json_string, write_to_file, Helpers};
use axum::{
    body::Body,
    extract::Form,
    http::StatusCode,
    response::{Redirect, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Employee {
    id: usize,
    first_name: String,
    last_name: String,
    email: String,
    role: String,
    visible: bool,
}
impl Helpers for Employee {
    fn to_json(&self) -> String {
        format!(
            "{{\"id\": \"{}\",\"first_name\": \"{}\",\"last_name\": \"{}\",\"email\": \"{}\",\"role\": \"{}\",\"visible\": \"{}\"}}",
            &self.id, &self.first_name, &self.last_name, &self.email, &self.role, &self.visible
        )
    }
    fn to_csv(&self) -> String {
        format!(
            "{},{},{},{},{},{}\n",
            &self.id, &self.first_name, &self.last_name, &self.email, &self.role, &self.visible
        )
    }
}

pub async fn employee_data_handler() -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(to_json_string(
            get_list("employees", parse_employee)
                .into_iter()
                .filter(|e| e.visible)
                .collect(),
        )))
        .unwrap_or_default()
}

pub async fn new_employee_handler(Form(emp): Form<Employee>) -> Redirect {
    let employee_list = get_list("employees", parse_employee);
    let ell = employee_list.len();
    write_to_file(
        employee_list
            .into_iter()
            .chain(vec![Employee {
                id: ell,
                first_name: emp.first_name,
                last_name: emp.last_name,
                email: emp.email,
                role: emp.role,
                visible: true,
            }])
            .collect(),
        "employees".to_string(),
    );
    Redirect::to("/employees")
}

pub async fn remove_employee_handler(Form(emp): Form<Employee>) -> Redirect {
    write_to_file(
        get_list("employees", parse_employee)
            .into_iter()
            .map(|e| {
                if e.id == emp.id {
                    Employee {
                        id: e.id,
                        first_name: e.first_name,
                        last_name: e.last_name,
                        email: e.email,
                        role: e.role,
                        visible: false,
                    }
                } else {
                    e
                }
            })
            .collect(),
        "employees".to_string(),
    );
    Redirect::to("/employees")
}

pub async fn update_employee_handler(Form(emp): Form<Employee>) -> Redirect {
    write_to_file(
        get_list("employees", parse_employee)
            .into_iter()
            .map(|e| if e.id == emp.id { emp.clone() } else { e })
            .collect(),
        "employees".to_string(),
    );
    Redirect::to("/employees")
}

fn parse_employee(s: &str) -> Employee {
    let e = s.split(',').collect::<Vec<&str>>();
    Employee {
        id: e[0].parse::<usize>().unwrap_or_default(),
        first_name: String::from(e[1]),
        last_name: String::from(e[1]),
        email: String::from(e[3]),
        role: String::from(e[4]),
        visible: e[5].parse::<bool>().unwrap_or_default(),
    }
}
