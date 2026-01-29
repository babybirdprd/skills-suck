use crate::indexer::Indexer;
use crate::models::{ProjectContext, Skill, SkillMetadata};
use std::fs;
use std::path::{Path, PathBuf};

pub struct Compiler<'a> {
    ctx: ProjectContext<'a>,
    skill_dirs: Vec<&'static str>,
    start_tag: &'static str,
    end_tag: &'static str,
}

impl<'a> Compiler<'a> {
    pub fn new(root: &'a PathBuf, output: &'a PathBuf) -> Self {
        Self {
            ctx: ProjectContext {
                root,
                output_file: output,
            },
            skill_dirs: vec![
                ".agent/skills",
                ".gemini/skills",
                ".claude/skills",
                ".cursor/skills",
            ],
            start_tag: "<!-- SKILLS-COMPILER-START -->",
            end_tag: "<!-- SKILLS-COMPILER-END -->",
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let skills = self.discover_skills();
        let project_index = Indexer::generate_project_index(&self.ctx);
        let skill_content = self.aggregate_skills(&skills);

        let compiled_section = format!(
            "{}\n\n{}\n\n{}\n\n{}",
            self.start_tag, project_index, skill_content, self.end_tag
        );

        let output_path = self.ctx.root.join(self.ctx.output_file);
        let final_content = if output_path.exists() {
            let existing_content = fs::read_to_string(&output_path)?;
            if existing_content.contains(self.start_tag) && existing_content.contains(self.end_tag)
            {
                // Replace existing section
                let re = regex::Regex::new(&format!(
                    r"(?s){}\s*.*?\s*{}",
                    regex::escape(self.start_tag),
                    regex::escape(self.end_tag)
                ))?;
                re.replace(&existing_content, compiled_section.as_str())
                    .into_owned()
            } else {
                // Append to bottom
                format!("{}\n\n\n{}", existing_content.trim_end(), compiled_section)
            }
        } else {
            // Create new file
            format!(
                "# Project Agents Configuration\n\n## Overview\nThis project uses modular skills compiled into a single source of truth for AI agents.\n\n{}",
                compiled_section
            )
        };

        fs::write(output_path, final_content)?;
        println!(
            "Successfully compiled {} skills into {}",
            skills.len(),
            self.ctx.output_file.display()
        );
        Ok(())
    }

    fn discover_skills(&self) -> Vec<Skill> {
        let mut discovered = Vec::new();
        for dir_rel in &self.skill_dirs {
            let dir_path = self.ctx.root.join(dir_rel);
            if dir_path.exists() && dir_path.is_dir() {
                if let Ok(entries) = fs::read_dir(dir_path) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_dir() {
                            let skill_md = path.join("SKILL.md");
                            if skill_md.exists() {
                                if let Ok(skill) = self.parse_skill(&path) {
                                    discovered.push(skill);
                                }
                            }
                        }
                    }
                }
            }
        }
        discovered
    }

    fn parse_skill(&self, path: &Path) -> Result<Skill, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path.join("SKILL.md"))?;

        let metadata = if content.starts_with("---") {
            let parts: Vec<&str> = content.splitn(3, "---").collect();
            if parts.len() >= 3 {
                serde_yaml::from_str::<SkillMetadata>(parts[1])?
            } else {
                self.default_metadata(path)
            }
        } else {
            self.default_metadata(path)
        };

        Ok(Skill {
            path: path.to_path_buf(),
            metadata,
            content,
        })
    }

    fn default_metadata(&self, path: &Path) -> SkillMetadata {
        SkillMetadata {
            name: path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned(),
            description: "No description provided".to_string(),
        }
    }

    fn aggregate_skills(&self, skills: &[Skill]) -> String {
        let mut aggregated = String::from("## Compiled Skills\n\n");
        for skill in skills {
            let skill_index = Indexer::generate_skill_index(skill);
            aggregated.push_str(&format!(
                "### Skill: {}\n\n{}\n\n{}\n\n---\n\n",
                skill.metadata.name, skill_index, skill.content
            ));
        }
        aggregated
    }
}
