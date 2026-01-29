use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct SkillMetadata {
    pub name: String,
    pub description: String,
}

#[derive(Debug)]
pub struct Skill {
    pub path: PathBuf,
    pub metadata: SkillMetadata,
    pub content: String,
}

pub struct ProjectContext<'a> {
    pub root: &'a PathBuf,
    pub output_file: &'a PathBuf,
}
