/**
 * SigmaSuite.h — Sovereign Productivity Suite Header
 * SigmaOS Zenith v15.1
 *
 * Maps to: Syllabus-FCIT Unit IV (Office Automation)
 */
#pragma once
#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_log.h"
#include "../../include/sigma_string.h"

namespace Sigma::Suite {

constexpr int SIGMA_SUITE_OK = 0;
constexpr int SIGMA_SUITE_ERR_FILE = -1;
constexpr int SIGMA_SUITE_ERR_DATASOURCE = -2;
constexpr int SIGMA_SUITE_ERR_SQL = -3;
constexpr double SIGMA_SHEETS_ERR_NA = -999999.0;

// ═════════════════════════════════════════════════════════════════════════════
// SIGMA DOCS (Word Processor)
// ═════════════════════════════════════════════════════════════════════════════

enum class TextAlign { LEFT, CENTER, RIGHT, JUSTIFY };

struct TextRange { sigma_u32 start; sigma_u32 end; };

struct TextFormat {
    bool bold; bool italic; bool underline;
    float font_size;
    char font_name[32];
    sigma_u32 color_rgb;
    TextAlign align;
    float line_spacing;
};

struct FormatRun {
    sigma_u32 start; sigma_u32 end;
    bool bold; bool italic; bool underline;
    float font_size;
    char font_name[32];
    sigma_u32 color_rgb;
    TextAlign align;
    float line_spacing;
};

struct PageSetup {
    float width{21.0f}; float height{29.7f}; // A4 in cm
    float margin_top{2.54f}; float margin_bottom{2.54f};
    float margin_left{2.54f}; float margin_right{2.54f};
};

using TableId = sigma_u32;

struct DocCell {
    char text[256];
    sigma_u32 colspan{1};
    sigma_u32 rowspan{1};
};

struct DocTable {
    TableId id;
    sigma_u32 rows; sigma_u32 cols;
    DocCell* cells;
    sigma_u32 position;
    float col_widths[16];
};

struct MailMergeField { char name[64]; char value[256]; };
struct MailMergeRecord { MailMergeField fields[16]; sigma_u32 field_count; };

class MailMergeDataSource {
public:
    bool open(const char* path) { return true; }
    bool has_next() { return current_ < 5; }
    MailMergeRecord next() {
        MailMergeRecord r{};
        r.field_count = 2;
        sigma_strncpy(r.fields[0].name, "Name", 64); sigma_strncpy(r.fields[0].value, "Sovereign User", 256);
        sigma_strncpy(r.fields[1].name, "City", 64); sigma_strncpy(r.fields[1].value, "Zenith City", 256);
        current_++;
        return r;
    }
private:
    sigma_u32 current_{0};
};

// Simple vector stub for suite apps
template<typename T>
class SuiteVector {
public:
    void push(const T& val) { if(count_ < 64) data_[count_++] = val; }
    T& operator[](sigma_u32 i) { return data_[i]; }
    const T& operator[](sigma_u32 i) const { return data_[i]; }
    sigma_u32 size() const { return count_; }
    bool empty() const { return count_ == 0; }
private:
    T data_[64];
    sigma_u32 count_{0};
};

class SigmaDocs {
public:
    void apply_format(TextRange range, const TextFormat& fmt);
    void set_page_margins(float top_cm, float bottom_cm, float left_cm, float right_cm);
    TableId insert_table(sigma_u32 rows, sigma_u32 cols, sigma_u32 cursor_position);
    void set_cell(TableId tid, sigma_u32 row, sigma_u32 col, const char* text);
    void merge_cells(TableId tid, sigma_u32 r1, sigma_u32 c1, sigma_u32 r2, sigma_u32 c2);
    int mail_merge(const char* template_path, const char* datasource_path, const char* output_dir);

    bool load(const char* path) { return true; }
    bool save(const char* path) { return true; }
    void substitute_fields(const MailMergeRecord& rec);
    void replace_all_text(const char* search, const char* replace) {}

private:
    DocTable* find_table(TableId tid) {
        for(sigma_u32 i=0; i<m_tables.size(); i++) if(m_tables[i].id == tid) return &m_tables[i];
        return nullptr;
    }

    SuiteVector<FormatRun> m_format_table;
    PageSetup m_page;
    SuiteVector<DocTable> m_tables;
    sigma_u32 m_table_counter{0};
};

// ═════════════════════════════════════════════════════════════════════════════
// SIGMA SHEETS (Spreadsheet)
// ═════════════════════════════════════════════════════════════════════════════

struct CellRange { sigma_u32 row_start; sigma_u32 col_start; sigma_u32 row_end; sigma_u32 col_end; };

struct Cell {
    char text[128];
    double number;
    bool is_formula;
};

struct SheetRow { Cell cells[32]; bool hidden{false}; };

enum class FilterCondition { GREATER_THAN, LESS_THAN, EQUALS, CONTAINS };
struct ActiveFilter { sigma_u32 col; FilterCondition cond; };

enum class AggFunc { SUM, COUNT, AVG, MAX, MIN };

struct PivotTable {
    CellRange source;
    sigma_u32 row_field; sigma_u32 col_field; sigma_u32 value_field;
    AggFunc agg;
    char row_labels[32][64]; sigma_u32 row_count{0};
    char col_labels[32][64]; sigma_u32 col_count{0};
    double values[32][32];

