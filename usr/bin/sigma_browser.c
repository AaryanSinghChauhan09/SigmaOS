/*
 * =============================================================================
 * Σ SIGMAOS USERSPACE: OMNIWEB BROWSER ENGINE (STUB)
 * =============================================================================
 * Inspired by: SerenityOS LibWeb, Ladybird
 * =============================================================================
 * A lightweight HTML/CSS parsing engine and DOM renderer that interfaces
 * directly with the SigmaOS Compositor.
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"

#define MAX_DOM_NODES 1024

typedef enum {
    NODE_ELEMENT,
    NODE_TEXT
} dom_node_type_t;

typedef struct dom_node {
    dom_node_type_t type;
    char tag_name[32];
    char text_content[256];
    
    struct dom_node* parent;
    struct dom_node* first_child;
    struct dom_node* next_sibling;
    
    /* Layout Box */
    sigma_u32 x, y, width, height;
} dom_node_t;

static dom_node_t dom_arena[MAX_DOM_NODES];
static sigma_u32 dom_alloc_idx = 0;

void browser_init(void) {
    sigma_memset(dom_arena, 0, sizeof(dom_arena));
    sigma_printf("[browser] OmniWeb Engine Initialized\n");
}

dom_node_t* browser_create_element(const char* tag) {
    if (dom_alloc_idx >= MAX_DOM_NODES) return SIGMA_NULL;
    dom_node_t* node = &dom_arena[dom_alloc_idx++];
    node->type = NODE_ELEMENT;
    sigma_strcpy(node->tag_name, tag, 32);
    return node;
}

dom_node_t* browser_create_text(const char* text) {
    if (dom_alloc_idx >= MAX_DOM_NODES) return SIGMA_NULL;
    dom_node_t* node = &dom_arena[dom_alloc_idx++];
    node->type = NODE_TEXT;
    sigma_strcpy(node->text_content, text, 256);
    return node;
}

void browser_append_child(dom_node_t* parent, dom_node_t* child) {
    if (!parent || !child) return;
    child->parent = parent;
    
    if (!parent->first_child) {
        parent->first_child = child;
    } else {
        dom_node_t* curr = parent->first_child;
        while (curr->next_sibling) {
            curr = curr->next_sibling;
        }
        curr->next_sibling = child;
    }
}

/* Very crude simulated parser */
dom_node_t* browser_parse_html(const char* html_data) {
    sigma_printf("[browser] Parsing HTML payload...\n");
    
    dom_node_t* html = browser_create_element("html");
    dom_node_t* body = browser_create_element("body");
    dom_node_t* h1 = browser_create_element("h1");
    dom_node_t* text = browser_create_text("Welcome to SigmaOS OmniWeb!");
    
    browser_append_child(html, body);
    browser_append_child(body, h1);
    browser_append_child(h1, text);
    
    return html;
}

void browser_render_layout(dom_node_t* root, sigma_u32 window_id) {
    sigma_printf("[browser] Rendering DOM layout to Window ID %u...\n", window_id);
    
    /* Traverse DOM and simulate calculating bounding boxes */
    sigma_printf("[browser]  -> Box [html]: 0,0 1024x768\n");
    sigma_printf("[browser]  -> Box [body]: 8,8 1008x752\n");
    sigma_printf("[browser]  -> Box [h1]  : 8,8 1008x48\n");
    sigma_printf("[browser]  -> Text Render: 'Welcome to SigmaOS OmniWeb!'\n");
    
    /* In reality, we issue draw calls to the Compositor's back-buffer here */
}
