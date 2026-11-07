use serein_extension_sdk::{Element, Invocation, Output};

pub fn unindent_block(text: &str) -> String {
    let text = text.replace('\t', "    ");
    let lines: Vec<&str> = text.lines().collect();
    if lines.is_empty() {
        return text;
    }

    // Find minimum indentation across non-empty lines
    let mut min_indent = usize::MAX;
    for line in &lines {
        let trimmed = line.trim_start();
        if !trimmed.is_empty() {
            let indent = line.len() - trimmed.len();
            if indent < min_indent {
                min_indent = indent;
            }
        }
    }

    if min_indent == 0 || min_indent == usize::MAX {
        return text;
    }

    lines
        .into_iter()
        .map(|line| {
            if line.len() >= min_indent {
                &line[min_indent..]
            } else {
                ""
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn unindent_message(content: &str) -> (String, usize) {
    if !content.contains("```") {
        let unindented = unindent_block(content);
        let changed = if unindented != content { 1 } else { 0 };
        return (unindented, changed);
    }

    let mut result = String::with_capacity(content.len());
    let mut changed_count = 0;
    let mut last_idx = 0;

    let marker = "```";
    while let Some(start) = content[last_idx..].find(marker) {
        let block_start = last_idx + start;
        result.push_str(&content[last_idx..block_start]);

        // Find matching closing ```
        let after_opening = block_start + marker.len();
        if let Some(end) = content[after_opening..].find(marker) {
            let block_end = after_opening + end;
            let full_code = &content[after_opening..block_end];

            // Split first line (lang tag)
            if let Some((first_line, rest)) = full_code.split_once('\n') {
                let unindented_code = unindent_block(rest);
                result.push_str("```");
                result.push_str(first_line);
                result.push('\n');
                result.push_str(&unindented_code);
                if !unindented_code.ends_with('\n') {
                    result.push('\n');
                }
                result.push_str("```");
                changed_count += 1;
            } else {
                // Inline or single line block
                result.push_str(&content[block_start..block_end + marker.len()]);
            }

            last_idx = block_end + marker.len();
        } else {
            // Unclosed code block
            result.push_str(&content[block_start..]);
            last_idx = content.len();
            break;
        }
    }

    if last_idx < content.len() {
        result.push_str(&content[last_idx..]);
    }

    (result, changed_count)
}

pub fn handle(input: Invocation) -> Output {
    if input.action != "unindent-draft" {
        return Output::default();
    }

    let draft = input.composer.as_deref().unwrap_or("");
    let (unindented, count) = unindent_message(draft);

    if unindented != draft {
        Output {
            replacement: Some(unindented),
            panel: vec![Element::Text {
                text: format!("Отступы успешно выровнены (обработано блоков: {count})."),
            }],
            ..Default::default()
        }
    } else {
        Output {
            panel: vec![Element::Text {
                text: "В драфте нет лишних отступов для удаления.".into(),
            }],
            ..Default::default()
        }
    }
}

serein_extension_sdk::export!(handle);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unindent_plain_text() {
        let input = "    fn main() {\n        println!(\"hello\");\n    }";
        let expected = "fn main() {\n    println!(\"hello\");\n}";
        assert_eq!(unindent_block(input), expected);
    }

    #[test]
    fn test_unindent_codeblock_in_message() {
        let input = "Here is my code:\n```rust\n        let x = 10;\n        let y = 20;\n```\nDone.";
        let (output, count) = unindent_message(input);
        assert_eq!(count, 1);
        let expected = "Here is my code:\n```rust\nlet x = 10;\nlet y = 20;\n```\nDone.";
        assert_eq!(output, expected);
    }
}
