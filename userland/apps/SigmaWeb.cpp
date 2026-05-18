/**
 * SigmaWeb.cpp — SigmaWeb Runtime Engine
 * SigmaOS Zenith v15.1
 *
 * Maps to: Syllabus-WebProgramming (HTML5, CSS, JavaScript, PHP)
 * Implements: HTML5 parser, CSS layout engine, JS bridge, PHP-FPM bridge
 */
#include "SigmaWeb.h"

namespace Sigma::Web {

// ─── HTML5 Parser ─────────────────────────────────────────────────────────────

DOMNode* SigmaWebParser::parse_html(const char* html, sigma_usize len) {
    DOMNode* root = alloc_node(NodeType::DOCUMENT);
    ParserState state = ParserState::INITIAL;
    DOMNode* current = root;
    const char* p = html;
    const char* end = html + len;

    while (p < end) {
        switch (state) {
            case ParserState::INITIAL:
                if (*p == '<') {
                    p++;
                    if (*p == '/') { state = ParserState::CLOSING_TAG; p++; }
                    else if (*p == '!') { state = ParserState::COMMENT; }
                    else state = ParserState::OPENING_TAG;
                } else {
                    // Text node
                    DOMNode* txt = alloc_node(NodeType::TEXT);
                    const char* start = p;
                    while (p < end && *p != '<') p++;
                    txt->text_content = intern_string(start, (sigma_usize)(p - start));
                    append_child(current, txt);
                }
                break;
            case ParserState::OPENING_TAG: {
                // Read tag name
                const char* tag_start = p;
                while (p < end && *p != ' ' && *p != '>' && *p != '/') p++;
                DOMNode* node = alloc_node(NodeType::ELEMENT);
                node->tag_name = intern_string(tag_start, (sigma_usize)(p - tag_start));
                // Parse attributes
                while (p < end && *p != '>' && !(*p=='/' && *(p+1)=='>')) {
                    while (*p == ' ') p++; // skip whitespace
                    if (*p == '>' || *p == '/') break;
                    parse_attribute(p, end, node);
                }
                append_child(current, node);
                if (*p == '/') { p += 2; } // self-closing />
                else { p++; // skip >
                    if (!is_void_element(node->tag_name)) current = node;
                }
                state = ParserState::INITIAL;
                break;
            }
            case ParserState::CLOSING_TAG:
                // Skip to >
                while (p < end && *p != '>') p++;
                p++; // skip >
                if (current->parent) current = current->parent;
                state = ParserState::INITIAL;
                break;
            default: p++; break;
        }
    }
    return root;
}

// ─── Semantic HTML5 Tags → ARIA roles ─────────────────────────────────────────
const char* SigmaWebParser::semantic_aria_role(const char* tag) {
    // Semantic tag → ARIA landmark mapping
    struct { const char* tag; const char* role; } map[] = {
        {"header",  "banner"},     {"nav",     "navigation"},
        {"main",    "main"},       {"article", "article"},
        {"section", "region"},     {"aside",   "complementary"},
        {"footer",  "contentinfo"},{"form",    "form"},
        {"button",  "button"},     {"a",       "link"},
        {nullptr, nullptr}
    };
    for (int i = 0; map[i].tag; i++)
        if (sigma_strcmp(tag, map[i].tag) == 0) return map[i].role;
    return "generic";
}

// ─── CSS Layout Engine ────────────────────────────────────────────────────────

void SigmaCSSEngine::apply_stylesheet(const char* css, DOMNode* root) {
    // Parse CSS rules
    CSSParser parser(css);
    while (parser.has_next()) {
        CSSRule rule = parser.next_rule();
        // Select matching DOM nodes
        DOMNodeList matches = query_selector_all(root, rule.selector);
        for (sigma_u32 i = 0; i < matches.count; i++) {
            apply_rule(matches.nodes[i], rule);
        }
    }
}

LayoutBox SigmaCSSEngine::compute_layout(DOMNode* node, const BoxConstraints& parent) {
    LayoutBox box;
    ComputedStyle& style = node->computed_style;

    // Display type determines layout mode
    if (style.display == Display::FLEX)        return layout_flex(node, parent);
    if (style.display == Display::GRID)        return layout_grid(node, parent);
    if (style.display == Display::BLOCK)       return layout_block(node, parent);
    if (style.display == Display::INLINE)      return layout_inline(node, parent);
    if (style.display == Display::NONE)        { box.visible = false; return box; }

    return layout_block(node, parent);
}

// ─── JavaScript Bridge ────────────────────────────────────────────────────────

JSRuntime* SigmaJSEngine::create_runtime() {
    // Embed QuickJS for JS execution in SigmaOS
    // QuickJS: small, embeddable, ES2020-compliant JS engine
    JSRuntime* rt = JS_NewRuntime();
    JS_SetMemoryLimit(rt, 64 * 1024 * 1024); // 64MB JS heap
    JS_SetMaxStackSize(rt, 512 * 1024);       // 512KB stack
    return rt;
}

JSValue SigmaJSEngine::eval(JSContext* ctx, const char* code, const char* filename) {
    return JS_Eval(ctx, code, sigma_strlen(code), filename,
                   JS_EVAL_TYPE_GLOBAL | JS_EVAL_FLAG_STRICT);
}

// Web Storage API → SovereignFS
void SigmaJSEngine::register_web_storage_api(JSContext* ctx) {
    // localStorage: persisted to /sigma/web/storage/<origin>/local.json
    // sessionStorage: persisted to /sigma/web/storage/<origin>/session.json
    JSValue storage_proto = JS_NewObject(ctx);
    JS_SetPropertyStr(ctx, storage_proto, "setItem",
        JS_NewCFunction(ctx, js_storage_set_item, "setItem", 2));
    JS_SetPropertyStr(ctx, storage_proto, "getItem",
        JS_NewCFunction(ctx, js_storage_get_item, "getItem", 1));
    JS_SetPropertyStr(ctx, storage_proto, "removeItem",
        JS_NewCFunction(ctx, js_storage_remove_item, "removeItem", 1));
    JS_SetPropertyStr(ctx, JS_GetGlobalObject(ctx), "localStorage", storage_proto);
}

// Web Workers → SigmaOS threads
void SigmaJSEngine::register_web_workers_api(JSContext* ctx) {
    JS_SetPropertyStr(ctx, JS_GetGlobalObject(ctx), "Worker",
        JS_NewCFunction(ctx, js_worker_constructor, "Worker", 1));
}

// WebSockets → SovereignNetStack
void SigmaJSEngine::register_websocket_api(JSContext* ctx) {
    JS_SetPropertyStr(ctx, JS_GetGlobalObject(ctx), "WebSocket",
        JS_NewCFunction(ctx, js_websocket_constructor, "WebSocket", 1));
}

// Geolocation → HAL GPS driver
void SigmaJSEngine::register_geolocation_api(JSContext* ctx) {
    JSValue nav = JS_GetPropertyStr(ctx, JS_GetGlobalObject(ctx), "navigator");
    JS_SetPropertyStr(ctx, nav, "geolocation",
        JS_NewCFunction(ctx, js_geolocation_object, "geolocation", 0));
    JS_FreeValue(ctx, nav);
}

// ─── PHP Bridge ───────────────────────────────────────────────────────────────
int SigmaPHPBridge::execute(const char* script_path, const char* query_string,
                             char* output_buf, sigma_usize output_len) {
    // Fork a PHP-FPM compatible worker process
    PHPWorker worker;
    worker.set_script(script_path);
    worker.set_env("QUERY_STRING", query_string);
    worker.set_env("REQUEST_METHOD", "GET");
    int rc = worker.execute(output_buf, output_len);
    sigma_klog(LOG_DEBUG, "[SigmaWeb] PHP %s → %d bytes\n", script_path, (int)output_len);
    return rc;
}

// ─── HTML5 Application Cache / Service Worker ─────────────────────────────────
int SigmaWebCache::cache_resource(const char* url, const char* data,
                                   sigma_usize len) {
    // Store in /sigma/web/cache/<origin>/<url_hash>
    char cache_path[512];
    sigma_u32 hash = fnv1a_hash(url, sigma_strlen(url));
    sigma_snprintf(cache_path, sizeof(cache_path),
                   "/sigma/web/cache/%08X", hash);
    SigmaFile* f = sigma_fopen(cache_path, "wb");
    if (!f) return -1;
    sigma_fwrite(data, 1, len, f);
    sigma_fclose(f);
    return SIGMA_WEB_OK;
}

} // namespace Sigma::Web
