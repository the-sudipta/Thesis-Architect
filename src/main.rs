use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

const APP_NAME: &str = "Thesis Architect";

const LAYOUT: &[&str] = &[
    "00_Master_Overview",
    "00_Master_Overview/Research_Overview.md",
    "00_Master_Overview/Research_Questions.md",
    "00_Master_Overview/Scope_and_Limitations.md",
    "00_Master_Overview/Timeline_and_Milestones.md",
    "00_Master_Overview/Progress_Checklist.md",
    "01_Brainstorm",
    "01_Brainstorm/Raw_Ideas.md",
    "01_Brainstorm/Mind_Map.canvas",
    "01_Brainstorm/Possible_Angles.md",
    "01_Brainstorm/Research_Gaps_I_See.md",
    "01_Brainstorm/Questions_I_Have.md",
    "02_Background_Learning",
    "02_Background_Learning/Field_Overview.md",
    "02_Background_Learning/History_of_the_Field.md",
    "02_Background_Learning/Key_Researchers_and_Labs.md",
    "02_Background_Learning/Important_Journals_and_Conferences.md",
    "02_Background_Learning/Topic_Notes",
    "02_Background_Learning/Topic_Notes/Sample_Topic_Note.md",
    "03_Literature_Review",
    "03_Literature_Review/Papers_PDF",
    "03_Literature_Review/Papers_PDF/Foundational",
    "03_Literature_Review/Papers_PDF/Recent",
    "03_Literature_Review/Papers_PDF/Methodology_Related",
    "03_Literature_Review/Papers_PDF/Background_Related",
    "03_Literature_Review/Papers_PDF/Not_Yet_Read",
    "03_Literature_Review/Paper_Summaries",
    "03_Literature_Review/Paper_Summaries/Sample_AuthorYear_ShortTitle.md",
    "03_Literature_Review/Literature_Gap_Analysis.md",
    "03_Literature_Review/Themes_and_Patterns.md",
    "03_Literature_Review/Contradictions_Found.md",
    "03_Literature_Review/My_Positioning.md",
    "03_Literature_Review/Reading_Queue.md",
    "04_Keywords_and_Concepts",
    "04_Keywords_and_Concepts/Sample_Keyword_or_Concept.md",
    "04_Keywords_and_Concepts/_Keyword_Index.md",
    "05_Data_Collection",
    "05_Data_Collection/Raw_Data",
    "05_Data_Collection/Raw_Data/Primary",
    "05_Data_Collection/Raw_Data/Secondary",
    "05_Data_Collection/Survey_or_Questionnaire",
    "05_Data_Collection/Survey_or_Questionnaire/Instrument_Design",
    "05_Data_Collection/Survey_or_Questionnaire/Responses",
    "05_Data_Collection/Interview_or_Field_Work",
    "05_Data_Collection/Interview_or_Field_Work/Transcripts",
    "05_Data_Collection/Interview_or_Field_Work/Field_Notes",
    "05_Data_Collection/Datasets",
    "05_Data_Collection/Data_Collection_Notes.md",
    "06_Methodology",
    "06_Methodology/Methodology_Notes.md",
    "07_Analysis_and_Results",
    "07_Analysis_and_Results/Processed_Data",
    "07_Analysis_and_Results/Code_and_Scripts",
    "07_Analysis_and_Results/Statistical_Outputs",
    "07_Analysis_and_Results/Figures_and_Charts_Raw",
    "07_Analysis_and_Results/Results_Notes.md",
    "08_Writing_LaTeX",
    "08_Writing_LaTeX/Writing_Notes.md",
    "09_Figures_for_Paper",
    "09_Figures_for_Paper/Diagrams_and_Flowcharts",
    "09_Figures_for_Paper/Result_Figures_Final",
    "09_Figures_for_Paper/Tables_Final",
    "09_Figures_for_Paper/Assets",
    "10_References_and_Citations",
    "10_References_and_Citations/references.bib",
    "10_References_and_Citations/references_annotated.bib",
    "10_References_and_Citations/Citation_Strategy.md",
    "11_Presentations",
    "11_Presentations/Progress_Presentations",
    "11_Presentations/Conference_Presentations",
    "11_Presentations/Seminar_Slides",
    "11_Presentations/Posters",
    "12_Submissions_and_Revisions",
    "12_Submissions_and_Revisions/Submitted_Versions",
    "12_Submissions_and_Revisions/Reviewer_Comments",
    "12_Submissions_and_Revisions/Response_to_Reviewers",
    "12_Submissions_and_Revisions/Revised_Versions",
    "12_Submissions_and_Revisions/Cover_Letters",
    "13_Admin_and_Correspondence",
    "13_Admin_and_Correspondence/Supervisor_Feedback",
    "13_Admin_and_Correspondence/Meeting_Notes",
    "13_Admin_and_Correspondence/Ethics_and_Approvals",
    "13_Admin_and_Correspondence/Funding_Documents",
    "13_Admin_and_Correspondence/Collaboration",
    "13_Admin_and_Correspondence/Research_Log.md",
    "14_Obsidian_System",
    "14_Obsidian_System/Templates",
    "14_Obsidian_System/Templates/Paper_Summary_Template.md",
    "14_Obsidian_System/Templates/Keyword_Definition_Template.md",
    "14_Obsidian_System/Templates/Topic_Note_Template.md",
    "14_Obsidian_System/Templates/Meeting_Note_Template.md",
    "14_Obsidian_System/_Attachments",
];

