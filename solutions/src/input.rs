use anyhow::Result;
use std::fmt::Debug;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

pub struct AdventInput {
    file: PathBuf,
}
impl AdventInput {
    pub fn for_day(day: u8) -> AdventInput {
        let file = PathBuf::from(format!("inputs/day{:02}.txt", day));
        AdventInput { file }
    }

    pub fn get_lines(&self) -> Result<Vec<String>> {
        Ok(fs::read_to_string(&self.file)?
            .lines()
            .map(|l| l.to_owned())
            .collect())
    }

    pub fn get_lines_as<T>(&self) -> Result<Vec<T>>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        let read = fs::read_to_string(&self.file)?;
        Ok(read.lines().map(|x| x.parse::<T>().unwrap()).collect())
    }
}