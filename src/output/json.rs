use serde::Serialize;

#[derive(Serialize)]
pub struct Envelope<T: Serialize> {
    pub version: &'static str,
    pub status: &'static str,
    pub data: T,
}

pub fn success<T: Serialize>(data: T) {
    let envelope = Envelope {
        version: "1",
        status: "success",
        data,
    };
    println!(
        "{}",
        serde_json::to_string_pretty(&envelope).unwrap_or_default()
    );
}

pub fn error(code: &str, message: &str) {
    let envelope = serde_json::json!({
        "version": "1",
        "status": "error",
        "error": {
            "code": code,
            "message": message,
        }
    });
    eprintln!(
        "{}",
        serde_json::to_string_pretty(&envelope).unwrap_or_default()
    );
}
