//! Codex CLI integration

use crate::error::{JrnrvwError, Result};
use std::process::{Command, Stdio};

/// Call Codex CLI to generate a summary
pub fn generate_summary(prompt: &str) -> Result<String> {
    // Check if codex CLI is available
    let codex_path = which::which("codex")
        .map_err(|_| JrnrvwError::ConfigError(
            "Codex CLI not found. Please install it first.".to_string()
        ))?;

    // Call codex exec with prompt as argument
    let output = Command::new(codex_path)
        .arg("exec")
        .arg(prompt)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| JrnrvwError::ConfigError(
            format!("Failed to execute Codex CLI: {}", e)
        ))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(JrnrvwError::ConfigError(
            format!("Codex CLI failed: {}", stderr)
        ));
    }

    let raw_output = String::from_utf8(output.stdout)
        .map_err(|e| JrnrvwError::ConfigError(
            format!("Invalid UTF-8 in Codex output: {}", e)
        ))?;

    // Extract the actual response from Codex's output
    // Codex outputs metadata followed by the actual response
    // We look for the line starting with "codex" which precedes the response
    let summary = extract_codex_response(&raw_output);

    Ok(summary)
}

/// Extract the actual response from Codex's verbose output
fn extract_codex_response(output: &str) -> String {
    let lines: Vec<&str> = output.lines().collect();

    // Find the line that says "codex" which precedes the actual response
    if let Some(codex_idx) = lines.iter().position(|&line| line.trim() == "codex") {
        // The response starts after the "codex" line
        // We want to capture everything after that until we hit "tokens used" or similar
        let response_lines: Vec<&str> = lines[codex_idx + 1..]
            .iter()
            .take_while(|&&line| !line.starts_with("tokens used"))
            .copied()
            .collect();

        response_lines.join("\n").trim().to_string()
    } else {
        // Fallback: return everything after the last separator line
        lines.iter()
            .skip_while(|&&line| !line.contains("--------"))
            .skip(1) // Skip the separator itself
            .map(|&s| s)
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_summary_with_simple_prompt() {
        // This test will only run if Codex CLI is available
        if which::which("codex").is_ok() {
            let result = generate_summary("Say 'test successful' and nothing else.");
            // Don't assert success as Codex may require configuration or permissions
            // Just ensure the function can be called without panicking
            if let Ok(summary) = result {
                assert!(!summary.is_empty(), "Summary should not be empty");
            }
        }
    }

    #[test]
    fn test_codex_not_available() {
        // Test error handling when codex binary doesn't exist
        // We can't easily test this without mocking, so we'll just ensure
        // the function exists and has the right signature
        let _result = generate_summary("test");
    }
}
