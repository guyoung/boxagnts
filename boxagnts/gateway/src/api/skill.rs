use std::collections::HashMap;

use serde::{ Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillConfig {
    pub name: String,
    pub description: String,
}


pub async fn load_skills() -> anyhow::Result<HashMap<String, SkillConfig>> {
    let cwd = boxagnts_workspace::path::get_workspace_dir().await;

    let mut dirs = vec![
        cwd.join("extensions").join("skills"),
    ];
    dirs.push(boxagnts_workspace::path::get_app_extensions_dir().await.join("skills"));

    let mut skills: HashMap<String, SkillConfig> = HashMap::new();


    for dir in dirs {
        match tokio::fs::read_dir(dir).await {
            Ok(mut entries) => {
                while let Ok(Some(entry)) = entries.next_entry().await {
                    let path = entry.path();
                    if path.is_dir() {
                        if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                            let skill_path = path.join("SKILL.md");

                            if skill_path.exists() && skill_path.is_file()
                                && !skills.contains_key(dir_name) {
                                let description = read_skill_description(&skill_path).await;

                                let skill_cfg = SkillConfig {
                                    name: dir_name.to_string(),
                                    description,
                                };

                                skills.insert(dir_name.to_string(), skill_cfg);
                            }
                        }

                    }
                }
            }
            Err(_) => {} // directory doesn't exist, skip
        }
    }


    Ok(skills)
}

async fn read_skill_description(path: &std::path::Path) -> String {
    let Ok(content) = tokio::fs::read_to_string(path).await else {
        return "(no description)".to_string();
    };
    let body = strip_frontmatter(&content);
    // First non-empty, non-heading line
    for line in body.lines() {
        let t = line.trim().trim_start_matches('#').trim();
        if !t.is_empty() {
            let truncated = if t.len() > 80 { &t[..80] } else { t };
            return truncated.to_string();
        }
    }
    "(no description)".to_string()
}

/// Remove YAML frontmatter delimited by `---` at the start of the file.
fn strip_frontmatter(content: &str) -> String {
    if content.starts_with("---") {
        // Find closing ---
        let after_open = &content[3..];
        if let Some(close_pos) = after_open.find("\n---") {
            // Skip past the closing delimiter and any leading newline
            let rest = &after_open[close_pos + 4..];
            return rest.trim_start_matches('\n').to_string();
        }
    }
    content.to_string()
}
