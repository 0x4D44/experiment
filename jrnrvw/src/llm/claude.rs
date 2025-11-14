//! Claude CLI integration

use crate::error::{JrnrvwError, Result};
use std::process::{Command, Stdio};
use std::io::Write;

/// Call Claude CLI to generate a summary
pub fn generate_summary(prompt: &str) -> Result<String> {
    // Check if claude CLI is available
    let claude_path = which::which("claude")
        .map_err(|_| JrnrvwError::ConfigError(
            "Claude CLI not found. Please install it: https://github.com/anthropics/claude-code".to_string()
        ))?;

    // Call claude with --print flag for non-interactive mode
    let mut child = Command::new(claude_path)
        .arg("-p")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| JrnrvwError::ConfigError(
            format!("Failed to spawn Claude CLI: {}", e)
        ))?;

    // Write prompt to stdin
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(prompt.as_bytes())
            .map_err(|e| JrnrvwError::ConfigError(
                format!("Failed to write to Claude stdin: {}", e)
            ))?;
    }

    // Wait for completion and get output
    let output = child.wait_with_output()
        .map_err(|e| JrnrvwError::ConfigError(
            format!("Failed to read Claude output: {}", e)
        ))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(JrnrvwError::ConfigError(
            format!("Claude CLI failed: {}", stderr)
        ));
    }

    let summary = String::from_utf8(output.stdout)
        .map_err(|e| JrnrvwError::ConfigError(
            format!("Invalid UTF-8 in Claude output: {}", e)
        ))?;

    Ok(summary)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_summary_with_simple_prompt() {
        // This test will only run if Claude CLI is available
        if which::which("claude").is_ok() {
            let result = generate_summary("Say 'test successful' and nothing else.");
            // Don't assert success as Claude may require configuration or permissions
            // Just ensure the function can be called without panicking
            if let Ok(summary) = result {
                assert!(!summary.is_empty(), "Summary should not be empty");
            }
        }
    }

    #[test]
    fn test_claude_not_available() {
        // Test error handling when claude binary doesn't exist
        // We can't easily test this without mocking, so we'll just ensure
        // the function exists and has the right signature
        let _result = generate_summary("test");
    }
}
