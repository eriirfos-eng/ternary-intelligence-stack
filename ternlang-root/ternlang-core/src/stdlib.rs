/// StdlibLoader / ModuleResolver — resolves `use` statements into parsed function definitions.
///
/// Two resolution strategies, tried in order:
///   1. Built-in stdlib (embedded at compile time via `include_str!`) — zero filesystem I/O.
///   2. User-defined modules: looks for `<segment>/<segment>.tern` relative to the source file.
///
/// Use `StdlibLoader::resolve()` for quick stdlib-only resolution (backwards-compat API).
/// Use `ModuleResolver::from_source_file(path).resolve(program)` for full module support.
use crate::ast::{Function, Program, Stmt};
use crate::parser::Parser;

// ─── Built-in stdlib sources (compile-time embedded) ─────────────────────────

fn stdlib_source_for(path: &[String]) -> Option<&'static str> {
    match path.join("::").as_str() {
        "std::trit"     => Some(include_str!("../stdlib/std/trit.tern")),
        "std::math"     => Some(include_str!("../stdlib/std/math.tern")),
        "std::tensor"   => Some(include_str!("../stdlib/std/tensor.tern")),
        "std::io"       => Some(include_str!("../stdlib/std/io.tern")),
        "ml::quantize"  => Some(include_str!("../stdlib/ml/quantize.tern")),
        "ml::inference" => Some(include_str!("../stdlib/ml/inference.tern")),
        _               => None,
    }
}

// ─── Shared helpers ──────────────────────────────────────────────────────────

/// Recursively collect `use` paths from a slice of statements.
fn collect_use_paths(stmts: &[Stmt]) -> Vec<Vec<String>> {
    let mut paths = Vec::new();
    for stmt in stmts {
        match stmt {
            Stmt::Use { path } => paths.push(path.clone()),
            Stmt::Block(inner) => paths.extend(collect_use_paths(inner)),
            Stmt::IfTernary { on_pos, on_zero, on_neg, .. } => {
                paths.extend(collect_use_paths(&[*on_pos.clone()]));
                paths.extend(collect_use_paths(&[*on_zero.clone()]));
                paths.extend(collect_use_paths(&[*on_neg.clone()]));
            }
            Stmt::Match { arms, .. } => {
                for (_, arm_stmt) in arms {
                    paths.extend(collect_use_paths(&[arm_stmt.clone()]));
                }
            }
            _ => {}
        }
    }
    paths
}

/// Parse source string and extract its functions; deduplicate against `known`.
fn parse_and_extract(src: &str, key: &str, known: &mut std::collections::HashSet<String>) -> Vec<Function> {
    let mut parser = Parser::new(src);
    match parser.parse_program() {
        Ok(prog) => prog.functions.into_iter().filter(|f| known.insert(f.name.clone())).collect(),
        Err(e)   => { eprintln!("[MOD-000] Failed to parse module '{key}': {e}"); vec![] }
    }
}

/// Resolve all `use` paths, injecting matching functions into `program`.
/// `extra_source` is called for paths not found in the stdlib — returns `Option<String>`.
fn resolve_with<F>(program: &mut Program, extra_source: F)
where
    F: Fn(&[String]) -> Option<String>,
{
    let mut known: std::collections::HashSet<String> =
        program.functions.iter().map(|f| f.name.clone()).collect();

    let mut all_paths: Vec<Vec<String>> = program
        .functions
        .iter()
        .flat_map(|f| collect_use_paths(&f.body))
        .collect();
    all_paths.sort();
    all_paths.dedup();

    let mut injected: Vec<Function> = Vec::new();

    for path in &all_paths {
        let key = path.join("::");
        if let Some(src) = stdlib_source_for(path) {
            injected.extend(parse_and_extract(src, &key, &mut known));
        } else if let Some(src) = extra_source(path) {
            injected.extend(parse_and_extract(&src, &key, &mut known));
        } else {
            eprintln!("[MOD-001] Unknown module '{key}' — no stdlib match and no file found. Did you mean std::trit?");
        }
    }

    injected.extend(program.functions.drain(..));
    program.functions = injected;
}

// ─── StdlibLoader (backwards-compat, stdlib-only) ────────────────────────────

pub struct StdlibLoader;

impl StdlibLoader {
    /// Resolve stdlib `use` statements in `program`.  User-defined modules are
    /// left for a `ModuleResolver` to handle.
    pub fn resolve(program: &mut Program) {
        resolve_with(program, |_| None);
    }
}

// ─── ModuleResolver (stdlib + user-defined cross-file modules) ───────────────

/// Full module resolver.  Resolves stdlib built-ins AND user `.tern` modules
/// found relative to a source file's directory.
///
/// # Usage
/// ```ignore
/// let mut resolver = ModuleResolver::from_source_file(Path::new("src/main.tern"));
/// resolver.resolve(&mut program);
/// ```
pub struct ModuleResolver {
    base_dir: Option<std::path::PathBuf>,
}

