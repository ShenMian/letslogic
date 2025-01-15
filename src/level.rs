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
        let mut result =
            String::with_capacity(self.map.len() + self.map.len() / self.width as usize);
        for (i, c) in self.map.chars().enumerate() {
            result.push(match c {
                '0' => ' ',
                '1' => '#',
                '2' => '@',
                '3' => '$',
                '4' => '.',
                '5' => '*',
                '6' => '+',
                '7' => '_',
                _ => panic!("invalid character"),
            });
            if (i + 1) % self.width as usize == 0 {
                result.push('\n');
            }
        }
        result
    }
}
