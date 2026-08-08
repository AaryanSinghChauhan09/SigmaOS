# SigmaOffice

**SigmaOffice** is the SigmaOS alternative to the Microsoft Office Suite.

## Features

### SigmaWriter (Word Processor)
Document editing with advanced formatting (Word alternative):
- Rich text formatting (bold, italic, underline, fonts, colors)
- Paragraph styles and templates
- Tables, images, and embedded objects
- Track changes and collaboration
- Export to PDF, ODT, DOCX
- Auto-save and version history
- Spell check and grammar check
- Mail merge functionality

### SigmaSheet (Spreadsheets)
Data manipulation, formulas, and charting (Excel alternative):
- Cell formatting and conditional formatting
- 500+ built-in functions (math, statistical, financial, text, date/time)
- Pivot tables and data analysis
- Charts and graphs (line, bar, pie, scatter, area)
- Data validation and protection
- Macro support with SigmaScript
- Import/export CSV, XLSX, ODS
- Real-time collaboration

### SigmaPresent (Presentations)
Slide decks and presentation tools (PowerPoint alternative):
- Slide templates and themes
- Transitions and animations
- Speaker notes and presenter view
- Embedded media (audio, video)
- Export to PDF, PPTX, ODP
- Collaboration and commenting
- Presentation mode with laser pointer

## Architecture

```
SigmaOffice Suite
   ├─ SigmaWriter (document engine)
   │   ├─ Rich text parser/renderer
   │   ├─ Formatting engine
   │   └─ Collaboration backend
   ├─ SigmaSheet (spreadsheet engine)
   │   ├─ Calculation engine
   │   ├─ Chart renderer
   │   └─ Data analysis tools
   └─ SigmaPresent (presentation engine)
       ├─ Slide editor
       ├─ Animation engine
       └─ Presenter mode
```

## File Formats

| Application | Native Format | Import Formats | Export Formats |
|---|---|---|---|
| SigmaWriter | .swriter | .docx, .odt, .txt, .rtf | .pdf, .docx, .odt, .html |
| SigmaSheet | .ssheet | .xlsx, .ods, .csv | .pdf, .xlsx, .ods, .csv |
| SigmaPresent | .spresent | .pptx, .odp | .pdf, .pptx, .odp |

## API Interface

```c
// SigmaWriter API
int sigma_writer_new_document(writer_doc_t *doc);
int sigma_writer_open(const char *path, writer_doc_t *doc);
int sigma_writer_save(const writer_doc_t *doc, const char *path);
int sigma_writer_export_pdf(const writer_doc_t *doc, const char *path);
int sigma_writer_apply_format(writer_doc_t *doc, const char *text, format_t fmt);

// SigmaSheet API
int sigma_sheet_new_workbook(sheet_workbook_t *wb);
int sigma_sheet_open(const char *path, sheet_workbook_t *wb);
int sigma_sheet_save(const sheet_workbook_t *wb, const char *path);
int sigma_sheet_set_cell(sheet_workbook_t *wb, const char *cell, const char *value);
int sigma_sheet_get_cell(sheet_workbook_t *wb, const char *cell, char *value);
int sigma_sheet_evaluate_formula(sheet_workbook_t *wb, const char *cell, char *result);
int sigma_sheet_create_chart(sheet_workbook_t *wb, const char *range, chart_type_t type);

// SigmaPresent API
int sigma_present_new_deck(present_deck_t *deck);
int sigma_present_open(const char *path, present_deck_t *deck);
int sigma_present_save(const present_deck_t *deck, const char *path);
int sigma_present_add_slide(present_deck_t *deck, slide_t *slide);
int sigma_present_export_pdf(const present_deck_t *deck, const char *path);
int sigma_present_start_present(const present_deck_t *deck);
```

## Integration

- **SigmaFS Integration**: Auto-save to SovereignFS snapshots
- **SigmaAI Integration**: AI-powered writing assistance, formula suggestions, design recommendations
- **SigmaCloud Integration**: Real-time collaboration via SovereignNet
- **Zenith Desktop Integration**: Native Zenith UI components

## Roadmap

- [x] Architecture design and component specification
- [ ] SigmaWriter rich text engine implementation
- [ ] SigmaSheet calculation engine implementation
- [ ] SigmaPresent slide editor implementation
- [ ] File format import/export (DOCX, XLSX, PPTX)
- [ ] Collaboration backend
- [ ] AI-powered features (writing assistant, formula suggestions)
- [ ] SigmaScript macro language
- [ ] Mobile versions (SigmaOS Mobile)
- [ ] Web versions (SigmaOS Cloud)

## Related Modules

- [`desktop/zenith_accessibility.rs`](../../desktop/zenith_accessibility.rs) — Accessibility features
- [`modules/core/fs`](../../modules/core/fs/README.md) — Filesystem integration
- [`modules/core/net`](../../modules/core/net/README.md) — Network collaboration
