use crate::models::{ProjectContext, Skill};
use std::collections::HashMap;
use std::path::Path;
use walkdir::WalkDir;

pub struct Indexer;

impl Indexer {
    pub fn generate_project_index(ctx: &ProjectContext) -> String {
        let files = Self::get_all_files(ctx.root, Some(ctx.output_file));
        // Filter out .agent directory from main project index to avoid duplication
        let files: Vec<String> = files
            .into_iter()
            .filter(|f| {
                !f.starts_with(".amazonq/")
                    && !f.starts_with(".agent/")
                    && !f.starts_with(".augment/")
                    && !f.starts_with(".claude/")
                    && !f.starts_with(".cline/")
                    && !f.starts_with(".codebuddy/")
                    && !f.starts_with(".codex/")
                    && !f.starts_with(".continue/")
                    && !f.starts_with(".cospec/")
                    && !f.starts_with(".crush/")
                    && !f.starts_with(".cursor/")
                    && !f.starts_with(".factory/")
                    && !f.starts_with(".gemini/")
                    && !f.starts_with(".github/")
                    && !f.starts_with(".iflow/")
                    && !f.starts_with(".kilocode/")
                    && !f.starts_with(".opencode/")
                    && !f.starts_with(".qoder/")
                    && !f.starts_with(".qwen/")
                    && !f.starts_with(".roo/")
                    && !f.starts_with(".trae/")
                    && !f.starts_with(".windsurf/")
            })
            .collect();

        let minified_parts = Self::minify_paths(files);

        let mut lines = Vec::new();
        lines.push("[Project Index]|root: ./".to_string());
        lines.push(
            "|IMPORTANT: Prefer retrieval-led reasoning over pre-training-led reasoning"
                .to_string(),
        );

        for part in minified_parts {
            lines.push(format!("|{}", part));
        }

        format!(
            "<!-- PROJECT-INDEX-START -->\n{}\n<!-- PROJECT-INDEX-END -->",
            lines.join("\n")
        )
    }

    pub fn generate_skill_index(skill: &Skill, project_root: &Path) -> String {
        let files = Self::get_all_files(&skill.path, None);
        // Filter out SKILL.md as it is embedded directly content-wise
        let files: Vec<String> = files
            .into_iter()
            .filter(|f| !f.ends_with("SKILL.md"))
            .collect();

        if files.is_empty() {
            return String::new();
        }

        let minified_parts = Self::minify_paths(files);

        let path_str = if let Ok(rel) = skill.path.strip_prefix(project_root) {
            format!("./{}", rel.to_string_lossy().replace("\\", "/"))
        } else {
            skill.path.display().to_string().replace("\\", "/")
        };

        let mut lines = Vec::new();
        lines.push(format!(
            "[{} Index]|root: {}",
            skill.metadata.name, path_str
        ));
        lines.push(format!(
            "|IMPORTANT: Use these tools for {} tasks",
            skill.metadata.name
        ));

        for part in minified_parts {
            lines.push(format!("|{}", part));
        }

        lines.join("\n")
    }

    fn get_all_files(root: &Path, exclude_output: Option<&Path>) -> Vec<String> {
        let mut paths = Vec::new();
        // Skip hidden and build dirs
        let walker = WalkDir::new(root).into_iter().filter_entry(|e| {
            let name = e.file_name().to_str().unwrap_or("");
            if name.starts_with('.')
                && name != ".amazonq"
                && name != ".agent"
                && name != ".augment"
                && name != ".claude"
                && name != ".cline"
                && name != ".codebuddy"
                && name != ".codex"
                && name != ".continue"
                && name != ".cospec"
                && name != ".crush"
                && name != ".cursor"
                && name != ".factory"
                && name != ".gemini"
                && name != ".github"
                && name != ".iflow"
                && name != ".kilocode"
                && name != ".opencode"
                && name != ".qoder"
                && name != ".qwen"
                && name != ".roo"
                && name != ".trae"
                && name != ".windsurf"
            {
                return false;
            }
            if name == "target" || name == "node_modules" {
                return false;
            }
            true
        });

        for entry in walker.flatten() {
            if entry.file_type().is_file() {
                let path = entry.path();
                // Get relative path without canonicalization drama
                if let Ok(rel_path) = path.strip_prefix(root) {
                    if let Some(exclude) = exclude_output {
                        if rel_path == exclude {
                            continue;
                        }
                    }
                    let path_str = rel_path.to_string_lossy().replace("\\", "/");

                    paths.push(path_str);
                }
            }
        }
        paths
    }

    fn minify_paths(paths: Vec<String>) -> Vec<String> {
        let mut groups: HashMap<String, Vec<String>> = HashMap::new();

        for path in paths {
            let p = Path::new(&path);
            let parent = p
                .parent()
                .map(|p| p.to_string_lossy().into_owned())
                .filter(|s| !s.is_empty() && s != ".")
                .unwrap_or_else(|| "root".to_string())
                .replace("\\", "/");
            let file = p
                .file_name()
                .map(|f| f.to_string_lossy().into_owned())
                .unwrap_or_default();

            groups.entry(parent).or_default().push(file);
        }

        let mut parts = Vec::new();
        let mut sorted_keys: Vec<_> = groups.keys().collect();
        sorted_keys.sort();

        for key in sorted_keys {
            let mut files = groups.get(key).unwrap().clone();
            files.sort();
            if key == "root" {
                parts.push(format!("{{{}}}", files.join(",")));
            } else {
                parts.push(format!("{}:{{{}}}", key, files.join(",")));
            }
        }

        parts
    }
}
