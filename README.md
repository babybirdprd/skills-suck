# skills-suck 🧹

A high-performance "skills compiler" for AI agents. Inspired by [Vercel's findings](https://vercel.com/blog/agents-md-outperforms-skills-in-our-agent-evals) that a consolidated `AGENTS.md` with a minified index outperforms distributed skills.

## The Problem
Modular skills (like those in `.agent/skills` or `.claude/skills`) are great for organization, but often introduce noise or get ignored by agents due to discovery overhead.

## The Solution
`skills-suck` compiles all your modular skills into a single, high-density `AGENTS.md` file at your project root. It uses a **Minified Documentation Index** format to provide the agent with a compressed view of all available resources (scripts, assets, references) without bloating the context window.

## Features
- **Minified Indexing**: Groups files by directory using a dense string format (`dir:{f1,f2}`).
- **Safe Updates**: Uses marker tags (`<!-- SKILLS-COMPILER-START -->`) to update `AGENTS.md` without overwriting your manual notes.
- **Multi-Spec Support**: Discovers skills from over 20+ tools including Amazon Q, Antigravity, Auggie, Claude Code, Cline, CodeBuddy, Codex, Continue, CoStrict, Crush, Cursor, Factory Droid, Gemini CLI, GitHub Copilot, iFlow, Kilo Code, OpenCode, Qoder, Qwen Code, RooCode, Trae, and Windsurf.
- **Rust-Powered**: Fast, reliable, and modular implementation.

## Installation
```bash
cargo install --path .
```

## Usage

### Compile Skills
Run the compiler from your project root:
```bash
skills-suck
```

This will automatically find skills in your dotfiles and update `AGENTS.md`.

### Options
```bash
skills-suck --output MY_AGENTS.md
```

## Philosophy
We believe skills are for **humans to organize**, but `AGENTS.md` is for **machines to execute**. `skills-suck` bridges that gap by automating the compilation of your organizational structure into an optimized machine-readable format.

---
