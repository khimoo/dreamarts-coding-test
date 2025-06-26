use longest_path_solver::{parse_input, Graph};
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct LongestPathTestCases {
    test_cases: Vec<LongestPathCase>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ExpectedPaths {
    Single { expected_path: Vec<usize> },
    Multiple { expected_paths: Vec<Vec<usize>> },
}

#[derive(Debug, Deserialize)]
struct LongestPathCase {
    name: String,
    description: String,
    input: String,
    #[serde(flatten)]
    expected: Option<ExpectedPaths>,
    expected_distance: f64,
    expected_path_length: usize,
}

#[derive(Debug, Deserialize)]
struct ErrorTestCases {
    error_cases: Vec<ErrorCase>,
}

#[derive(Debug, Deserialize)]
struct ErrorCase {
    name: String,
    description: String,
    input: String,
    expected_error: String,
}

#[test]
fn test_longest_path_cases() {
    let yaml = fs::read_to_string("tests/data/longest_path_cases.yaml").expect("YAML読み込み失敗");
    let cases: LongestPathTestCases = serde_yaml::from_str(&yaml).expect("YAMLパース失敗");
    for case in cases.test_cases {
        let graph = parse_input(&case.input).expect(&format!("parse_input失敗: {}", case.name));
        let (path, distance) = graph.find_longest_path();
        match &case.expected {
            Some(ExpectedPaths::Single { expected_path }) => {
                assert_eq!(&path, expected_path, "{}: path", case.name);
                assert_eq!(path.len(), case.expected_path_length, "{}: path length", case.name);
            }
            Some(ExpectedPaths::Multiple { expected_paths }) => {
                assert!(expected_paths.iter().any(|p| p == &path), "{}: path {:?} がどの expected_paths にも一致しません", case.name, path);
                let allowed_lengths: Vec<_> = expected_paths.iter().map(|p| p.len()).collect();
                assert!(allowed_lengths.contains(&path.len()), "{}: path length {} が許容されていません（許容長さ: {:?}）", case.name, path.len(), allowed_lengths);
            }
            None => {}
        }
        assert!((distance - case.expected_distance).abs() < 1e-8, "{}: distance", case.name);
    }
}

#[test]
fn test_error_cases() {
    let yaml = fs::read_to_string("tests/data/error_cases.yaml").expect("YAML読み込み失敗");
    let cases: ErrorTestCases = serde_yaml::from_str(&yaml).expect("YAMLパース失敗");
    for case in cases.error_cases {
        let result = parse_input(&case.input);
        assert!(result.is_err(), "{}: エラーであるべき", case.name);
        let err_msg = format!("{}", result.unwrap_err());
        assert!(err_msg.contains(&case.expected_error), "{}: エラーメッセージ不一致: got '{}', want contains '{}'", case.name, err_msg, case.expected_error);
    }
} 