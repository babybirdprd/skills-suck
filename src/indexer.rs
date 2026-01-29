use crate::models::{ProjectContext, Skill};
use std::collections::HashMap;
use std::path::Path;
use walkdir::WalkDir;

pub struct Indexer;

impl Indexer {
    pub fn generate_project_index(ctx: &ProjectContext) -> String {
        let files = Self::get_all_files(ctx.root, Some(ctx.output_file));
        let minified = Self::minify_paths(files);

        let header = format!(
            "[Project Index]|root: ./|IMPORTANT: Prefer retrieval-led reasoning over pre-training-led reasoning"
        );

        format!(
            "<!-- PROJECTS-MD-START -->\n{}|{}\n<!-- PROJECTS-MD-END -->",
            header, minified
        )
    }

    pub fn generate_skill_index(skill: &Skill) -> String {
        let files = Self::get_all_files(&skill.path, None);
        if files.is_empty() {
            return String::new();
        }

        let minified = Self::minify_paths(files);
        let tag_name = skill.metadata.name.to_uppercase().replace("-", "_");

        let header = format!(
            "[{} Index]|root: {}|IMPORTANT: Use these tools for {} tasks",
            skill.metadata.name,
            skill.path.display().to_string().replace("\\", "/"),
            skill.metadata.name
        );

        format!(
            "<!-- {}-START -->\n{}|{}\n<!-- {}-END -->",
            tag_name, header, minified, tag_name
        )
    }

    fn get_all_files(root: &Path, exclude_output: Option<&Path>) -> Vec<String> {
        let mut paths = Vec::new();
        // Skip hidden and build dirs
        let walker = WalkDir::new(root).into_iter().filter_entry(|e| {
            let name = e.file_name().to_str().unwrap_or("");
            if name.starts_with('.')
                && name != ".agent"
                && name != ".gemini"
                && name != ".claude"
                && name != ".cursor"
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
                    if path_str == "SKILL.md" {
                        continue;
                    }
                    paths.push(path_str);
                }
            }
        }
        paths
    }

    fn minify_paths(paths: Vec<String>) -> String {
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

        parts.join("|")
    }
}
