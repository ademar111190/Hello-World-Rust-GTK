use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Year {
    year: u16,
    champion: String,
    runner_up: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct YearsPayload {
    years: Vec<Year>
}

pub fn build_years_payload(json: &str) -> YearsPayload {
    serde_json::from_str(json)
        .expect(&format!("YearsPayload cannot be build with the given json: {}", json))
}

#[cfg(test)]
mod test {
    use crate::year::{build_years_payload, Year, YearsPayload};

    #[test]
    fn json_complete() {
        assert_eq!(
            build_years_payload(r#"{
                "years": [{
                    "year": 1902,
                    "champion": "São Paulo Athletic",
                    "runner_up": "Paulistano"
                }]
            }"#),
            YearsPayload {
                years: vec![
                    Year {
                        year: 1902,
                        champion: String::from("São Paulo Athletic"),
                        runner_up: String::from("Paulistano"),
                    }
                ]
            }
        );
    }

    #[test]
    #[should_panic]
    fn json_missing_year() {
        build_years_payload(r#"{
            "years": [{
                "champion": "São Paulo Athletic",
                "runner_up": "Paulistano"
            }]
        }"#);
    }

    #[test]
    #[should_panic]
    fn json_missing_champion() {
        build_years_payload(r#"{
            "years": [{
                "year": 1902,
                "runner_up": "Paulistano"
            }]
        }"#);
    }

    #[test]
    #[should_panic]
    fn json_missing_runner_up() {
        build_years_payload(r#"{
            "years": [{
                "year": 1902,
                "champion": "São Paulo Athletic"
            }]
        }"#);
    }

    #[test]
    fn json_empty_years() {
        assert_eq!(
            build_years_payload(r#"{
                "years": []
            }"#),
            YearsPayload {
                years: Vec::new()
            }
        );
    }

    #[test]
    #[should_panic]
    fn json_invalid() {
        build_years_payload(r#"{}"#);
    }
}
