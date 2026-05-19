/**
 * SigmaSuite.cpp — Sovereign Productivity Suite
 * SigmaOS Zenith v15.1
 *
 * Maps to: Syllabus-FCIT Unit IV (Office Automation)
 * Implements: SigmaDocs (Word), SigmaSheets (Excel), SigmaSlides (PowerPoint),
 *             SigmaAccess (MS Access) — all as native SigmaOS applications.
 *
 * All apps read/write SovereignFS via the VFS API.
 * File formats: .sdoc (SigmaDocs), .sxls (SigmaSheets), .sppt (SigmaSlides), .sdb (SigmaAccess)
 */
#include "SigmaSuite.h"

namespace Sigma::Suite {

// ═════════════════════════════════════════════════════════════════════════════
// SIGMA DOCS — Word Processor (maps to MS Word syllabus)
// ═════════════════════════════════════════════════════════════════════════════

// ─── Text Formatting ──────────────────────────────────────────────────────────
void SigmaDocs::apply_format(TextRange range, const TextFormat& fmt) {
    // Store format run in document's format table
    FormatRun run;
    run.start      = range.start;
    run.end        = range.end;
    run.bold       = fmt.bold;
    run.italic     = fmt.italic;
    run.underline  = fmt.underline;
    run.font_size  = fmt.font_size;     // in points
    run.font_name  = fmt.font_name;     // e.g., "Inter", "Roboto"
    run.color_rgb  = fmt.color_rgb;     // 0xRRGGBB
    run.align      = fmt.align;         // LEFT, CENTER, RIGHT, JUSTIFY
    run.line_spacing = fmt.line_spacing; // e.g., 1.5
    m_format_table.push(run);
}

void SigmaDocs::set_page_margins(float top_cm, float bottom_cm,
                                  float left_cm, float right_cm) {
    m_page.margin_top    = top_cm;
    m_page.margin_bottom = bottom_cm;
    m_page.margin_left   = left_cm;
    m_page.margin_right  = right_cm;
}

// ─── Tables ──────────────────────────────────────────────────────────────────
TableId SigmaDocs::insert_table(sigma_u32 rows, sigma_u32 cols,
                                 sigma_u32 cursor_position) {
    DocTable tbl;
    tbl.rows = rows;
    tbl.cols = cols;
    tbl.cells = new DocCell[rows * cols]();
    tbl.position = cursor_position;
    tbl.id = ++m_table_counter;
    // Default: equal column widths filling page width
    float col_width = (m_page.width - m_page.margin_left - m_page.margin_right) / (float)cols;
    for (sigma_u32 c = 0; c < cols; c++) tbl.col_widths[c] = col_width;
    m_tables.push(tbl);
    sigma_klog(sigma_printf, "[SigmaDocs] INSERT TABLE %dx%d at pos=%u\n", rows, cols, cursor_position);
    return tbl.id;
}

void SigmaDocs::set_cell(TableId tid, sigma_u32 row, sigma_u32 col, const char* text) {
    DocTable* t = find_table(tid);
    if (!t) return;
    DocCell& cell = t->cells[row * t->cols + col];
    sigma_strncpy(cell.text, text, sizeof(cell.text));
}

void SigmaDocs::merge_cells(TableId tid, sigma_u32 r1, sigma_u32 c1,
                              sigma_u32 r2, sigma_u32 c2) {
    DocTable* t = find_table(tid);
    if (!t) return;
    DocCell& anchor = t->cells[r1 * t->cols + c1];
    anchor.colspan = c2 - c1 + 1;
    anchor.rowspan = r2 - r1 + 1;
}

// ─── Mail Merge ──────────────────────────────────────────────────────────────
int SigmaDocs::mail_merge(const char* template_path,
                           const char* datasource_path,
                           const char* output_dir) {
    // 1. Load template document
    if (!load(template_path)) return SIGMA_SUITE_ERR_FILE;
    // 2. Open data source (SigmaDB query or CSV file)
    MailMergeDataSource ds;
    if (!ds.open(datasource_path)) return SIGMA_SUITE_ERR_DATASOURCE;
    // 3. For each record, substitute field markers and output a document
    sigma_u32 count = 0;
    while (ds.has_next()) {
        MailMergeRecord record = ds.next();
        SigmaDocs output_doc = *this;  // Copy template
        output_doc.substitute_fields(record);
        char out_path[512];
        sigma_printf(out_path, sizeof(out_path), "%s/merged_%04u.sdoc", output_dir, count++);
        output_doc.save(out_path);
    }
    sigma_klog(sigma_printf, "[SigmaDocs] Mail merge: %u documents generated\n", count);
    return SIGMA_SUITE_OK;
}

void SigmaDocs::substitute_fields(const MailMergeRecord& rec) {
    // Replace «FieldName» markers in document text with record values
    for (sigma_u32 i = 0; i < rec.field_count; i++) {
        char marker[128];
        sigma_printf(marker, sizeof(marker), "<<%s>>", rec.fields[i].name);
        replace_all_text(marker, rec.fields[i].value);
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// SIGMA SHEETS — Spreadsheet (maps to MS Excel syllabus)
// ═════════════════════════════════════════════════════════════════════════════

// ─── Data Sorting ─────────────────────────────────────────────────────────────
void SigmaSheets::sort_range(CellRange range, sigma_u32 key_col, bool ascending) {
    sigma_u32 n = range.row_end - range.row_start;
    // Insertion sort on rows by key_col value
    for (sigma_u32 i = 1; i < n; i++) {
        sigma_u32 j = i;
        while (j > 0) {
            Cell& a = get_cell(range.row_start + j - 1, key_col);
            Cell& b = get_cell(range.row_start + j,     key_col);
            bool swap = ascending ? (cell_compare(a, b) > 0)
                                  : (cell_compare(a, b) < 0);
            if (!swap) break;
            swap_rows(range.row_start + j - 1, range.row_start + j,
                      range.col_start, range.col_end);
            j--;
        }
    }
}

// ─── Data Filtering (AutoFilter) ──────────────────────────────────────────────
void SigmaSheets::apply_filter(sigma_u32 header_row, sigma_u32 col,
                                FilterCondition cond) {
    ActiveFilter f;
    f.col = col;
    f.cond = cond;
    m_filters.push(f);
    // Mark filtered rows as hidden
    for (sigma_u32 r = header_row + 1; r < m_row_count; r++) {
        Cell& c = get_cell(r, col);
        bool match = eval_filter(c, cond);
        m_rows[r].hidden = !match;
    }
}

// ─── Pivot Tables ─────────────────────────────────────────────────────────────
PivotTable SigmaSheets::create_pivot(CellRange source, sigma_u32 row_field,
                                      sigma_u32 col_field, sigma_u32 value_field,
                                      AggFunc agg) {
    PivotTable pt;
    pt.source      = source;
    pt.row_field   = row_field;
    pt.col_field   = col_field;
    pt.value_field = value_field;
    pt.agg         = agg;  // SUM, COUNT, AVG, MAX, MIN

    // Collect unique row/col labels
    for (sigma_u32 r = source.row_start + 1; r <= source.row_end; r++) {
        if (!m_rows[r].hidden) {
            pt.add_row_label(get_cell(r, row_field));
            pt.add_col_label(get_cell(r, col_field));
        }
    }

    // Compute aggregate values
    for (sigma_u32 ri = 0; ri < pt.row_count; ri++) {
        for (sigma_u32 ci = 0; ci < pt.col_count; ci++) {
            pt.values[ri][ci] = compute_agg(source, row_field, pt.row_labels[ri],
                                             col_field, pt.col_labels[ci],
                                             value_field, agg);
        }
    }
    return pt;
}

// ─── Built-in Formulas ────────────────────────────────────────────────────────
double SigmaSheets::formula_sum(CellRange r)    { return aggregate(r, AggFunc::SUM); }
double SigmaSheets::formula_avg(CellRange r)    { return aggregate(r, AggFunc::AVG); }
double SigmaSheets::formula_max(CellRange r)    { return aggregate(r, AggFunc::MAX); }
double SigmaSheets::formula_min(CellRange r)    { return aggregate(r, AggFunc::MIN); }
sigma_u32 SigmaSheets::formula_count(CellRange r) { return (sigma_u32)aggregate(r, AggFunc::COUNT); }

double SigmaSheets::formula_vlookup(const char* val, CellRange table,
                                     sigma_u32 col_idx, bool exact) {
    for (sigma_u32 r = table.row_start; r <= table.row_end; r++) {
        if (sigma_strcmp(get_cell(r, table.col_start).text, val) == 0)
            return get_cell(r, table.col_start + col_idx - 1).number;
    }
    return SIGMA_SHEETS_ERR_NA; // #N/A
}

// ═════════════════════════════════════════════════════════════════════════════
// SIGMA SLIDES — Presentation Software (maps to MS PowerPoint syllabus)
// ═════════════════════════════════════════════════════════════════════════════

SlideId SigmaSlides::add_slide(SlideLayout layout) {
    Slide s;
    s.id      = ++m_slide_counter;
    s.layout  = layout;
    s.bg_color = 0x1A1A2E; // SigmaOS dark theme default
    m_slides.push(s);
    return s.id;
}

void SigmaSlides::add_text_box(SlideId sid, const char* text,
                                float x, float y, float w, float h,
                                const TextFormat& fmt) {
    Slide* s = find_slide(sid);
    if (!s) return;
    TextBox tb;
    sigma_strncpy(tb.text, text, sizeof(tb.text));
    tb.x = x; tb.y = y; tb.w = w; tb.h = h;
    tb.format = fmt;
    s->elements.push(SlideElement{ .type=ElemType::TEXT_BOX, .text_box=tb });
}

void SigmaSlides::add_image(SlideId sid, const char* image_path,
                             float x, float y, float w, float h) {
    Slide* s = find_slide(sid);
    if (!s) return;
    ImageElement img;
    sigma_strncpy(img.path, image_path, sizeof(img.path));
    img.x = x; img.y = y; img.w = w; img.h = h;
    s->elements.push(SlideElement{ .type=ElemType::IMAGE, .image=img });
}

void SigmaSlides::add_video(SlideId sid, const char* video_path,
                             float x, float y, float w, float h, bool autoplay) {
    Slide* s = find_slide(sid);
    if (!s) return;
    VideoElement vid;
    sigma_strncpy(vid.path, video_path, sizeof(vid.path));
    vid.x = x; vid.y = y; vid.w = w; vid.h = h;
    vid.autoplay = autoplay;
    s->elements.push(SlideElement{ .type=ElemType::VIDEO, .video=vid });
}

void SigmaSlides::set_transition(SlideId sid, Transition t, float duration_s) {
    Slide* s = find_slide(sid);
    if (!s) return;
    s->transition = t;
    s->transition_duration = duration_s;
}

void SigmaSlides::set_animation(SlideId sid, sigma_u32 elem_idx,
                                 Animation anim, float delay_s) {
    Slide* s = find_slide(sid);
    if (!s || elem_idx >= s->elements.size()) return;
    s->elements[elem_idx].animation = anim;
    s->elements[elem_idx].anim_delay = delay_s;
}

int SigmaSlides::export_pdf(const char* output_path) {
    // Render each slide to PDF page via SigmaRenderEngine
    SigmaRenderEngine renderer;
    for (sigma_u32 i = 0; i < m_slides.size(); i++) {
        renderer.add_page(m_slides[i]);
    }
    return renderer.write_pdf(output_path);
}

// ═════════════════════════════════════════════════════════════════════════════
// SIGMA ACCESS — Database Frontend (maps to MS Access syllabus)
// ═════════════════════════════════════════════════════════════════════════════

int SigmaAccess::open_database(const char* path) {
    // Open or create a SigmaDB file-based database
    if (!m_db.open(path)) {
        // Create new .sdb file with SovereignFS
        if (!m_db.create(path)) return SIGMA_SUITE_ERR_FILE;
    }
    sigma_klog(sigma_printf, "[SigmaAccess] Opened database: %s\n", path);
    return SIGMA_SUITE_OK;
}

int SigmaAccess::run_query(const char* sql, ResultSet* out) {
    // Parse and execute SQL via SigmaDB
    QueryParser parser(sql);
    ParsedQuery q = parser.parse();
    if (!q.valid) {
        sigma_klog(sigma_printf, "[SigmaAccess] Query parse error: %s\n", sql);
        return SIGMA_SUITE_ERR_SQL;
    }
    *out = m_db.execute(q);
    sigma_klog(sigma_printf, "[SigmaAccess] Query returned %u rows\n", out->row_count);
    return SIGMA_SUITE_OK;
}

void SigmaAccess::create_form(const char* table_name, FormLayout layout) {
    // Auto-generate a data entry form for the given table
    TableSchema schema = m_db.get_schema(table_name);
    Form form;
    form.table = table_name;
    for (sigma_u32 i = 0; i < schema.column_count; i++) {
        FormField f;
        f.column = schema.columns[i];
        f.input_type = (schema.columns[i].type == ColType::BOOLEAN) ?
                       InputType::CHECKBOX : InputType::TEXT;
        form.fields.push(f);
    }
    m_forms.push(form);
}

} // namespace Sigma::Suite
