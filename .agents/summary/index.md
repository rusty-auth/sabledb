# SableDB Knowledge Base Index

## Purpose
Use this file as the primary context entry when asking an AI assistant about this repository. It explains what each documentation file contains, when to consult it, and how the files relate to one another.

## How to use this knowledge base
1. Start with `codebase_info.md` for repository-wide orientation.
2. Use the table below to select the most relevant document(s).
3. For design questions, consult `architecture.md` and `components.md`.
4. For API, protocol, or integration questions, consult `interfaces.md`.
5. For schema or structured-data questions, consult `data_models.md`.
6. For behavior and execution questions, consult `workflows.md`.
7. For dependency and external-system questions, consult `dependencies.md`.
8. Review `review_notes.md` for documented gaps or caveats before relying on the summary.

## Document map

| File | Purpose | Best used for | Summary |
|---|---|---|---|
| `codebase_info.md` | Repository overview and structural map | Fast orientation, codebase navigation | High-level repository layout, stack, main entry points, and limitations of the current analysis. |
| `architecture.md` | System architecture and design patterns | Understanding how the system is organized | Explains runtime layers, storage/replication/server boundaries, and architectural relationships. |
| `components.md` | Major components and responsibilities | Locating code owners by subsystem | Describes the core modules and what each subsystem is responsible for. |
| `interfaces.md` | APIs, interfaces, and integration points | Finding entry points, public APIs, and protocols | Summarizes exported library APIs, command-line surfaces, config boundaries, and network integration points. |
| `data_models.md` | Data structures and models | Understanding types and persisted structures | Describes configuration structs, metadata models, and storage abstractions. |
| `workflows.md` | Key processes and workflows | Tracing runtime behavior and execution paths | Covers startup, client interaction, storage access, and replication-oriented flows. |
| `dependencies.md` | External dependencies and their usage | Assessing stack, libraries, and tooling | Lists the main third-party crates and tooling families used by the project. |
| `review_notes.md` | Consistency and completeness review | Evaluating confidence and gaps | Notes inconsistencies, missing detail, and documentation limitations. |

## Relationships between documents
- `architecture.md` explains the system layout at a conceptual level; `components.md` maps that layout to code ownership areas.
- `interfaces.md` connects the architecture to concrete APIs and runtime entry points.
- `data_models.md` describes the types that move through the interfaces and workflows.
- `workflows.md` ties the models and interfaces together into execution sequences.
- `dependencies.md` provides context for the technology choices used throughout all other documents.
- `review_notes.md` should be read when deciding whether additional source inspection is needed.

## Guidance for AI assistants
When answering questions:
- Prefer the smallest set of documents that fully addresses the question.
- Use this file first to determine which detailed document to inspect.
- If asked to navigate the repository, begin with `codebase_info.md` and `components.md`.
- If asked how a command, server process, or replication path works, inspect `interfaces.md` and `workflows.md`.
- If asked about persistence format or configuration, inspect `data_models.md` and `dependencies.md`.
- If asked about architectural tradeoffs, inspect `architecture.md` and `review_notes.md`.

## File summaries
### `codebase_info.md`
Repository-wide orientation, language support, technology stack, subsystem map, and Mermaid hierarchy diagram.

### `architecture.md`
Architecture overview, subsystem boundaries, design principles, and Mermaid diagrams showing how data and control move through the system.

### `components.md`
Component-by-component descriptions of the main crates and major library subsystems.

### `interfaces.md`
Public APIs, CLI entry points, configuration and network integration points, and key exported types.

### `data_models.md`
Configuration structs, server options, metadata types, storage abstractions, and other structured data models.

### `workflows.md`
Startup, client handling, command execution, storage, and replication workflows.

### `dependencies.md`
Core third-party libraries, runtime/tooling dependencies, and how they support the project.

### `review_notes.md`
What is consistent, what is incomplete, and what should be checked manually.

## Practical query examples
- "Which file should I read to find the server startup path?"
- "Where are the public library exports for command handling?"
- "What config struct controls RocksDB and replication settings?"
- "How does the client CLI connect and execute commands?"
- "What are the main subsystems and where do they live in the repo?"
