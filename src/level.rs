use serde::{Deserialize, Serialize};

use crate::{submit_solution, SubmitSolutionError, SubmitSolutionResult};

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct Level {
    id: i32,
    height: i32,
    width: i32,
    title: String,
    author: Option<String>,
    map: String,
    #[serde(rename = "blue_moves")]
    best_move_moves: Option<i32>,
    #[serde(rename = "blue_pushes")]
    best_move_pushes: Option<i32>,
    #[serde(rename = "green_moves")]
    best_push_moves: Option<i32>,
    #[serde(rename = "green_pushes")]
    best_push_pushes: Option<i32>,
}

impl Level {
    pub async fn submit_solution(
        &self,
        api_key: &str,
        solution: &str,
    ) -> Result<SubmitSolutionResult, SubmitSolutionError> {
        submit_solution(api_key, self.id, solution).await
    }

    pub fn xsb(&self) -> String {
        assert!(self.map.is_ascii());

        let mut result = String::new();
        for line in self.map.as_bytes().chunks(self.width as usize) {
            result.extend(line.iter().map(|c| match c {
                b'0' => ' ',
                b'1' => '#',
                b'2' => '@',
                b'3' => '$',
                b'4' => '.',
                b'5' => '*',
                b'6' => '+',
                b'7' => '_',
                _ => panic!("invalid character '{}'", *c as char),
            }));
            result.push('\n');
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn xsb() {
        let level = Level {
            id: 681,
            height: 9,
            width: 9,
            title: "Jason's Cave".to_string(),
            author: Some("Blizzard".to_string()),
            map:
                "111111111100000001100230301111111001104441301104111001100000301100000001111111111"
                    .to_string(),
            best_move_moves: Some(105),
            best_move_pushes: Some(38),
            best_push_moves: Some(105),
            best_push_pushes: Some(38),
        };
        assert_eq!(
            level.xsb(),
            indoc! {"
            #########
            #       #
            #  @$ $ #
            ######  #
            # ...#$ #
            # .###  #
            #     $ #
            #       #
            #########
        "}
        );
    }
}