const VISUAL_TREE_TEXT: &str = r##"root
|-- 00_Master_Overview
|   |-- Research_Overview.md
|   |-- Research_Questions.md
|   |-- Scope_and_Limitations.md
|   |-- Timeline_and_Milestones.md
|   `-- Progress_Checklist.md
|-- 01_Brainstorm
|   |-- Raw_Ideas.md
|   |-- Mind_Map.canvas
|   |-- Possible_Angles.md
|   |-- Research_Gaps_I_See.md
|   `-- Questions_I_Have.md
|-- 02_Background_Learning
|   |-- Field_Overview.md
|   |-- History_of_the_Field.md
|   |-- Key_Researchers_and_Labs.md
|   |-- Important_Journals_and_Conferences.md
|   `-- Topic_Notes
|       `-- Sample_Topic_Note.md
|-- 03_Literature_Review
|   |-- Papers_PDF
|   |   |-- Foundational
|   |   |-- Recent
|   |   |-- Methodology_Related
|   |   |-- Background_Related
|   |   `-- Not_Yet_Read
|   |-- Paper_Summaries
|   |   `-- Sample_AuthorYear_ShortTitle.md
|   |-- Literature_Gap_Analysis.md
|   |-- Themes_and_Patterns.md
|   |-- Contradictions_Found.md
|   |-- My_Positioning.md
|   `-- Reading_Queue.md
|-- 04_Keywords_and_Concepts
|   |-- Sample_Keyword_or_Concept.md
|   `-- _Keyword_Index.md
|-- 05_Data_Collection
|   |-- Raw_Data
|   |   |-- Primary
|   |   `-- Secondary
|   |-- Survey_or_Questionnaire
|   |   |-- Instrument_Design
|   |   `-- Responses
|   |-- Interview_or_Field_Work
|   |   |-- Transcripts
|   |   `-- Field_Notes
|   |-- Datasets
|   `-- Data_Collection_Notes.md
|-- 06_Methodology
|   `-- Methodology_Notes.md
|-- 07_Analysis_and_Results
|   |-- Processed_Data
|   |-- Code_and_Scripts
|   |-- Statistical_Outputs
|   |-- Figures_and_Charts_Raw
|   `-- Results_Notes.md
|-- 08_Writing_LaTeX
|   `-- Writing_Notes.md
|-- 09_Figures_for_Paper
|   |-- Diagrams_and_Flowcharts
|   |-- Result_Figures_Final
|   |-- Tables_Final
|   `-- Assets
|-- 10_References_and_Citations
|   |-- references.bib
|   |-- references_annotated.bib
|   `-- Citation_Strategy.md
|-- 11_Presentations
|   |-- Progress_Presentations
|   |-- Conference_Presentations
|   |-- Seminar_Slides
|   `-- Posters
|-- 12_Submissions_and_Revisions
|   |-- Submitted_Versions
|   |-- Reviewer_Comments
|   |-- Response_to_Reviewers
|   |-- Revised_Versions
|   `-- Cover_Letters
|-- 13_Admin_and_Correspondence
|   |-- Supervisor_Feedback
|   |-- Meeting_Notes
|   |-- Ethics_and_Approvals
|   |-- Funding_Documents
|   |-- Collaboration
|   `-- Research_Log.md
`-- 14_Obsidian_System
    |-- Templates
    |   |-- Paper_Summary_Template.md
    |   |-- Keyword_Definition_Template.md
    |   |-- Topic_Note_Template.md
    |   `-- Meeting_Note_Template.md
    `-- _Attachments
"##;

const GENERAL_BOILERPLATE: &str = r##"# Document Reference Node

**Project Parent Context:** [[00_Master_Overview/Research_Overview]]
**Last Structural Processing Update:** 2026-05

---
## Notes & Sandbox Track
*Dynamic analytical context logging space.*
"##;

const RESEARCH_OVERVIEW: &str = r##"# Master Research Overview Map (MOC)

## Core Objective
Write a short, powerful summary of what this specific research project aims to solve or evaluate.

## Navigation Matrix
* Ideation Hub: [[01_Brainstorm/Raw_Ideas]] | [[01_Brainstorm/Mind_Map.canvas|Visual Concept Canvas]]
* Literature Base: [[03_Literature_Review/Reading_Queue|Reading Queue]] | [[03_Literature_Review/Literature_Gap_Analysis|Identified Research Gaps]]
* Execution & Notes: [[05_Data_Collection/Data_Collection_Notes]] | [[06_Methodology/Methodology_Notes]]
* Continuous Updates: [[13_Admin_and_Correspondence/Research_Log|Daily Technical Research Log]]
"##;

const RESEARCH_QUESTIONS: &str = r##"# Core Research Questions and Hypotheses

## Research Questions (RQs)
* RQ1:
* RQ2:

## Formulated Hypotheses (H0/H1)
* H1 (Alternative):
* H0 (Null):
"##;

const SAMPLE_TOPIC_NOTE: &str = r##"# Topic Note: [Write Field Concept Here]

**Context Stack:** [[02_Background_Learning/Field_Overview]]
**Related Nodes:** [[04_Keywords_and_Concepts/_Keyword_Index]]

---
## Concept Core Background
Deconstruct complex foundational paradigms or textbook methodology definitions here.
"##;

const SAMPLE_PAPER_SUMMARY: &str = r##"# [AuthorLastName] et al. [Year] -- [Short Paper Title Here]

**File Reference Link:** [[03_Literature_Review/Papers_PDF/Not_Yet_Read/Sample.pdf]]
**Year:** 2026 | **Journal/Conference:** **Status:** Unread / Read

---
## Core Contribution
* What is the definitive baseline asset or performance tracking advancement introduced?

## Operational Methodology
* How did the authors assemble their data processing pipelines?

## Key Results and Empirical Evidence
* Benchmark validation results or confusion matrix achievements.

## Limitations and Identified Gaps
* Structural weaknesses noted by authors or caught during analysis.

## Keywords Link Ecosystem
* [[Sample_Keyword_or_Concept]]
"##;

const PAPER_SUMMARY_TEMPLATE: &str = r##"# Paper Title

**Link:** [[03_Literature_Review/Papers_PDF/]]
**Authors:** **Year:** 2026

---
## Contribution
* ## Methodology
* ## Keywords
* [[Keyword]]
"##;

const SAMPLE_KEYWORD: &str = r##"# Concept: [Keyword Identifier]

**Master Index Reference:** [[04_Keywords_and_Concepts/_Keyword_Index]]

---
## Domain Definition
Provide a modular, clear description of this key research parameter.

## Internal Vault Back-links
* Encountered in: [[03_Literature_Review/Paper_Summaries/Sample_AuthorYear_ShortTitle]]
"##;

const KEYWORD_TEMPLATE: &str = r##"# Concept:

---
## Definition
* ## References
* [[Paper_Summary]]
"##;

const TOPIC_NOTE_TEMPLATE: &str = r##"# Topic:

---
## Core Concepts
* 
"##;

const MEETING_NOTE_TEMPLATE: &str = r##"# Meeting Minutes: [Date]

**Attendees:** **Main Agenda:** ---
## Key Discussion Points
* ## Action Items and Next Steps
- [ ] Task 1
"##;

const RESEARCH_LOG: &str = r##"# Daily Technical Research Log and Diary

## [2026-05-25]
* Activity: Vault initialization and repository layout planning established.
* Blockers: None.
* Next Target: Process core literature review stack.
"##;

const REFERENCES_BIB: &str = r##"@article{SampleDas2026,
  author    = {Das, Sudipta Kumar},
  title     = {Advanced Spatial Models and Multimodal Integration Frameworks},
  journal   = {Journal of Advanced Computer Research},
  year      = {2026},
  volume    = {1},
  pages     = {1--10}
}
"##;

const REFERENCES_ANNOTATED_BIB: &str = r##"@article{SampleDas2026,
  author    = {Das, Sudipta Kumar},
  title     = {Advanced Spatial Models and Multimodal Integration Frameworks},
  journal   = {Journal of Advanced Computer Science},
  year      = {2026},
  note      = {Annotation: Core foundational reference roadmap for metrics optimization.}
}
"##;

const MIND_MAP_CANVAS: &str = r##"{
  "nodes": [
    {"id": "n1", "type": "text", "text": "# Master Visual Project Mind Map\nDouble-click anywhere to begin connecting nodes.", "x": 0, "y": 0, "width": 450, "height": 120}
  ],
  "edges": []
}
"##;

#[derive(Debug)]
struct Config {
    topic_name: Option<String>,
    output_dir: PathBuf,
    pause_at_end: bool,
}

#[derive(Default, Debug)]
struct DeployStats {
    created_project_root: bool,
    created_blueprint: bool,
    created_dirs: usize,
    created_files: usize,
    skipped_existing: usize,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("\nError: {error}");
        let _ = pause("Press Enter to exit...");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let config = parse_args(env::args_os().skip(1))?;

    if config.topic_name.as_deref() == Some("__help__") {
        print_help();
        return Ok(());
    }

    if config.topic_name.as_deref() == Some("__version__") {
        println!("{} {}", APP_NAME, env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    println!("{APP_NAME}");
    println!("Create a complete thesis and research workspace.\n");

    let topic_name = match config.topic_name {
        Some(name) => name.trim().to_owned(),
        None => prompt("Enter Research Topic or Title (Short Name): ")?,
    };

    validate_topic_name(&topic_name)?;

    let target_dir = config.output_dir.join(&topic_name);
    let stats = deploy_structure(&target_dir)?;

    println!("\nStructure safely deployed.");
    println!("Location: {}", target_dir.display());
    println!(
        "Project folder: {}.",
        if stats.created_project_root {
            "created"
        } else {
            "already existed"
        }
    );
    println!(
        "Created {} folders and {} starter files. Blueprint map {}. Skipped {} existing item(s).",
        stats.created_dirs,
        stats.created_files,
        if stats.created_blueprint {
            "created"
        } else {
            "already existed"
        },
        stats.skipped_existing
    );

    if config.pause_at_end {
        pause("\nPress Enter to exit...")?;
    }

    Ok(())
}

fn parse_args<I>(args: I) -> Result<Config, Box<dyn Error>>
where
    I: IntoIterator<Item = OsString>,
{
    let mut topic_name = None;
    let mut output_dir = env::current_dir()?;
    let mut pause_at_end = true;
    let mut args = args.into_iter().peekable();

    while let Some(arg) = args.next() {
        let arg_text = arg.to_string_lossy();

        match arg_text.as_ref() {
            "-h" | "--help" => {
                topic_name = Some("__help__".to_owned());
            }
            "-V" | "--version" => {
                topic_name = Some("__version__".to_owned());
            }
            "--no-pause" => {
                pause_at_end = false;
            }
            "-o" | "--output" => {
                let Some(value) = args.next() else {
                    return Err("missing path after --output".into());
                };
                output_dir = PathBuf::from(value);
            }
            _ if arg_text.starts_with("--output=") => {
                let value = arg_text.trim_start_matches("--output=");
                if value.is_empty() {
                    return Err("missing path after --output=".into());
                }
                output_dir = PathBuf::from(value);
            }
            _ if arg_text.starts_with('-') => {
                return Err(format!("unknown option: {arg_text}").into());
            }
            _ => {
                if topic_name.is_some() {
                    return Err("only one project name can be provided".into());
                }
                topic_name = Some(arg_text.into_owned());
                pause_at_end = false;
            }
        }
    }

    Ok(Config {
        topic_name,
        output_dir,
        pause_at_end,
    })
}

fn print_help() {
    println!(
        "{APP_NAME} {}

Usage:
  thesis-architect [PROJECT_NAME] [--output DIR] [--no-pause]

Examples:
  thesis-architect
  thesis-architect AI_in_Education
  thesis-architect \"AI in Education\" --output ./research

Options:
  -o, --output DIR  Create the research workspace inside DIR
      --no-pause    Exit immediately after completion
  -h, --help        Show this help
  -V, --version     Show version",
        env!("CARGO_PKG_VERSION")
    );
}

fn prompt(message: &str) -> io::Result<String> {
    print!("{message}");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_owned())
}

fn pause(message: &str) -> io::Result<()> {
    print!("{message}");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(())
}

fn validate_topic_name(topic_name: &str) -> Result<(), Box<dyn Error>> {
    let topic_name = topic_name.trim();

    if topic_name.is_empty() {
        return Err("topic name cannot be empty".into());
    }

    if topic_name == "." || topic_name == ".." {
        return Err("topic name cannot be '.' or '..'".into());
    }

    let invalid_chars = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];
    if topic_name
        .chars()
        .any(|character| character.is_control() || invalid_chars.contains(&character))
    {
        return Err("topic name contains a character that is not safe for file paths".into());
    }

    Ok(())
}

fn deploy_structure(target_dir: &Path) -> Result<DeployStats, Box<dyn Error>> {
    let mut stats = DeployStats::default();

    if target_dir.exists() {
        stats.skipped_existing += 1;
    } else {
        fs::create_dir_all(target_dir)?;
        stats.created_project_root = true;
    }

    let blueprint_path = target_dir.join("folder_structure.txt");
    write_blueprint_if_missing(&blueprint_path, VISUAL_TREE_TEXT, &mut stats)?;

    println!("Executing structure generation...\n");

    for item in LAYOUT {
        let relative_path = path_from_blueprint_item(item);
        let full_path = target_dir.join(relative_path);

        if is_file_item(item) {
            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent)?;
            }

            let file_name = full_path
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or("blueprint file name is not valid UTF-8")?;
            write_file_if_missing(&full_path, content_for(file_name), &mut stats)?;
        } else if full_path.exists() {
            stats.skipped_existing += 1;
        } else {
            fs::create_dir_all(&full_path)?;
            stats.created_dirs += 1;
            println!("Created Directory:      {}", item);
        }
    }

    Ok(stats)
}

fn write_blueprint_if_missing(
    path: &Path,
    content: &str,
    stats: &mut DeployStats,
) -> Result<(), Box<dyn Error>> {
    if path.exists() {
        stats.skipped_existing += 1;
        return Ok(());
    }

    fs::write(path, content)?;
    stats.created_blueprint = true;
    println!("Created Blueprint Map:  folder_structure.txt");

    Ok(())
}

fn path_from_blueprint_item(item: &str) -> PathBuf {
    item.split('/').collect()
}

fn is_file_item(item: &str) -> bool {
    Path::new(item).extension().is_some()
}

fn write_file_if_missing(
    path: &Path,
    content: &str,
    stats: &mut DeployStats,
) -> Result<(), Box<dyn Error>> {
    if path.exists() {
        stats.skipped_existing += 1;
        return Ok(());
    }

    fs::write(path, content)?;
    stats.created_files += 1;

    if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
        println!("Created Template File:  {}", file_name);
    }

    Ok(())
}

fn content_for(file_name: &str) -> &'static str {
    match file_name {
        "Research_Overview.md" => RESEARCH_OVERVIEW,
        "Research_Questions.md" => RESEARCH_QUESTIONS,
        "Sample_Topic_Note.md" => SAMPLE_TOPIC_NOTE,
        "Sample_AuthorYear_ShortTitle.md" => SAMPLE_PAPER_SUMMARY,
        "Paper_Summary_Template.md" => PAPER_SUMMARY_TEMPLATE,
        "Sample_Keyword_or_Concept.md" => SAMPLE_KEYWORD,
        "Keyword_Definition_Template.md" => KEYWORD_TEMPLATE,
        "Topic_Note_Template.md" => TOPIC_NOTE_TEMPLATE,
        "Meeting_Note_Template.md" => MEETING_NOTE_TEMPLATE,
        "Research_Log.md" => RESEARCH_LOG,
        "references.bib" => REFERENCES_BIB,
        "references_annotated.bib" => REFERENCES_ANNOTATED_BIB,
        "Mind_Map.canvas" => MIND_MAP_CANVAS,
        _ => GENERAL_BOILERPLATE,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn rejects_empty_or_unsafe_topic_names() {
        assert!(validate_topic_name("").is_err());
        assert!(validate_topic_name("..").is_err());
        assert!(validate_topic_name("bad/name").is_err());
        assert!(validate_topic_name("good topic").is_ok());
    }

    #[test]
    fn deploys_expected_structure_without_overwriting_existing_files() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after Unix epoch")
            .as_nanos();
        let root = env::temp_dir().join(format!("thesis_architect_test_{unique}"));
        let target = root.join("AI_in_Education");

        deploy_structure(&target).expect("first deployment should succeed");

        let overview = target
            .join("00_Master_Overview")
            .join("Research_Overview.md");
        assert!(overview.exists());
        assert!(target.join("folder_structure.txt").exists());
        assert!(target
            .join("14_Obsidian_System")
            .join("Templates")
            .join("Meeting_Note_Template.md")
            .exists());

        fs::write(&overview, "keep me").expect("fixture write should succeed");
        deploy_structure(&target).expect("second deployment should skip existing files");
        assert_eq!(
            fs::read_to_string(&overview).expect("overview should be readable"),
            "keep me"
        );

        fs::remove_dir_all(root).expect("test fixture should be removable");
    }
}
