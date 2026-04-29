/**
 * SigmaOS Code Snippet Manager Shard
 * USP/Logic: Save snippets directly from tutorials into a searchable library.
 */

class CodeSnippetManager {
    constructor() {
        this.shardId = "S" + "60_code_snippet_manager.js".split('_')[0] + "_CodeSnippetManager";
        this.active = false;
        
        console.log(`Σ://INIT> ${this.shardId} Initializing: Code Snippet Manager...`);
        this.init();
    }

    init() {
        window.addEventListener('sigma.core.boot', () => {
            this.active = true;
            console.log(`Σ://USP> ${this.shardId} Online. Save snippets directly from tutorials into a searchable library.`);
        });
    }
}

window.SigmaCodeSnippetManager = new CodeSnippetManager();
