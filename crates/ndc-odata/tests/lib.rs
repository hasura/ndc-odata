use serde::Deserialize;
use std::path;

#[derive(Deserialize)]
struct Test {
    endpoint: String,
    method: Method,
    data: serde_json::Value,
}

#[derive(Deserialize)]
enum Method {
    GET,
    POST,
}

#[test_each::path(glob = "crates/ndc-odata/tests/**/*.json")]
fn test_snapshot(path: path::PathBuf) {
    let client = reqwest::blocking::Client::new();
    let content = std::fs::read_to_string(&path).unwrap();
    let test: Test = serde_json::from_str(&content).unwrap();

    let request = match test.method {
        Method::GET => client.get(test.endpoint),
        Method::POST => client.post(test.endpoint).json(&test.data),
    };

    // The path to the root of the `ndc-odata` crate.
    // Removing this means that we have a name that is consistent across platforms.
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    insta::with_settings!({
        input_file => &path,
        snapshot_path => root.join("tests/snapshots"),
    }, {
        insta::assert_json_snapshot!(
            path.strip_prefix(root).unwrap().to_str().unwrap(),
            request.send().unwrap().json::<serde_json::Value>().unwrap()
        )
    });
}