impl ModuleResolver {
    /// Resolve relative to the directory containing `source_file`.
    pub fn from_source_file(source_file: &std::path::Path) -> Self {
        Self { base_dir: source_file.parent().map(|p| p.to_path_buf()) }
    }

    /// Resolve relative to `dir` (directory, not file).
    pub fn from_dir(dir: std::path::PathBuf) -> Self {
        Self { base_dir: Some(dir) }
    }

    /// Stdlib-only resolver (no file-system access). Equivalent to `StdlibLoader`.
    pub fn stdlib_only() -> Self {
        Self { base_dir: None }
    }

    /// Attempt to load `path` (e.g. `["mymod", "utils"]`) as `base_dir/mymod/utils.tern`.
    fn load_user_module(&self, path: &[String]) -> Option<String> {
        let base = self.base_dir.as_ref()?;
        let mut file_path = base.clone();
        for (i, segment) in path.iter().enumerate() {
            if i == path.len() - 1 {
                file_path = file_path.join(format!("{segment}.tern"));
            } else {
                file_path = file_path.join(segment);
            }
        }
        match std::fs::read_to_string(&file_path) {
            Ok(src) => Some(src),
            Err(_)  => None,
        }
    }

    /// Resolve all `use` statements: stdlib first, then user files.
    pub fn resolve(&self, program: &mut Program) {
        resolve_with(program, |path| self.load_user_module(path));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;

    /// Verify that each stdlib module parses without errors.
    #[test]
    fn all_stdlib_modules_parse() {
        let modules = [
            vec!["std".to_string(), "trit".to_string()],
            vec!["std".to_string(), "math".to_string()],
            vec!["std".to_string(), "tensor".to_string()],
            vec!["std".to_string(), "io".to_string()],
            vec!["ml".to_string(), "quantize".to_string()],
            vec!["ml".to_string(), "inference".to_string()],
        ];
        for path in &modules {
            let src = stdlib_source_for(path)
                .unwrap_or_else(|| panic!("Missing stdlib source for {}", path.join("::")));
            let mut parser = Parser::new(src);
            parser.parse_program()
                .unwrap_or_else(|e| panic!("Parse error in {}: {:?}", path.join("::"), e));
        }
    }

    /// A program with `use std::trit;` should gain abs/min/max/etc after resolve.
    #[test]
    fn resolve_injects_trit_stdlib() {
        let src = r#"
fn main() -> trit {
    use std::trit;
    let x: trit = abs(-1);
    return x;
}
"#;
        let mut parser = Parser::new(src);
        let mut prog = parser.parse_program().expect("parse failed");
        assert!(!prog.functions.iter().any(|f| f.name == "abs"),
            "abs should not be present before resolve");
        StdlibLoader::resolve(&mut prog);
        assert!(prog.functions.iter().any(|f| f.name == "abs"),
            "abs should be injected after resolve");
        assert!(prog.functions.iter().any(|f| f.name == "min"));
        assert!(prog.functions.iter().any(|f| f.name == "majority"));
    }

    /// Multiple use statements should all be resolved, with no duplicates.
    #[test]
    fn resolve_multiple_modules_no_duplicates() {
        let src = r#"
fn main() -> trit {
    use std::trit;
    use std::math;
    let x: trit = neg(1);
    return x;
}
"#;
        let mut parser = Parser::new(src);
        let mut prog = parser.parse_program().expect("parse failed");
        StdlibLoader::resolve(&mut prog);

        // Count how many times "neg" appears — should be exactly 1
        let neg_count = prog.functions.iter().filter(|f| f.name == "neg").count();
        assert_eq!(neg_count, 1, "neg should appear exactly once");

        // Both modules should be present
        assert!(prog.functions.iter().any(|f| f.name == "abs"));   // std::trit
        assert!(prog.functions.iter().any(|f| f.name == "rectify")); // std::math
    }

    /// Resolve is idempotent — calling it twice should not duplicate functions.
    #[test]
    fn resolve_is_idempotent() {
        let src = r#"
fn main() -> trit {
    use std::trit;
    return 0;
}
"#;
        let mut parser = Parser::new(src);
        let mut prog = parser.parse_program().expect("parse failed");
        StdlibLoader::resolve(&mut prog);
        StdlibLoader::resolve(&mut prog);
        let abs_count = prog.functions.iter().filter(|f| f.name == "abs").count();
        assert_eq!(abs_count, 1, "abs should not be duplicated by double resolve");
    }

    /// Unknown module paths are silently skipped (not a hard error).
    #[test]
    fn unknown_module_skipped_gracefully() {
        let src = r#"
fn main() -> trit {
    use std::nonexistent;
    return 0;
}
"#;
        let mut parser = Parser::new(src);
        let mut prog = parser.parse_program().expect("parse failed");
        // Should not panic
        StdlibLoader::resolve(&mut prog);
    }
}