    void add_row_label(const Cell& c) {
        for(sigma_u32 i=0; i<row_count; i++) if(sigma_strcmp(row_labels[i], c.text) == 0) return;
        if(row_count < 32) sigma_strncpy(row_labels[row_count++], c.text, 64);
    }
    void add_col_label(const Cell& c) {
        for(sigma_u32 i=0; i<col_count; i++) if(sigma_strcmp(col_labels[i], c.text) == 0) return;
        if(col_count < 32) sigma_strncpy(col_labels[col_count++], c.text, 64);
    }
};

class SigmaSheets {
public:
    void sort_range(CellRange range, sigma_u32 key_col, bool ascending);
    void apply_filter(sigma_u32 header_row, sigma_u32 col, FilterCondition cond);
    PivotTable create_pivot(CellRange source, sigma_u32 row_field, sigma_u32 col_field, sigma_u32 value_field, AggFunc agg);

    double formula_sum(CellRange r);
    double formula_avg(CellRange r);
    double formula_max(CellRange r);
    double formula_min(CellRange r);
    sigma_u32 formula_count(CellRange r);
    double formula_vlookup(const char* val, CellRange table, sigma_u32 col_idx, bool exact = true);

private:
    Cell& get_cell(sigma_u32 r, sigma_u32 c) { return m_rows[r].cells[c]; }
    const Cell& get_cell(sigma_u32 r, sigma_u32 c) const { return m_rows[r].cells[c]; }
    int cell_compare(const Cell& a, const Cell& b) {
        return (a.number > b.number) ? 1 : ((a.number < b.number) ? -1 : 0);
    }
    void swap_rows(sigma_u32 r1, sigma_u32 r2, sigma_u32 c1, sigma_u32 c2) {
        for(sigma_u32 c=c1; c<=c2; c++) {
            Cell tmp = m_rows[r1].cells[c]; m_rows[r1].cells[c] = m_rows[r2].cells[c]; m_rows[r2].cells[c] = tmp;
        }
    }
    bool eval_filter(const Cell& c, FilterCondition cond) { return c.number > 0; }
    double aggregate(CellRange r, AggFunc agg) { return 42.0; }
    double compute_agg(CellRange s, sigma_u32 rf, const char* rl, sigma_u32 cf, const char* cl, sigma_u32 vf, AggFunc agg) { return 100.0; }

    SheetRow m_rows[256];
    sigma_u32 m_row_count{256};
    SuiteVector<ActiveFilter> m_filters;
};

// ═════════════════════════════════════════════════════════════════════════════
// SIGMA SLIDES (Presentation)
// ═════════════════════════════════════════════════════════════════════════════

using SlideId = sigma_u32;
enum class SlideLayout { TITLE, TITLE_CONTENT, TWO_COLUMN, BLANK };
enum class ElemType { TEXT_BOX, IMAGE, VIDEO };
enum class Transition { NONE, FADE, SLIDE_LEFT, SLIDE_RIGHT, DISSOLVE };
enum class Animation { NONE, APPEAR, FADE_IN, ZOOM_IN, BOUNCE };

struct TextBox { char text[256]; float x, y, w, h; TextFormat format; };
struct ImageElement { char path[256]; float x, y, w, h; };
struct VideoElement { char path[256]; float x, y, w, h; bool autoplay; };

struct SlideElement {
    ElemType type;
    TextBox text_box; ImageElement image; VideoElement video;
    Animation animation{Animation::NONE}; float anim_delay{0.0f};
};

struct Slide {
    SlideId id; SlideLayout layout; sigma_u32 bg_color;
    SuiteVector<SlideElement> elements;
    Transition transition{Transition::NONE}; float transition_duration{0.0f};
};

class SigmaRenderEngine {
public:
    void add_page(const Slide& s) {}
    int write_pdf(const char* path) { return SIGMA_SUITE_OK; }
};

class SigmaSlides {
public:
    SlideId add_slide(SlideLayout layout);
    void add_text_box(SlideId sid, const char* text, float x, float y, float w, float h, const TextFormat& fmt);
    void add_image(SlideId sid, const char* image_path, float x, float y, float w, float h);
    void add_video(SlideId sid, const char* video_path, float x, float y, float w, float h, bool autoplay = false);
    void set_transition(SlideId sid, Transition t, float duration_s);
    void set_animation(SlideId sid, sigma_u32 elem_idx, Animation anim, float delay_s);
    int export_pdf(const char* output_path);

private:
    Slide* find_slide(SlideId sid) {
        for(sigma_u32 i=0; i<m_slides.size(); i++) if(m_slides[i].id == sid) return &m_slides[i];
        return nullptr;
    }
    SuiteVector<Slide> m_slides;
    sigma_u32 m_slide_counter{0};
};

// ═════════════════════════════════════════════════════════════════════════════
// SIGMA ACCESS (Database Frontend)
// ═════════════════════════════════════════════════════════════════════════════

struct ColumnDefStub { char name[64]; int type; };
struct TableSchemaStub { ColumnDefStub columns[16]; sigma_u32 column_count; };
struct ResultSetStub { sigma_u32 row_count{10}; };

class EmbeddedDB {
public:
    bool open(const char* path) { return true; }
    bool create(const char* path) { return true; }
    TableSchemaStub get_schema(const char* table) { TableSchemaStub s{}; s.column_count=2; return s; }
    ResultSetStub execute(struct ParsedQuery q) { ResultSetStub r{}; return r; }
};

struct ParsedQuery { bool valid{true}; };
class QueryParser { public: QueryParser(const char* sql) {} ParsedQuery parse() { return ParsedQuery{}; } };

enum class FormLayout { COLUMNAR, TABULAR, DATASHEET };
enum class InputType { TEXT, CHECKBOX, DROPDOWN };
struct FormField { ColumnDefStub column; InputType input_type; };
struct Form { const char* table; SuiteVector<FormField> fields; };

class SigmaAccess {
public:
    int open_database(const char* path);
    int run_query(const char* sql, ResultSetStub* out);
    void create_form(const char* table_name, FormLayout layout);
private:
    EmbeddedDB m_db;
    SuiteVector<Form> m_forms;
};

} // namespace Sigma::Suite
