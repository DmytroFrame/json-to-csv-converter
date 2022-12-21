use iron::{status, IronResult, Plugin, Request, Response};
use serde_json::Value;

pub fn convert(req: &mut Request) -> IronResult<Response> {
    match req.get::<bodyparser::Json>() {
        Ok(Some(json)) => {
            return Ok(Response::with((status::Ok, json_to_csv(&json, ','))));
        }
        Ok(None) => {}
        Err(_) => {}
    }
    Ok(Response::with(status::Ok))
}

fn json_to_csv(json: &Value, separator: char) -> String {
    let mut text = String::new();
    if json.is_array() {
        text.push_str(&get_keys(&json[0], separator));
        for index in 0..json.as_array().unwrap().len() {
            text.push_str(&get_values(&json[index], separator));
        }
    } else {
        text.push_str(&get_keys(&json, separator));
        text.push_str(&get_values(&json, separator));
    }
    text
}

fn get_keys(json: &Value, separator: char) -> String {
    let mut keys = String::new();
    for (key, _) in json.as_object().unwrap() {
        keys.push_str(&key);
        keys.push(separator);
    }
    keys
}

fn get_values(json: &Value, separator: char) -> String {
    let mut values = String::new();
    values.push_str("\n");
    for (_, value) in json.as_object().unwrap() {
        values.push_str(&value.to_string().replace(",", ". ").replace('"', ""));
        values.push(separator);
    }
    values
}
