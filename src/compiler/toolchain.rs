// SigmaOS Compiler — Self-Hosting Toolchain Orchestrator
// Coordinates the full compilation pipeline: source → tokens → AST → typed IR → assembly.
// Designed to bootstrap: the Sigma compiler can compile itself once the language
// is expressive enough to represent its own source.

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

use super::codegen::CodeGenerator;
use super::lexer::Lexer;
use super::parser::Parser;
use super::typechecker::TypeChecker;

// ─── Compilation Pipeline ───────────────────────────────────────────────────

/// Compilation error with phase information.
#[derive(Debug, Clone)]
pub struct CompileError {
    pub phase: CompilePhase,
    pub message: String,
    pub file: String,
    pub line: usize,
    pub column: usize,
}

/// Which compiler phase produced the error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompilePhase {
    Lexing,
    Parsing,
    TypeChecking,
    CodeGeneration,
    Linking,
}

/// Compilation options.
#[derive(Debug, Clone)]
pub struct CompileOptions {
    /// Input source files.
    pub sources: Vec<SourceFile>,
    /// Output file path.
    pub output: String,
    /// Optimization level: 0 = none, 1 = basic, 2 = aggressive.
    pub opt_level: u32,
    /// Emit debug info.
    pub debug: bool,
    /// Target architecture.
    pub target: CompileTarget,
    /// Emit assembly instead of object file.
    pub emit_asm: bool,
    /// Dry run — parse and typecheck only, no codegen.
    pub check_only: bool,
    /// Verbose output.
    pub verbose: bool,
}

impl Default for CompileOptions {
    fn default() -> Self {
        Self {
            sources: Vec::new(),
            output: String::from("a.out"),
            opt_level: 0,
            debug: false,
            target: CompileTarget::X86_64,
            emit_asm: false,
            check_only: false,
            verbose: false,
        }
    }
}

/// A source file to compile.
#[derive(Debug, Clone)]
pub struct SourceFile {
    pub path: String,
    pub content: String,
}

/// Supported compilation targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompileTarget {
    X86_64,
    Aarch64,
    RiscV64,
    Wasm32,
}

impl CompileTarget {
    pub fn triple(&self) -> &'static str {
        match self {
            CompileTarget::X86_64 => "x86_64-sigmaos-elf",
            CompileTarget::Aarch64 => "aarch64-sigmaos-elf",
            CompileTarget::RiscV64 => "riscv64gc-sigmaos-elf",
            CompileTarget::Wasm32 => "wasm32-sigmaos-wasi",
        }
    }
}

/// Result of a compilation.
#[derive(Debug, Clone)]
pub struct CompileResult {
    pub success: bool,
    pub errors: Vec<CompileError>,
    pub warnings: Vec<CompileWarning>,
    pub assembly: Option<String>,
    pub stats: CompileStats,
}

/// Compilation warnings.
#[derive(Debug, Clone)]
pub struct CompileWarning {
    pub message: String,
    pub file: String,
    pub line: usize,
}

/// Compilation statistics.
#[derive(Debug, Clone)]
pub struct CompileStats {
    pub files_processed: usize,
    pub tokens_lexed: usize,
    pub ast_nodes: usize,
    pub assembly_lines: usize,
    pub errors: usize,
    pub warnings: usize,
}

// ─── Toolchain ──────────────────────────────────────────────────────────────

/// The SigmaOS self-hosting compiler toolchain.
///
/// Pipeline: Source → Lexer → Parser → TypeChecker → CodeGenerator → Assembly
///
/// The toolchain is designed for self-hosting: once the Sigma language is complete
/// enough, this compiler can compile its own source code, achieving bootstrap.
pub struct SigmaToolchain {
    options: CompileOptions,
}

impl SigmaToolchain {
    pub fn new(options: CompileOptions) -> Self {
        Self { options }
    }

