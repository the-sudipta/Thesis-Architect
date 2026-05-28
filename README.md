<div align="center">

# Thesis Architect

### Build a complete thesis and research workspace in seconds.

[![GitHub Repo stars](https://img.shields.io/github/stars/the-sudipta/Thesis-Architect?style=for-the-badge&logo=github&label=Stars)](https://github.com/the-sudipta/Thesis-Architect/stargazers)
[![GitHub forks](https://img.shields.io/github/forks/the-sudipta/Thesis-Architect?style=for-the-badge&logo=github&label=Forks)](https://github.com/the-sudipta/Thesis-Architect/forks)
[![GitHub watchers](https://img.shields.io/github/watchers/the-sudipta/Thesis-Architect?style=for-the-badge&logo=github&label=Watchers)](https://github.com/the-sudipta/Thesis-Architect/watchers)
[![GitHub issues](https://img.shields.io/github/issues/the-sudipta/Thesis-Architect?style=for-the-badge&logo=github&label=Issues)](https://github.com/the-sudipta/Thesis-Architect/issues)
[![GitHub pull requests](https://img.shields.io/github/issues-pr/the-sudipta/Thesis-Architect?style=for-the-badge&logo=github&label=Pull%20Requests)](https://github.com/the-sudipta/Thesis-Architect/pulls)
[![GitHub license](https://img.shields.io/github/license/the-sudipta/Thesis-Architect?style=for-the-badge&label=License)](LICENSE)

[![Latest release](https://img.shields.io/github/v/release/the-sudipta/Thesis-Architect?style=flat-square&logo=github&label=latest%20release)](https://github.com/the-sudipta/Thesis-Architect/releases/latest)
[![Release downloads](https://img.shields.io/github/downloads/the-sudipta/Thesis-Architect/total?style=flat-square&logo=github&label=downloads)](https://github.com/the-sudipta/Thesis-Architect/releases)
[![Last commit](https://img.shields.io/github/last-commit/the-sudipta/Thesis-Architect?style=flat-square&logo=git&label=last%20commit)](https://github.com/the-sudipta/Thesis-Architect/commits/main)
[![Commit activity](https://img.shields.io/github/commit-activity/m/the-sudipta/Thesis-Architect?style=flat-square&logo=git&label=monthly%20commits)](https://github.com/the-sudipta/Thesis-Architect/commits/main)
[![Repo size](https://img.shields.io/github/repo-size/the-sudipta/Thesis-Architect?style=flat-square&label=repo%20size)](https://github.com/the-sudipta/Thesis-Architect)
[![Code size](https://img.shields.io/github/languages/code-size/the-sudipta/Thesis-Architect?style=flat-square&label=code%20size)](https://github.com/the-sudipta/Thesis-Architect)
[![Top language](https://img.shields.io/github/languages/top/the-sudipta/Thesis-Architect?style=flat-square&label=top%20language)](https://github.com/the-sudipta/Thesis-Architect)
[![Language count](https://img.shields.io/github/languages/count/the-sudipta/Thesis-Architect?style=flat-square&label=languages)](https://github.com/the-sudipta/Thesis-Architect)
[![Contributors](https://img.shields.io/github/contributors/the-sudipta/Thesis-Architect?style=flat-square&label=contributors)](https://github.com/the-sudipta/Thesis-Architect/graphs/contributors)

**Repository:** [the-sudipta/Thesis-Architect](https://github.com/the-sudipta/Thesis-Architect)

</div>

Thesis Architect is a small Windows automation tool that creates a complete, ready-to-use research workspace from one project name. It is designed for students, young researchers, thesis writers, and anyone who wants a clean research folder system without rebuilding the same directories by hand every time.

## Project Snapshot

| Area | Stat |
| --- | ---: |
| Blueprint sections | 15 |
| Generated folders | 57 |
| Generated starter files | 35 |
| Total blueprint entries | 92 |
| Release package files | 3 |
| Installers required | 0 |
| Internet required | No |
| Main platform | Windows |
| Main script | `Generate_Research_Structure.ps1` |
| Launcher | `Launch_Generate_Research_Structure.bat` |

> Live GitHub stats are shown in the badges above, including stars, forks, issues, pull requests, latest release, downloads, last commit, repo size, language stats, and contributors.

## Why This Exists

![GitHub issues](https://img.shields.io/github/issues/the-sudipta/Thesis-Architect?style=social&label=Open%20issues)
![GitHub pull requests](https://img.shields.io/github/issues-pr/the-sudipta/Thesis-Architect?style=social&label=Open%20PRs)
![GitHub last commit](https://img.shields.io/github/last-commit/the-sudipta/Thesis-Architect?style=social&label=Updated)

Research folders become messy quickly. PDFs, notes, data, figures, drafts, supervisor feedback, references, and presentations often end up scattered across random locations. That makes it harder to find work, track progress, review literature, and prepare final submissions.

This tool solves that by creating a consistent research structure in seconds. You enter a short research topic name, and the script creates a complete folder tree with starter files for planning, literature review, data collection, methodology, results, references, presentations, submissions, admin notes, and Obsidian templates.

## What It Creates

![Blueprint sections](https://img.shields.io/badge/blueprint%20sections-15-0f9b73?style=for-the-badge)
![Generated folders](https://img.shields.io/badge/generated%20folders-57-2463eb?style=for-the-badge)
![Starter files](https://img.shields.io/badge/starter%20files-35-e35d52?style=for-the-badge)
![Total entries](https://img.shields.io/badge/total%20entries-92-7c4dff?style=for-the-badge)

The generated workspace includes:

- `00_Master_Overview` for research goals, questions, scope, timeline, and progress.
- `01_Brainstorm` for raw ideas, mind maps, possible angles, gaps, and open questions.
- `02_Background_Learning` for field notes, history, important researchers, journals, and topic notes.
- `03_Literature_Review` for PDFs, paper summaries, gap analysis, themes, contradictions, positioning, and reading queues.
- `04_Keywords_and_Concepts` for concept notes and a keyword index.
- `05_Data_Collection` for raw data, surveys, interviews, datasets, and collection notes.
- `06_Methodology` for methodology notes.
- `07_Analysis_and_Results` for processed data, scripts, outputs, charts, and results notes.
- `08_Writing_LaTeX` for writing notes.
- `09_Figures_for_Paper` for diagrams, final figures, tables, and assets.
- `10_References_and_Citations` for BibTeX references and citation strategy.
- `11_Presentations` for progress talks, conferences, seminars, and posters.
- `12_Submissions_and_Revisions` for submitted versions, reviewer comments, responses, revisions, and cover letters.
- `13_Admin_and_Correspondence` for supervisor feedback, meetings, approvals, funding, collaboration, and research logs.
- `14_Obsidian_System` for reusable note templates and attachments.

It also writes a `folder_structure.txt` file inside the generated research folder so users can inspect the complete layout later.

## Requirements

![Windows](https://img.shields.io/badge/platform-Windows-2463eb?style=flat-square&logo=windows)
![PowerShell](https://img.shields.io/badge/runtime-PowerShell-0f9b73?style=flat-square&logo=powershell)
![No install](https://img.shields.io/badge/install-none-e35d52?style=flat-square)
![Offline ready](https://img.shields.io/badge/offline-ready-7c4dff?style=flat-square)

- Windows.
- PowerShell, which is already included with modern Windows.
- No installation and no internet connection required.

## How To Use

![Steps](https://img.shields.io/badge/use%20steps-5-0f9b73?style=for-the-badge)
![Double click launcher](https://img.shields.io/badge/launcher-double%20click-2463eb?style=for-the-badge)
![Generated beside script](https://img.shields.io/badge/output-beside%20script-f3b624?style=for-the-badge)

1. Download or clone this repository.
2. Double-click `Launch_Generate_Research_Structure.bat`.
3. Enter a short research folder name when prompted.
4. Wait a few seconds while the folders and starter files are created.
5. Open the new folder created beside the script.

You can also run the script directly from PowerShell:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File ".\Generate_Research_Structure.ps1"
```

## Example

![Example project](https://img.shields.io/badge/example-AI_in_Education-2463eb?style=flat-square)
![Root folders](https://img.shields.io/badge/root%20folders-15-0f9b73?style=flat-square)

If you enter:

```text
AI_in_Education
```

The tool creates:

```text
AI_in_Education/
|-- 00_Master_Overview/
|-- 01_Brainstorm/
|-- 02_Background_Learning/
|-- 03_Literature_Review/
|-- ...
`-- 14_Obsidian_System/
```

## Repository Web Page

![Static site](https://img.shields.io/badge/site-static%20HTML%2FCSS%2FJS-0f9b73?style=for-the-badge)
![Interactive explorer](https://img.shields.io/badge/interactive-folder%20explorer-2463eb?style=for-the-badge)
![No backend](https://img.shields.io/badge/backend-none-e35d52?style=for-the-badge)

Open `index.html` in a browser to explore the project visually. It includes an interactive temporary folder explorer where users can compare manual folder creation with the automated workflow.

## Release Package

[![Latest release](https://img.shields.io/github/v/release/the-sudipta/Thesis-Architect?style=for-the-badge&logo=github&label=Latest%20Release)](https://github.com/the-sudipta/Thesis-Architect/releases/latest)
[![Total downloads](https://img.shields.io/github/downloads/the-sudipta/Thesis-Architect/total?style=for-the-badge&logo=github&label=Total%20Downloads)](https://github.com/the-sudipta/Thesis-Architect/releases)
![Package files](https://img.shields.io/badge/package%20files-5-7c4dff?style=for-the-badge)

For a GitHub release, include these files in the zip:

- `Generate_Research_Structure.ps1`
- `Launch_Generate_Research_Structure.bat`
- `README.html`
- `README.md`
- `LICENSE`

Users can open `README.html` first for a friendly visual guide, then double-click the `.bat` launcher to create their research workspace.

## Safety Notes

![No delete](https://img.shields.io/badge/deletes%20files-no-0f9b73?style=flat-square)
![No overwrite](https://img.shields.io/badge/overwrites%20existing%20files-no-0f9b73?style=flat-square)
![Creates beside script](https://img.shields.io/badge/creates-beside%20script-2463eb?style=flat-square)

- The script creates the new research folder beside the script itself.
- It does not delete existing files.
- If a folder or file already exists, the script skips it instead of overwriting it.
- Use short, clear project names without special path characters.

## Repository Activity

[![Stars](https://img.shields.io/github/stars/the-sudipta/Thesis-Architect?style=social)](https://github.com/the-sudipta/Thesis-Architect/stargazers)
[![Forks](https://img.shields.io/github/forks/the-sudipta/Thesis-Architect?style=social)](https://github.com/the-sudipta/Thesis-Architect/forks)
[![Watchers](https://img.shields.io/github/watchers/the-sudipta/Thesis-Architect?style=social)](https://github.com/the-sudipta/Thesis-Architect/watchers)

| GitHub Area | Link |
| --- | --- |
| Releases | [View releases](https://github.com/the-sudipta/Thesis-Architect/releases) |
| Issues | [Report a bug or idea](https://github.com/the-sudipta/Thesis-Architect/issues) |
| Pull requests | [View pull requests](https://github.com/the-sudipta/Thesis-Architect/pulls) |
| Contributors | [View contributors](https://github.com/the-sudipta/Thesis-Architect/graphs/contributors) |
| Commit history | [View commits](https://github.com/the-sudipta/Thesis-Architect/commits/main) |

## License

[![MIT License](https://img.shields.io/github/license/the-sudipta/Thesis-Architect?style=for-the-badge&label=License)](LICENSE)

This project is released under the MIT License. See [LICENSE](LICENSE) for details.
