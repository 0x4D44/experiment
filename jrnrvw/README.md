# jrnrvw - Journal Review Tool

A command-line tool for finding and analyzing task journal files written in Rust.

## Overview

`jrnrvw` (Journal Review) helps you discover, parse, and analyze journal files following a specific naming convention. It provides powerful filtering, grouping, and reporting capabilities to review your work activities across projects and time periods.

## Features

- 📁 **Recursive Discovery**: Automatically finds journal files in directory trees
- 📅 **Date Parsing**: Extracts dates from filenames with pattern `yyyy.mm.dd - JRN - <description>.md`
- 🔍 **Metadata Extraction**: Parses markdown content for tasks, activities, notes, and time spent
- 🎯 **Flexible Filtering**: Filter by date ranges, repositories, tasks
- 📊 **Multiple Grouping**: Group by repository, task, date, week, or month
- 📋 **Multiple Output Formats**: Text, Markdown, JSON, HTML, CSV
- 🎨 **Colored Output**: Beautiful colored terminal output
- ⚙️ **Configurable**: Support for TOML configuration files

## Installation

### From Source

```bash
git clone <repository-url>
cd jrnrvw
cargo build --release
cargo install --path .
```

## Quick Start

### Basic Usage

```bash
# Analyze all journals in current directory
jrnrvw

# Analyze specific directory
jrnrvw /path/to/journals

# Last week's work
jrnrvw --last-week

# Generate markdown report
jrnrvw --format markdown -o report.md
```

### Journal File Format

Journal files should follow this naming pattern:
```
yyyy.mm.dd - JRN - <description>.md
```

Example: `2025.11.13 - JRN - project implementation.md`

Expected markdown structure:
```markdown
# yyyy.mm.dd - Journal: <Title>

## Task
<Task or project name>

## Repository
<Repository name>

## Activities
- Activity 1
- Activity 2
- Activity 3

## Notes
<Additional notes>

## Time Spent
<Time spent, e.g., "2h", "1.5 hours">
```

## Usage Examples

### Time Range Filtering

```bash
# Last 7 days
jrnrvw --last-week

# Last 30 days
jrnrvw --last-month

# This week (Monday-Sunday)
jrnrvw --this-week

# Custom date range
jrnrvw --from 2025-11-01 --to 2025-11-13

# Since a specific date
jrnrvw --since 2025-11-01
```

### Filtering

```bash
# Filter by repository
jrnrvw --repo "myproject"

# Filter by task
jrnrvw --task "authentication"

# Combine filters
jrnrvw --last-week --repo "myproject" --with-activities
```

### Grouping and Sorting

```bash
# Group by task instead of repository
jrnrvw --group-by task

# Group by date
jrnrvw --group-by date

# Sort by repository name
jrnrvw --sort-by repo

# Reverse sort order
jrnrvw --sort-by date --reverse
```

### Output Formats

```bash
# Text output (default)
jrnrvw

# JSON output
jrnrvw --format json -o journals.json

# Markdown report
jrnrvw --format markdown -o report.md

# HTML report
jrnrvw --format html -o report.html

# CSV export
jrnrvw --format csv -o journals.csv
```

### Display Options

```bash
# Show only summary
jrnrvw --summary

# Include activity details
jrnrvw --with-activities

# Include notes
jrnrvw --with-notes

# Include statistics
jrnrvw --stats

# Verbose output
jrnrvw -v

# Quiet mode
jrnrvw -q -o output.json
```

## Command-Line Options

```
jrnrvw [OPTIONS] [PATH]

ARGUMENTS:
  [PATH]  Root directory to search (default: current directory)

OPTIONS:
  Time Range:
    --last-week              Last 7 calendar days
    --last-month             Last 30 calendar days
    --this-week              Current calendar week
    --this-month             Current calendar month
    --activity-days <N>      Last N days with journal entries
    --from <DATE>            Start date (yyyy-mm-dd)
    --to <DATE>              End date (yyyy-mm-dd)
    --since <DATE>           All entries since date
    --before <DATE>          All entries before date

  Filtering:
    --repo <NAME>            Filter by repository name (regex)
    --task <NAME>            Filter by task name (regex)

  Grouping:
    --group-by <TYPE>        Group by: repo, task, date, week, month (default: repo)
    --sort-by <FIELD>        Sort by: date, repo, task (default: date)
    --reverse                Reverse sort order

  Output:
    -o, --output <FILE>      Output file (default: stdout)
    -f, --format <FORMAT>    Output format: text, markdown, json, html, csv (default: text)
    --no-color               Disable colored output
    --verbose, -v            Verbose output
    --quiet, -q              Minimal output

  Display Options:
    --summary                Show only summary statistics
    --with-activities        Include activity lists
    --with-notes             Include notes sections
    --stats                  Include statistics

  Other:
    -h, --help               Show help information
    -V, --version            Show version information
    --config <FILE>          Load configuration from file
```

## Configuration

Create a `.jrnrvw.toml` file in your home directory or project root:

```toml
[general]
default_path = "."
default_format = "text"
colored_output = true

[discovery]
exclude_dirs = [".git", "node_modules", "target"]
case_sensitive = false

[output]
default_group_by = "repo"
default_sort_by = "date"
include_stats = true
date_format = "%Y-%m-%d"
```

## Development

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running with Cargo

```bash
cargo run -- tests/fixtures/sample_journals --format text
```

## Project Structure

```
jrnrvw/
├── src/
│   ├── main.rs              # Entry point
│   ├── lib.rs               # Library root
│   ├── cli.rs               # CLI definitions
│   ├── error.rs             # Error types
│   ├── models/              # Data models
│   ├── discovery/           # File scanning
│   ├── parser/              # Markdown parsing
│   ├── analyzer/            # Filtering and grouping
│   ├── output/              # Output formatters
│   └── config/              # Configuration
├── tests/                   # Integration tests
└── Cargo.toml              # Dependencies
```

## License

MIT OR Apache-2.0

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
