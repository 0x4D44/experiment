//! jrnrvw - Journal Review Tool
//!
//! Main entry point for the CLI application

use clap::Parser;
use jrnrvw::{
    cli::Cli,
    discovery::discover_journals,
    analyzer::{EntryFilter, TimeRange, ReportBuilder},
    output::{Formatter, OutputOptions},
    models::{GroupBy, SortBy, OutputFormat},
    Result,
};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    // Determine root path
    let root_path = cli.path.clone()
        .unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    if cli.verbose {
        eprintln!("Scanning directory: {}", root_path.display());
    }

    // Load configuration (optional)
    let _config = if let Some(ref config_path) = cli.config {
        Some(jrnrvw::config::Config::load_from_file(config_path)?)
    } else {
        jrnrvw::config::Config::load_default()?
    };

    // Discover journal files
    let mut entries = discover_journals(&root_path, vec![])?;

    if cli.verbose {
        eprintln!("Found {} journal files", entries.len());
    }

    if entries.is_empty() {
        if !cli.quiet {
            println!("No journal files found in {}", root_path.display());
        }
        return Ok(());
    }

    // Parse content for each entry
    for entry in &mut entries {
        if let Ok(content) = fs::read_to_string(&entry.filepath) {
            entry.raw_content = content.clone();

            // Parse the journal content
            let parser = jrnrvw::parser::JournalParser::new(content);
            if let Ok(parsed) = parser.parse() {
                let extractor = jrnrvw::parser::MetadataExtractor::new(parsed.sections);

                entry.task = extractor.extract_task();
                entry.activities = extractor.extract_activities();
                entry.notes = extractor.extract_notes();
                entry.time_spent = extractor.extract_time_spent();

                // Override repository if specified in journal
                if let Some(repo) = extractor.extract_repository() {
                    entry.repository = Some(repo);
                }
            }
        }
    }

    // Build filter from CLI arguments
    let filter = build_filter(&cli)?;

    // Build report with grouping
    let group_by = convert_group_by(cli.group_by);
    let sort_by = convert_sort_by(cli.sort_by);

    // Build report
    let report = ReportBuilder::new(entries)
        .with_filter(filter)
        .with_grouping(group_by, sort_by)
        .build()?;

    // Check if AI summarization is requested
    if cli.summarize {
        if cli.verbose {
            eprintln!("Generating AI summary using {}...", format!("{:?}", cli.llm).to_lowercase());
        }

        // Get repositories and date range from report
        let repositories = &report.repositories;
        let date_range = report.metadata.period.as_ref().map(|dr| (dr.from, dr.to));

        // Convert CLI LlmArg to LlmProvider
        let llm_provider = match cli.llm {
            jrnrvw::cli::LlmArg::Claude => jrnrvw::llm::LlmProvider::Claude,
            jrnrvw::cli::LlmArg::Codex => jrnrvw::llm::LlmProvider::Codex,
        };

        // Generate summary
        let summary = jrnrvw::llm::summarize(llm_provider, repositories, date_range)?;

        // Write summary output
        if let Some(ref summary_path) = cli.summary_output {
            fs::write(summary_path, &summary)?;
            if !cli.quiet {
                eprintln!("AI summary written to {}", summary_path.display());
            }
        } else {
            println!("{}", summary);
            io::stdout().flush()?;
        }

        // If --summary-output is specified, also generate the regular report
        if cli.summary_output.is_some() && cli.output.is_some() {
            let output_options = OutputOptions {
                colored: !cli.no_color && atty::is(atty::Stream::Stdout),
                verbose: cli.verbose,
                include_activities: cli.with_activities || !cli.summary,
                include_notes: cli.with_notes,
                include_stats: cli.stats || !cli.summary,
                summary_only: cli.summary,
            };

            let output_format = convert_format(cli.format);
            let formatted = format_report(&report, output_format, &output_options)?;

            if let Some(output_path) = cli.output {
                fs::write(&output_path, formatted)?;
                if !cli.quiet {
                    eprintln!("Report written to {}", output_path.display());
                }
            }
        }

        return Ok(());
    }

    // Build output options
    let output_options = OutputOptions {
        colored: !cli.no_color && atty::is(atty::Stream::Stdout),
        verbose: cli.verbose,
        include_activities: cli.with_activities || !cli.summary,
        include_notes: cli.with_notes,
        include_stats: cli.stats || !cli.summary,
        summary_only: cli.summary,
    };

    // Format output
    let output_format = convert_format(cli.format);
    let formatted = format_report(&report, output_format, &output_options)?;

    // Write output
    if let Some(output_path) = cli.output {
        fs::write(&output_path, formatted)?;
        if !cli.quiet {
            eprintln!("Report written to {}", output_path.display());
        }
    } else {
        print!("{}", formatted);
        io::stdout().flush()?;
    }

    Ok(())
}

