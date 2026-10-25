# Office and Productivity Suites Specification

> **Principles:** ABSTRACTION · CODE REUSABILITY · INTERFACE SEGREGATION · OPEN STANDARD AND INTEROPERABILITY · SEPARATION OF CONCERNS · UNIFORM ACCESS · USER FREEDOM AND CONTROL · ZERO-COPY · FIRST TESTING · LOCALITY OF REFERENCE

This specification documents the document processing, spreadsheet calculation, presentation authoring, vector drawing, mathematical typesetting, database frontend, AI-assisted document generation, real-time collaboration, and Craft format interoperability architectures implemented in **SigmaOS** and **[`SigmaOS-office`](https://github.com/AaryanSinghChauhan09/SigmaOS-office)**.

---

## 1. Office Module Matrix & Upstream Inspirations

SigmaOS delivers native productivity capabilities inspired by leading open-source and modern office suites:
- **LibreOffice & Apache OpenOffice:** Complete 6-module application parity (Writer, Calc, Impress, Draw, Math, Base) and icon/style themes (`libreoffice-mint`, `libreoffice-breeze`, `libreoffice-elementary`).
- **GenOffice:** Offline-first generative AI assistant for prompt-to-document synthesis, presentation generation, summarization, readability analysis, and template expansion.
- **OnlyOffice:** Operational Transformation (OT) co-authoring engine, document locking, threaded comments, and user presence telemetry.
- **CraftOffice (Craft Creative Suite):** Native `.wcf` (WordCraft), `.gcf` (GridCraft), `.dcf` (DeckCraft), `.vcf` (VectorCraft), `.mcf` (MathCraft), and `.dbcf` (DataCraft) format processing.

| Productivity Domain | LibreOffice / OpenOffice | OnlyOffice Parity | SigmaOS Craft Engine | Supported File Formats |
| :--- | :--- | :--- | :--- | :--- |
| **Word Processing** | Writer | Document Server | `WordCraft` | `.odt`, `.docx`, `.wcf`, `.rtf`, `.txt`, `.md` |
| **Spreadsheets & Modeling** | Calc | Spreadsheet Server | `GridCraft` | `.ods`, `.xlsx`, `.gcf`, `.csv`, `.tsv`, `.arrow` |
| **Presentations & Decks** | Impress | Presentation Server | `DeckCraft` | `.odp`, `.pptx`, `.dcf`, `.pdf`, interactive HTML |
| **Vector Graphics & Flow** | Draw | Shape Engine | `VectorCraft` | `.odg`, `.svg`, `.vcf`, `.eps`, `.pdf` |
| **Formula Typesetting** | Math | Equation Editor | `MathCraft` | MathML, LaTeX, `.mcf`, OpenFormula |
| **Relational Database** | Base | Data Connector | `DataCraft` | SQLite, DuckDB, `.dbcf`, ODBC, CSV |
| **AI Document Assistant** | — | AI Plugins | `GenOffice AI` | AST synthesis, Markdown summaries |
| **Real-Time Collaboration** | — | Co-authoring Core | `OnlyOffice Collab` | Operational Transform, Threaded Comments |

---

## 2. System Architecture

```mermaid
graph TD
    UI["SigmaOS Zenith Desktop / CLI (sigma-office)"] --> BRIDGE["CraftBridge & Format Filters"]
    
    subgraph Core Engines (SigmaOS-office)
        WRITER["DocumentEngine (Writer / WordCraft)"]
        CALC["SpreadsheetEngine (Calc / GridCraft)"]
        IMPRESS["PresentationEngine (Impress / DeckCraft)"]
        DRAW["VectorDrawEngine (Draw / VectorCraft)"]
        MATH["MathEngine (Math / MathCraft)"]
        BASE["DatabaseEngine (Base / DataCraft)"]
        GENAI["GenOffice AI Assistant"]
        COLLAB["OnlyOffice Collab & OT Engine"]
        STYLE["Style & Icon Coverage Planner"]
    end
    
    BRIDGE --> WRITER
    BRIDGE --> CALC
    BRIDGE --> IMPRESS
    BRIDGE --> DRAW
    BRIDGE --> MATH
    BRIDGE --> BASE
    
    GENAI --> WRITER
    GENAI --> IMPRESS
    COLLAB --> WRITER
    STYLE --> UI
```

---

## 3. Sub-Engine Implementation Details

### 3.1 Document Engine (`src/document_engine.py`)
- **AST Architecture:** Hierarchical representation of `CharacterStyle`, `TextRun`, `Paragraph` (with alignment, heading levels, and bullet lists), `Table` with `TableCell` spans, and `TrackedChange` logs.
- **Serialization Filters:** Multi-format conversion to Markdown, JSON, and plain text.
- **Metrics:** Real-time word count and character count calculation without external dependencies.

### 3.2 Spreadsheet Engine (`src/spreadsheet_engine.py`)
- **Cell Matrix:** Sparse 2D coordinate grid parsing alphanumeric coordinates (`A1`, `B5`, `AA10`) to 0-based indices and vice versa.
- **Formula DAG Evaluator:** Recursive formula evaluator supporting:
  - Aggregations over ranges: `SUM(A1:B10)`, `AVERAGE(A1:A5)`, `MIN`, `MAX`, `COUNT`.
  - Conditionals: `IF(condition, true_val, false_val)`.
  - Arithmetic: Binary expression evaluation (`+`, `-`, `*`, `/`).
- **CSV Engine:** Bidirectional streaming CSV export and import.

### 3.3 Presentation Engine (`src/presentation_engine.py`)
- **Slide Deck Structure:** Presentation AST managing `Slide` collections, themes (`catppuccin-mocha`), transitions (`fade`, `wipe`, `push`), and speaker notes.
- **Layouts:** `title`, `title_content`, `two_column`, `conclusion`.
- **Markdown Interop:** Bidirectional export and import supporting slide delimiters (`---`) and presenter note blocks (`<!-- Notes: ... -->`).

### 3.4 Vector Draw Engine (`src/vector_draw_engine.py`)
- **Primitive Shapes:** `Rectangle` (with rounded corners `rx`, `ry`), `Ellipse`, `Line`, `Connector` (with orthogonal routing and arrow markers), and `TextShape`.
- **Styling & Geometry:** `StrokeStyle` (color, width, dash array), `FillStyle` (opacity, transparency), and 2D affine `Transform` (translate, rotate, scale).
- **SVG Generation:** Standards-compliant W3C SVG vector export.

### 3.5 Math & Formula Engine (`src/math_engine.py`)
- **Symbol Translation:** Pure lookup dictionary translating LaTeX tokens (`\alpha`, `\sum`, `\int`, `\pi`, `\infty`, `\approx`, `\leq`, `\times`, etc.) into Unicode glyphs.
- **Structural Conversion:** LaTeX fractions `\frac{a}{b}` and exponents/subscripts into typographic forms.
- **MathML XML:** Generating W3C `<math>` markup with `<mo>`, `<mn>`, `<mi>`, and `<mrow>` nodes.

### 3.6 Database Engine (`src/database_engine.py`)
- **Relational Model:** Strongly-typed column definitions (`TEXT`, `INTEGER`, `REAL`, `BOOLEAN`, `DATE`, `BLOB`) with primary key validation and nullability constraints.
- **Query Builder:** Fluent query API supporting `.where(col, op, val)` (operators `=`, `!=`, `>`, `<`, `>=`, `<=`, `LIKE`, `IN`), `.order_by(col, descending)`, `.limit(count, offset)`, and `.aggregate(func, col)`.
- **Forms & Reports:** `FormDesign` field specifications and `ReportGenerator` for structured tabular text reports.

### 3.7 GenOffice AI Engine (`src/genoffice_ai.py`)
- **Deterministic Generation:** Prompt-to-document and prompt-to-presentation synthesis generating complete ASTs with structured headings, narrative, tables, and agendas.
- **Summarization:** Executive summary extraction and reading-time estimation.
- **Readability & Style:** Detection of passive voice constructions, sentence length auditing, and Flesch-Kincaid grade level scoring.
- **Template Engine:** Template variable expansion supporting `{{variable}}`, loops `{{#each items}}...{{/each}}`, and conditionals `{{#if condition}}...{{/if}}`.

### 3.8 OnlyOffice Collaboration Engine (`src/onlyoffice_collab.py`)
- **Operational Transformation (OT):** Atomic operations (`insert`, `delete`, `retain`, `format`) with deterministic rebasing against concurrent mutations.
- **Lock Management:** Pessimistic section/block locks with TTL expiry and optimistic concurrency revision control.
- **Threaded Annotations:** `CommentThread` and `CommentReply` trees anchored to document elements.
- **Telemetry:** Real-time user cursor presence tracking.

### 3.9 Craft Bridge (`src/craft_bridge.py`)
- **Native Packages:** WordCraft (`.wcf`), GridCraft (`.gcf`), DeckCraft (`.dcf`), VectorCraft (`.vcf`), and MathCraft (`.mcf`).
- **Format Sniffer:** Unified JSON inspection and signature detection.

---

## 4. CLI Usage Guide (`sigma-office`)

The `sigma-office` binary provides a unified command-line interface:

```bash
# Style & Icon Planner
sigma-office mapping
sigma-office icons mint
sigma-office apply-plan mint
sigma-office verify mint --available available.txt

# Document Writer
sigma-office writer --title "Release Notes" --text "SigmaOS 2.0 Released." --format markdown

# Spreadsheet Calc
sigma-office calc --set A1=100 --set A2=200 --set A3==SUM(A1:A2)

# Presentation Impress
sigma-office impress --title "Keynote 2026" --slides 3

# Vector Draw
sigma-office draw --demo > diagram.svg

# Math Formula
sigma-office math "\sum_{i=1}^n x_i = \alpha"

# Relational Base
sigma-office base --demo

# GenOffice AI Assistant
sigma-office gen --topic "Microkernel Security" --type doc
sigma-office gen --summarize "Long text to summarize..."
sigma-office gen --review "Text to audit for passive voice and grade level."

# Collaboration & OT
sigma-office collab --demo

# Deterministic Self-Tests
sigma-office self-test
```

---

## 5. Quality & Deterministic Verification

All 49 unit tests in `tests/` execute in < 0.15 seconds with 0 network calls and 0 filesystem mutations outside temporary directories, strictly adhering to the **Arrange-Act-Assert (AAA)** and **FIRST** principles.
