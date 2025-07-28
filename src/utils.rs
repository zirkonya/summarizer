/*
fn get_files(path: PathBuf, depth: u8) -> Vec<PathBuf> {
    if depth == 0 {
        return Vec::new();
    }
    if path.is_file() {
        return vec![path];
    } else {
        let mut files = Vec::new();
        let path = &path;
        if let Ok(read_dir) = path.read_dir() {
            for entry in read_dir {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    files.extend(get_files(path, depth - 1));
                }
            }
        }
        files
    }
}
*/

use std::{collections::VecDeque, fs, path::PathBuf};

pub fn get_files(root: PathBuf, max_depth: u8) -> Vec<PathBuf> {
    let mut result = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back((root, 0u8)); // (path, current_depth)

    while let Some((path, depth)) = queue.pop_front() {
        if depth >= max_depth {
            continue;
        }

        match fs::metadata(&path) {
            Ok(metadata) => {
                if metadata.is_file() {
                    result.push(path);
                } else if metadata.is_dir() {
                    if let Ok(entries) = fs::read_dir(&path) {
                        for entry in entries.flatten() {
                            queue.push_back((entry.path(), depth + 1));
                        }
                    }
                }
            }
            Err(_) => continue,
        }
    }

    result
}

/*
fn remove_think(src: String) -> String {
    let parts: Vec<&str> = src.split("<think>").collect();
    if parts.len() == 1 {
        return src;
    } else {
        parts[0].to_string()
            + &parts[1..]
                .iter()
                .map(|p| p.split("</think>").nth(1).unwrap_or(&p))
                .collect::<Vec<&str>>()
                .join("")
                .trim()
    }
}*/

pub fn remove_think(src: &str) -> String {
    let mut result = String::with_capacity(src.len());
    let mut chars = src.char_indices().peekable();
    let mut in_think = false;
    let mut i;

    while let Some((idx, _)) = chars.peek().copied() {
        if !in_think && src[idx..].starts_with("<think>") {
            in_think = true;
            i = idx + "<think>".len();
            for (next_idx, _) in chars.by_ref() {
                if next_idx >= i {
                    break;
                }
            }
        } else if in_think && src[idx..].starts_with("</think>") {
            in_think = false;
            i = idx + "</think>".len();
            for (next_idx, _) in chars.by_ref() {
                if next_idx >= i {
                    break;
                }
            }
        } else if !in_think {
            result.push(src.as_bytes()[idx] as char);
            chars.next();
        } else {
            chars.next(); // skip characters inside <think>...</think>
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use crate::utils::{get_files, remove_think};

    #[test]
    fn test_remove_think() {
        let with_think = include_str!("tests/message_with_think.txt");
        let without_think = include_str!("tests/message_without_think.txt");
        assert_eq!(remove_think(with_think).trim(), without_think.trim())
    }

    #[test]
    fn test_get_files() {
        let file_name = vec![
            "main.rs",
            "summarizer.rs",
            "synthesizer.rs",
            "utils.rs",
            "message_with_think.txt",
            "message_without_think.txt",
        ];
        let files = get_files(PathBuf::from("."), 2);
    }
}
