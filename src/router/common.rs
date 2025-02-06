use std::{
    fs::{read_to_string, File},
    io::Write,
};

pub trait Helpers {
    fn to_json(&self) -> String;
    fn to_csv(&self) -> String;
}

pub fn to_json_string<T: Helpers>(arr: Vec<T>) -> String {
    let mut output = String::new();
    for item in arr {
        output += &format!("{},", item.to_json());
    }
    if output.len() > 0 {
        format!("[{}]", &output[0..output.len() - 1])
    } else {
        "{}".to_string()
    }
}

pub fn get_list<T: Helpers>(name: &str, parse: fn(&str) -> T) -> Vec<T> {
    read_to_string(format!("data/{name}.csv"))
        .unwrap_or_default()
        .lines()
        .map(parse)
        .collect()
}

pub fn write_to_file<T: Helpers>(arr: Vec<T>, filename: String) {
    Write::write_all(
        &mut File::create(format!("data/{}.csv", filename))
            .unwrap_or_else(|_| panic!("unable to write file {}.csv", filename)),
        arr.into_iter()
            .map(|item| item.to_csv())
            .collect::<String>()
            .as_bytes(),
    )
    .unwrap_or_default()
}