    /// Run the full compilation pipeline.
    pub fn compile(&self) -> CompileResult {
        let mut all_errors = Vec::new();
        let all_warnings = Vec::new();
        let mut total_tokens = 0;
        let mut total_ast_nodes = 0;
        let mut final_assembly = String::new();

        for source in &self.options.sources {
            if self.options.verbose {
                // In a real implementation, this would log to stderr
            }

            // Phase 1: Lexing
            let mut lexer = Lexer::new(&source.content, &source.path);
            let tokens = match lexer.tokenize() {
                Ok(tokens) => {
                    total_tokens += tokens.len();
                    tokens
                }
                Err(lex_errors) => {
                    for err in lex_errors {
                        all_errors.push(CompileError {
                            phase: CompilePhase::Lexing,
                            message: err.message,
                            file: source.path.clone(),
                            line: err.span.line,
                            column: err.span.column,
                        });
                    }
                    continue;
                }
            };

            // Phase 2: Parsing
            let mut parser = Parser::new(tokens);
            let program = match parser.parse_program() {
                Ok(prog) => {
                    total_ast_nodes += Self::count_ast_nodes(&prog);
                    prog
                }
                Err(parse_error) => {
                    all_errors.push(CompileError {
                        phase: CompilePhase::Parsing,
                        message: parse_error.message,
                        file: source.path.clone(),
                        line: parse_error.span.line,
                        column: parse_error.span.column,
                    });
                    continue;
                }
            };

            // Phase 3: Type Checking
            let mut checker = TypeChecker::new();
            if let Err(type_errors) = checker.check_program(&program) {
                for err in type_errors {
                    all_errors.push(CompileError {
                        phase: CompilePhase::TypeChecking,
                        message: err.message,
                        file: err.file,
                        line: err.line,
                        column: err.column,
                    });
                }
                // Continue to codegen even with type errors if not strict
                if !self.options.check_only {
                    // In strict mode, we'd stop here
                }
            }

            if self.options.check_only {
                continue;
            }

            // Phase 4: Code Generation
            let mut codegen = CodeGenerator::new();
            let asm_output = codegen.generate(&program);
            let rendered = asm_output.render();
            final_assembly.push_str(&rendered);
        }

        let asm_lines = final_assembly.lines().count();
        let error_count = all_errors.len();

        CompileResult {
            success: all_errors.is_empty(),
            errors: all_errors,
            warnings: all_warnings,
            assembly: if final_assembly.is_empty() {
                None
            } else {
                Some(final_assembly)
            },
            stats: CompileStats {
                files_processed: self.options.sources.len(),
                tokens_lexed: total_tokens,
                ast_nodes: total_ast_nodes,
                assembly_lines: asm_lines,
                errors: error_count,
                warnings: 0,
            },
        }
    }

    /// Quick single-file compilation helper.
    pub fn compile_source(source: &str, filename: &str) -> CompileResult {
        let options = CompileOptions {
            sources: vec![SourceFile {
                path: String::from(filename),
                content: String::from(source),
            }],
            ..CompileOptions::default()
        };
        let toolchain = SigmaToolchain::new(options);
        toolchain.compile()
    }

    /// Check-only mode (parse + typecheck, no codegen).
    pub fn check_source(source: &str, filename: &str) -> CompileResult {
        let options = CompileOptions {
            sources: vec![SourceFile {
                path: String::from(filename),
                content: String::from(source),
            }],
            check_only: true,
            ..CompileOptions::default()
        };
        let toolchain = SigmaToolchain::new(options);
        toolchain.compile()
    }

    fn count_ast_nodes(program: &super::parser::Program) -> usize {
        // Simple heuristic: count items
        program.items.len()
    }
}

/// Toolchain version information.
pub struct ToolchainVersion;

impl ToolchainVersion {
    pub const MAJOR: u32 = 0;
    pub const MINOR: u32 = 1;
    pub const PATCH: u32 = 0;
    pub const CODENAME: &'static str = "Sigma-Bootstrap";

    pub fn version_string() -> String {
        alloc::format!(
            "sigmac {}.{}.{} ({})",
            Self::MAJOR,
            Self::MINOR,
            Self::PATCH,
            Self::CODENAME
        )
    }

    pub fn target_info() -> String {
        alloc::format!(
            "Target: {} | Host: sigmaos-x86_64",
            CompileTarget::X86_64.triple()
        )
    }
}

// ─── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_pipeline() {
        let result = SigmaToolchain::compile_source(
            "fn main() -> i64 { return 42 }",
            "test.sg",
        );
        assert!(result.success);
        assert!(result.assembly.is_some());
        let asm = result.assembly.unwrap();
        assert!(asm.contains("main:"));
        assert!(asm.contains("$42"));
    }

    #[test]
    fn test_check_only() {
        let result = SigmaToolchain::check_source(
            "fn add(a: i32, b: i32) -> i32 { return a }",
            "test.sg",
        );
        assert!(result.success);
        assert!(result.assembly.is_none());
    }

    #[test]
    fn test_lex_error() {
        let result = SigmaToolchain::compile_source(
            "fn main() { let x = `invalid` }",
            "test.sg",
        );
        assert!(!result.success);
        assert_eq!(result.errors[0].phase, CompilePhase::Lexing);
    }

    #[test]
    fn test_multi_function() {
        let result = SigmaToolchain::compile_source(
            "fn square(x: i64) -> i64 { return x * x }\nfn main() -> i64 { return square(5) }",
            "test.sg",
        );
        assert!(result.success);
        let asm = result.assembly.unwrap();
        assert!(asm.contains("square:"));
        assert!(asm.contains("main:"));
        assert!(asm.contains("call square"));
    }

    #[test]
    fn test_version() {
        let ver = ToolchainVersion::version_string();
        assert!(ver.contains("sigmac"));
        assert!(ver.contains("Sigma-Bootstrap"));
    }

    #[test]
    fn test_compile_stats() {
        let result = SigmaToolchain::compile_source("fn main() { return 0 }", "test.sg");
        assert!(result.stats.files_processed == 1);
        assert!(result.stats.tokens_lexed > 0);
    }
}
