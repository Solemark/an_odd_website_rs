use super::settings::get_settings_list;
use axum::{body::Body, extract::Path, http::StatusCode, response::Response};
use std::fs::read_to_string;

pub async fn index_handler() -> Response {
    webpage_builder("static/index.html", "dashboard").await
}

pub async fn webpage_handler(Path(page): Path<String>) -> Response {
    webpage_builder(&format!("static/{page}.html"), &page).await
}

async fn webpage_builder(path: &str, page: &str) -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(Body::from(format!(
            "{}{}",
            get_page_head(&page),
            get_file(&path)
                .await
                .replace("<!--NAVBAR-->", &get_navbar(page))
        )))
        .unwrap_or_default()
}

pub async fn style_handler(Path(style): Path<String>) -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/css")
        .body(Body::from(
            get_file(&format!("static/styles/{style}.css")).await,
        ))
        .unwrap_or_default()
}

pub async fn script_handler(Path(script): Path<String>) -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/javascript")
        .body(Body::from(
            get_file(&format!("static/scripts/{script}.js")).await,
        ))
        .unwrap_or_default()
}

async fn get_file(path: &str) -> String {
    read_to_string(path).unwrap_or_else(|e| format!("{}", e))
}

fn get_navbar(page: &str) -> String {
    let page = {
        if page == "dashboard" {
            ""
        } else {
            page
        }
    };
    let mut navbar = read_to_string("static/components/navbar.html")
        .unwrap_or_default()
        .replace(
            &format!("<a href=\"/{}\">", page),
            &format!("<a class=\"active\" href=\"/{}\">", page),
        );
    for setting in get_settings_list().into_iter().filter(|s| !s.status) {
        navbar = match setting.name.as_str() {
            "enable-clients" => navbar.replace("<a href=\"/clients\">Clients</a>", ""),
            "enable-employees" => navbar.replace("<a href=\"/employees\">Employees</a>", ""),
            "enable-exporters" => navbar.replace("<a href=\"/accounting\">Accounting</a>", ""),
            _ => navbar,
        }
    }
    navbar
}

fn get_page_head(page: &str) -> String {
    let head = read_to_string("static/components/head.html")
        .unwrap_or_default()
        .replace("PAGENAME", page);
    match page {
        "clients" => head.replace(
            "<!--SCRIPT-->",
            "<script defer src=\"/scripts/clients\"></script>",
        ),
        "employees" => head.replace(
            "<!--SCRIPT-->",
            "<script defer src=\"/scripts/employees\"></script>",
        ),
        "accounting" => head.replace(
            "<!--SCRIPT-->",
            "<script defer src=\"scripts/accounting\"></script>",
        ),
        "settings" => head.replace(
            "<!--SCRIPT-->",
            "<script defer src=\"scripts/settings\"></script>",
        ),
        _ => head.replace(
            "<!--SCRIPT-->",
            "<script defer src=\"scripts/dashboard\"></script>",
        ),
    }
}
