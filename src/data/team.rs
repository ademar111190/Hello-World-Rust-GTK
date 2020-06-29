use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Team {
    name: String,
    titles: u8,
    runner_ups: u8,
    title_years: Vec<u16>,
    runner_up_years: Vec<u16>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct TeamsPayload {
    teams: Vec<Team>
}

pub fn build_teams_payload(json: &str) -> TeamsPayload {
    serde_json::from_str(json)
        .expect(&format!("TeamsPayload cannot be build with the given json: {}", json))
}

#[cfg(test)]
mod test {
    use crate::data::team::{build_teams_payload, Team, TeamsPayload};

    #[test]
    fn json_complete() {
        assert_eq!(
            build_teams_payload(r#"{
                "teams": [{
                    "name": "A team",
                    "titles": 1,
                    "runner_ups": 2,
                    "title_years": [1990],
                    "runner_up_years": [1888, 2020]
                }]
            }"#),
            TeamsPayload {
                teams: vec![
                    Team {
                        name: String::from("A team"),
                        titles: 1,
                        runner_ups: 2,
                        title_years: vec![1990],
                        runner_up_years: vec![1888, 2020],
                    }
                ]
            }
        );
    }

    #[test]
    fn json_empty_arrays() {
        assert_eq!(
            build_teams_payload(r#"{
                "teams": [{
                    "name": "A team",
                    "titles": 1,
                    "runner_ups": 2,
                    "title_years": [],
                    "runner_up_years": []
                }]
            }"#),
            TeamsPayload {
                teams: vec![
                    Team {
                        name: String::from("A team"),
                        titles: 1,
                        runner_ups: 2,
                        title_years: vec![],
                        runner_up_years: vec![],
                    }
                ]
            }
        );
    }

    #[test]
    #[should_panic]
    fn json_missing_name() {
        build_teams_payload(r#"{
            "teams": [{
                "titles": 1,
                "runner_ups": 2,
                "title_years": [1990],
                "runner_up_years": [1888, 2020]
            }]
        }"#);
    }

    #[test]
    #[should_panic]
    fn json_missing_titles() {
        build_teams_payload(r#"{
            "teams": [{
                "name": "A team",
                "runner_ups": 2,
                "title_years": [1990],
                "runner_up_years": [1888, 2020]
            }]
        }"#);
    }

    #[test]
    #[should_panic]
    fn json_missing_runner_ups() {
        build_teams_payload(r#"{
            "teams": [{
                "name": "A team",
                "titles": 1,
                "title_years": [1990],
                "runner_up_years": [1888, 2020]
            }]
        }"#);
    }

    #[test]
    #[should_panic]
    fn json_missing_title_years() {
        build_teams_payload(r#"{
            "teams": [{
                "name": "A team",
                "titles": 1,
                "runner_ups": 2,
                "runner_up_years": [1888, 2020]
            }]
        }"#);
    }

    #[test]
    #[should_panic]
    fn json_missing_runner_up_years() {
        build_teams_payload(r#"{
            "teams": [{
                "name": "A team",
                "titles": 1,
                "runner_ups": 2,
                "title_years": [1990]
            }]
        }"#);
    }

    #[test]
    fn json_empty_teams() {
        assert_eq!(
            build_teams_payload(r#"{
                "teams": []
            }"#),
            TeamsPayload {
                teams: Vec::new()
            }
        );
    }

    #[test]
    #[should_panic]
    fn json_invalid() {
        build_teams_payload(r#"{}"#);
    }
}
