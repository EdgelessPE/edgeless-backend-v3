use serde_json::Value;
use std::fs;

pub fn get_json(path: String) -> Result<Value, String> {
    let text_res = fs::read_to_string(&path);
    if let Err(e) = text_res {
        return Err(format!("Can't read file {} to string : {}", &path, e));
    }
    let text = &text_res.unwrap();
    let parse_res: serde_json::Result<Value> = serde_json::from_str(text);
    if let Err(e) = parse_res {
        return Err(format!("Can't parse file {} as json : {}", &path, e));
    }
    Ok(parse_res.unwrap())
}