fn build_filter(cli: &Cli) -> Result<EntryFilter> {
    let mut filter = EntryFilter::new();

    // Determine time range
    let time_range = if cli.last_week {
        Some(TimeRange::LastWeek)
    } else if cli.last_month {
        Some(TimeRange::LastMonth)
    } else if cli.this_week {
        Some(TimeRange::ThisWeek)
    } else if cli.this_month {
        Some(TimeRange::ThisMonth)
    } else if cli.activity_days.is_some() || cli.activity_window.is_some() {
        // Activity days is a simple flag in the enum
        Some(TimeRange::ActivityDays)
    } else if let (Some(from), Some(to)) = (cli.from, cli.to) {
        Some(TimeRange::Custom(from, to))
    } else if let Some(date) = cli.since {
        Some(TimeRange::Since(date))
    } else if let Some(date) = cli.before {
        Some(TimeRange::Before(date))
    } else {
        None
    };

    if let Some(range) = time_range {
        filter = filter.with_time_range(range);
    }

    // Repository filter
    if let Some(repo) = &cli.repo {
        filter = filter.with_repository(repo.clone());
    }

    // Task filter
    if let Some(task) = &cli.task {
        filter = filter.with_task(task.clone());
    }

    Ok(filter)
}

fn convert_group_by(arg: jrnrvw::cli::GroupByArg) -> GroupBy {
    match arg {
        jrnrvw::cli::GroupByArg::Repo => GroupBy::Repository,
        jrnrvw::cli::GroupByArg::Task => GroupBy::Task,
        jrnrvw::cli::GroupByArg::Date => GroupBy::Date,
        jrnrvw::cli::GroupByArg::Week => GroupBy::Week,
        jrnrvw::cli::GroupByArg::Month => GroupBy::Month,
    }
}

fn convert_sort_by(arg: jrnrvw::cli::SortByArg) -> SortBy {
    match arg {
        jrnrvw::cli::SortByArg::Date => SortBy::Date,
        jrnrvw::cli::SortByArg::Repo => SortBy::Repository,
        jrnrvw::cli::SortByArg::Task => SortBy::Task,
    }
}

fn convert_format(arg: jrnrvw::cli::FormatArg) -> OutputFormat {
    match arg {
        jrnrvw::cli::FormatArg::Text => OutputFormat::Text,
        jrnrvw::cli::FormatArg::Markdown => OutputFormat::Markdown,
        jrnrvw::cli::FormatArg::Json => OutputFormat::Json,
        jrnrvw::cli::FormatArg::Html => OutputFormat::Html,
        jrnrvw::cli::FormatArg::Csv => OutputFormat::Csv,
    }
}

fn format_report(
    report: &jrnrvw::Report,
    format: OutputFormat,
    options: &OutputOptions,
) -> Result<String> {
    match format {
        OutputFormat::Text => {
            let formatter = jrnrvw::output::text::TextFormatter::new();
            formatter.format(report, options)
        }
        OutputFormat::Markdown => {
            let formatter = jrnrvw::output::markdown::MarkdownFormatter::new();
            formatter.format(report, options)
        }
        OutputFormat::Json => {
            let formatter = jrnrvw::output::json::JsonFormatter::new();
            formatter.format(report, options)
        }
        OutputFormat::Html => {
            let formatter = jrnrvw::output::html::HtmlFormatter::new()?;
            formatter.format(report, options)
        }
        OutputFormat::Csv => {
            let formatter = jrnrvw::output::csv::CsvFormatter::new();
            formatter.format(report, options)
        }
    }
}
