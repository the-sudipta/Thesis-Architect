# Thesis Architect
# Creates a complete research/thesis folder structure beside this script.

$topicName = Read-Host "Enter Research Topic or Title (Short Name)"
$topicName = $topicName.Trim()

if ([string]::IsNullOrEmpty($topicName)) {
    Write-Host "Topic name cannot be empty. Exiting..." -ForegroundColor Red
    Start-Sleep -Seconds 3
    exit
}

$targetDir = Join-Path -Path $PSScriptRoot -ChildPath $topicName

if (-not (Test-Path -Path $targetDir)) {
    New-Item -Path $targetDir -ItemType Directory | Out-Null
}

$layoutArray = @(
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
    "14_Obsidian_System/_Attachments"
)

$visualTreeText = @'
root
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
'@

Set-Content -Path (Join-Path -Path $targetDir -ChildPath "folder_structure.txt") -Value $visualTreeText -Encoding UTF8

$contentMap = @{
    "Research_Overview.md" = @'
# Master Research Overview Map (MOC)

## Core Objective
Write a short, powerful summary of what this specific research project aims to solve or evaluate.

## Navigation Matrix
* Ideation Hub: [[01_Brainstorm/Raw_Ideas]] | [[01_Brainstorm/Mind_Map.canvas|Visual Concept Canvas]]
* Literature Base: [[03_Literature_Review/Reading_Queue|Reading Queue]] | [[03_Literature_Review/Literature_Gap_Analysis|Identified Research Gaps]]
* Execution & Notes: [[05_Data_Collection/Data_Collection_Notes]] | [[06_Methodology/Methodology_Notes]]
* Continuous Updates: [[13_Admin_and_Correspondence/Research_Log|Daily Technical Research Log]]
'@

    "Research_Questions.md" = @'
# Core Research Questions and Hypotheses

## Research Questions (RQs)
* RQ1:
* RQ2:

## Formulated Hypotheses (H0/H1)
* H1 (Alternative):
* H0 (Null):
'@

    "Sample_Topic_Note.md" = @'
# Topic Note: [Write Field Concept Here]

**Context Stack:** [[02_Background_Learning/Field_Overview]]
**Related Nodes:** [[04_Keywords_and_Concepts/_Keyword_Index]]

---
## Concept Core Background
Deconstruct complex foundational paradigms or textbook methodology definitions here.
'@

    "Sample_AuthorYear_ShortTitle.md" = @'
# [AuthorLastName] et al. [Year] -- [Short Paper Title Here]

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
'@

    "Paper_Summary_Template.md" = @'
# Paper Title

**Link:** [[03_Literature_Review/Papers_PDF/]]
**Authors:** **Year:** 2026

---
## Contribution
* ## Methodology
* ## Keywords
* [[Keyword]]
'@

    "Sample_Keyword_or_Concept.md" = @'
# Concept: [Keyword Identifier]

**Master Index Reference:** [[04_Keywords_and_Concepts/_Keyword_Index]]

---
## Domain Definition
Provide a modular, clear description of this key research parameter.

## Internal Vault Back-links
* Encountered in: [[03_Literature_Review/Paper_Summaries/Sample_AuthorYear_ShortTitle]]
'@

    "Keyword_Definition_Template.md" = @'
# Concept:

---
## Definition
* ## References
* [[Paper_Summary]]
'@

    "Topic_Note_Template.md" = @'
# Topic:

---
## Core Concepts
* 
'@

    "Meeting_Note_Template.md" = @'
# Meeting Minutes: [Date]

**Attendees:** **Main Agenda:** ---
## Key Discussion Points
* ## Action Items and Next Steps
- [ ] Task 1
'@

    "Research_Log.md" = @'
# Daily Technical Research Log and Diary

## [2026-05-25]
* Activity: Vault initialization and repository layout planning established.
* Blockers: None.
* Next Target: Process core literature review stack.
'@

    "references.bib" = @'
@article{SampleDas2026,
  author    = {Das, Sudipta Kumar},
  title     = {Advanced Spatial Models and Multimodal Integration Frameworks},
  journal   = {Journal of Advanced Computer Research},
  year      = {2026},
  volume    = {1},
  pages     = {1--10}
}
'@

    "references_annotated.bib" = @'
@article{SampleDas2026,
  author    = {Das, Sudipta Kumar},
  title     = {Advanced Spatial Models and Multimodal Integration Frameworks},
  journal   = {Journal of Advanced Computer Science},
  year      = {2026},
  note      = {Annotation: Core foundational reference roadmap for metrics optimization.}
}
'@

    "Mind_Map.canvas" = @'
{
  "nodes": [
    {"id": "n1", "type": "text", "text": "# Master Visual Project Mind Map\nDouble-click anywhere to begin connecting nodes.", "x": 0, "y": 0, "width": 450, "height": 120}
  ],
  "edges": []
}
'@
}

$generalBoilerplate = @'
# Document Reference Node

**Project Parent Context:** [[00_Master_Overview/Research_Overview]]
**Last Structural Processing Update:** 2026-05

---
## Notes & Sandbox Track
*Dynamic analytical context logging space.*
'@

Write-Host "Executing Array Processing...`n" -ForegroundColor Green
Write-Host "Created Blueprint Reference: folder_structure.txt" -ForegroundColor Cyan

foreach ($item in $layoutArray) {
    $fullPath = Join-Path -Path $targetDir -ChildPath $item

    if ($item -like "*.*") {
        $parentDir = Split-Path -Path $fullPath -Parent
        if (-not (Test-Path -Path $parentDir)) {
            New-Item -Path $parentDir -ItemType Directory | Out-Null
        }

        if (-not (Test-Path -Path $fullPath)) {
            $fileName = Split-Path -Path $fullPath -Leaf
            $content = $generalBoilerplate
            if ($contentMap.ContainsKey($fileName)) {
                $content = $contentMap[$fileName]
            }
            Set-Content -Path $fullPath -Value $content -Encoding UTF8 | Out-Null
            Write-Host "Created Template File:  $item" -ForegroundColor Gray
        }
    } else {
        if (-not (Test-Path -Path $fullPath)) {
            New-Item -Path $fullPath -ItemType Directory | Out-Null
            Write-Host "Created Directory:      $item" -ForegroundColor Yellow
        }
    }
}

Write-Host "`nStructure safely deployed without any encoding bugs!" -ForegroundColor Green
Start-Sleep -Seconds 4
