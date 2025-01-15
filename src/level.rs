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
    blue_moves: Option<i32>,
    blue_pushes: Option<i32>,
    green_moves: Option<i32>,
    green_pushes: Option<i32>,
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
                _ => panic!("invalid character"),
            }));
            result.push('\n');
        }
        result
    }
}
