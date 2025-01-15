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
            id: 100089,
            height: 13,
            width: 13,
            title: "简单休闲系列".to_string(),
            author: Some(
                "闲(XIAN)".to_string(),
            ),
            map: "7111111111111710400304032171314134043317105043510401710530503014111001034311011044143041031133040430400114530504334011001304143341104053040030110000501111111111111177777".to_string(),
            best_move_moves: Some(
                265,
            ),
            best_move_pushes: Some(
                101,
            ),
            best_push_moves: Some(
                395,
            ),
            best_push_pushes: Some(
                73,
            ),
        };
        assert_eq!(
            level.xsb(),
            indoc! {"
                _############
                _# .  $ . $@#
                _#$#.#$. .$$#
                _# * .$*# . #
                _# *$ * $ #.#
                ##  # $.$## #
                # ..#.$ .# $#
                #$$ . .$ .  #
                #.*$ * .$$. #
                #  #$ .#.$$.#
                # . *$ .  $ #
                #    * ######
                ########_____
            "}
        );
    }
}
