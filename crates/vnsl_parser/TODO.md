# VNSL Parser Improvement Roadmap

This document outlines planned improvements for the `vnsl_parser` crate, organized by priority.

---

## 🔴 High Priority

### 1. Remove Debug Print Statement

**File:** `statement/command/dialogue.rs`  
**Effort:** Low  
**Impact:** Immediate fix for production code quality

#### Current Problem
There's a leftover debug `println!` statement in production code:

```rust
Rule::char_identifier => {
    println!("{:#?}", inner);  // <-- Remove this line
    set_char = Some(VnslSetCharacter {
        id: inner.as_str().to_string(),
    })
}
```

#### Action Items
- [ ] Remove line 21 in `statement/command/dialogue.rs`
- [ ] Search the entire crate for other debug `println!` statements: `grep -r "println!" src/`
- [ ] Consider using `tracing` or `log` crate for intentional debug logging

---

### 2. Replace `unreachable!()` with Proper Error Handling

**Files:** Multiple  
**Effort:** Medium  
**Impact:** Better error messages, no panics in production

#### Current Problem
The codebase uses `unreachable!()` macros which will panic if grammar changes or unexpected input is received:

**Locations to fix:**
- `lib.rs:57` - main parsing loop
- `label.rs:17` - label parsing
- `block.rs:17` - block parsing
- `statement/mod.rs:20` - statement dispatch
- `statement/choices.rs:23, 43` - choices parsing
- `statement/condition.rs:43` - condition parsing
- `statement/command/mod.rs:23` - command dispatch
- `statement/command/action.rs:16, 40` - action parsing
- `statement/command/dialogue.rs:28` - dialogue parsing
- `data_type.rs:21` - data type parsing

#### Action Items

1. **Add a new error variant in `error.rs`:**
```rust
#[derive(Debug, Clone, Error)]
pub enum ParseError {
    // ... existing variants ...
    
    #[error("Unexpected rule {found:?} while parsing {context}. Expected one of: {expected:?}")]
    UnexpectedRule {
        expected: Vec<Rule>,
        found: Rule,
        context: &'static str,
    },
}
```

2. **Create a helper function in `utils/mod.rs`:**
```rust
pub fn unexpected_rule<T>(
    found: Rule,
    expected: &[Rule],
    context: &'static str,
) -> ParsingResult<T> {
    Err(ParseError::UnexpectedRule {
        found,
        expected: expected.to_vec(),
        context,
    }.into())
}
```

3. **Replace each `unreachable!()` call:**
```rust
// Before:
_ => unreachable!("Unexpected rule {:?}", rule.as_rule()),

// After:
other => return unexpected_rule(other, &[Rule::identifier, Rule::block], "label"),
```

4. **Update tests to verify error messages are helpful**

---

## 🟡 Medium Priority

### 3. Unify Error Handling Strategy

**Files:** All parser modules  
**Effort:** Medium  
**Impact:** Consistency, better error context

#### Current Problem
Inconsistent use of `anyhow::Result` vs `ParsingResult`:
- `statement/mod.rs` uses `anyhow::Result`
- `block.rs` uses `ParsingResult`
- Some functions wrap errors, others don't

#### Action Items

1. **Standardize on `ParsingResult` for all parsing functions:**
```rust
// All parser functions should return:
pub fn parse_xxx(rule: Pair<Rule>) -> ParsingResult<XxxType>
```

2. **Update `error.rs` to simplify the error type:**
```rust
pub type ParsingResult<T> = Result<T, ParsingError>;

#[derive(Debug, Error)]
pub struct ParsingError {
    pub rule: Rule,
    pub span: (usize, usize),  // start, end positions
    pub line: usize,
    pub source: ParsingErrorKind,
}

#[derive(Debug, Error)]
pub enum ParsingErrorKind {
    #[error("Unexpected rule: expected {expected:?}, found {found:?}")]
    UnexpectedRule { expected: Vec<Rule>, found: Rule },
    
    #[error("Missing required element: {0}")]
    MissingRequired(String),
    
    #[error("Invalid value: {0}")]
    InvalidValue(String),
    
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
```

3. **Use `wrap_parsing_result` consistently:**
```rust
// In block.rs - already good
let statement = wrap_parsing_result(rule, statement::parse_statement)?;

// Apply same pattern everywhere
```

4. **Files to update:**
   - [ ] `lib.rs` - change `parse_scene` return type
   - [ ] `statement/mod.rs`
   - [ ] `statement/choices.rs`
   - [ ] `statement/condition.rs`
   - [ ] `statement/lua_expr.rs`
   - [ ] `statement/command/mod.rs` and all command submodules
   - [ ] `label.rs`

---

### 4. Implement `Parse` Trait for Cleaner Code

**Files:** New trait definition, all parser modules  
**Effort:** Medium  
**Impact:** Cleaner, more idiomatic Rust code

#### Action Items

1. **Create new file `src/parser_trait.rs`:**
```rust
use pest::iterators::Pair;
use crate::{Rule, error::ParsingResult};

/// Trait for types that can be parsed from a pest rule.
pub trait Parse: Sized {
    /// The expected rule type for this parser.
    const RULE: Rule;
    
    /// Parse from a pest Pair.
    fn parse(rule: Pair<Rule>) -> ParsingResult<Self>;
    
    /// Parse with rule validation.
    fn parse_checked(rule: Pair<Rule>) -> ParsingResult<Self> {
        if rule.as_rule() != Self::RULE {
            return Err(/* unexpected rule error */);
        }
        Self::parse(rule)
    }
}
```

2. **Implement for each AST type:**
```rust
impl Parse for VnslStatement {
    const RULE: Rule = Rule::stmt;
    
    fn parse(rule: Pair<Rule>) -> ParsingResult<Self> {
        // Move logic from statement::parse_statement here
    }
}

impl Parse for VnslBlock {
    const RULE: Rule = Rule::block;
    
    fn parse(rule: Pair<Rule>) -> ParsingResult<Self> {
        // Move logic from block::parse_block here
    }
}
```

3. **Update existing modules to use the trait:**
```rust
// Before:
let statement = statement::parse_statement(rule)?;

// After:
let statement = VnslStatement::parse(rule)?;
```

4. **Update `lib.rs` exports:**
```rust
mod parser_trait;
pub use parser_trait::Parse;
```

---

### 5. Improve Rule Extraction Utilities

**File:** `utils/mod.rs`  
**Effort:** Medium  
**Impact:** Less boilerplate, cleaner parsing code

#### Action Items

1. **Create a builder-style API in `utils/mod.rs`:**
```rust
pub struct RuleExtractor<'a> {
    pairs: Vec<Pair<'a, Rule>>,
    context: &'static str,
}

impl<'a> RuleExtractor<'a> {
    pub fn new(rule: Pair<'a, Rule>, context: &'static str) -> Self {
        Self {
            pairs: rule.into_inner().collect(),
            context,
        }
    }
    
    /// Extract a required rule, error if not found
    pub fn require(&mut self, rule: Rule) -> ParsingResult<Pair<'a, Rule>> {
        self.pairs
            .iter()
            .position(|p| p.as_rule() == rule)
            .map(|i| self.pairs.remove(i))
            .ok_or_else(|| ParsingError::missing(rule, self.context))
    }
    
    /// Extract an optional rule
    pub fn optional(&mut self, rule: Rule) -> Option<Pair<'a, Rule>> {
        self.pairs
            .iter()
            .position(|p| p.as_rule() == rule)
            .map(|i| self.pairs.remove(i))
    }
    
    /// Iterate over remaining pairs
    pub fn remaining(self) -> impl Iterator<Item = Pair<'a, Rule>> {
        self.pairs.into_iter()
    }
    
    /// Assert no remaining pairs
    pub fn expect_empty(self) -> ParsingResult<()> {
        if self.pairs.is_empty() {
            Ok(())
        } else {
            Err(/* unexpected extra rules error */)
        }
    }
}
```

2. **Refactor parsing functions to use the extractor:**
```rust
// Before (in global.rs):
pub fn parse_global(rule: Pair<Rule>) -> anyhow::Result<VnslGlobal> {
    let mut pairs = utils::extract_inners(rule, [Rule::identifier, Rule::data_type]);
    let name = pairs[&Rule::identifier].as_str().to_string();
    let value = data_type::parse_data_type(pairs.remove(&Rule::data_type).unwrap())?;
    Ok(VnslGlobal { name, value })
}

// After:
pub fn parse_global(rule: Pair<Rule>) -> ParsingResult<VnslGlobal> {
    let mut ext = RuleExtractor::new(rule, "global");
    let name = ext.require(Rule::identifier)?.as_str().to_string();
    let value = VnslDataType::parse(ext.require(Rule::data_type)?)?;
    ext.expect_empty()?;
    Ok(VnslGlobal { name, value })
}
```

---

## 🟢 Low Priority (Future Enhancements)

### 6. Add Source Span Information to AST

**Files:** `vnsl_core/src/model/*`, all parser modules  
**Effort:** High  
**Impact:** Enables IDE features, better error reporting

#### Action Items

1. **Create span types in `vnsl_core`:**
```rust
// In vnsl_core/src/span.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(node: T, span: Span) -> Self {
        Self { node, span }
    }
}

impl<T> std::ops::Deref for Spanned<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.node
    }
}
```

2. **Add helper to extract span from pest Pair:**
```rust
// In vnsl_parser/src/utils/mod.rs
pub fn span_from_pair(pair: &Pair<Rule>) -> Span {
    let pest_span = pair.as_span();
    Span {
        start: pest_span.start(),
        end: pest_span.end(),
    }
}
```

3. **Gradually add spans to AST nodes:**
```rust
// Start with top-level nodes
pub struct VnslScene {
    pub name: Spanned<String>,  // Now tracks location
    pub main_block: VnslBlock,
    pub labels: HashMap<String, VnslLabel>,
    pub span: Span,  // Overall span
}
```

4. **Update parser to populate spans:**
```rust
Rule::scene => {
    let span = span_from_pair(&rule);
    let name_pair = rule.into_inner().next().unwrap();
    let name = Spanned::new(
        name_pair.as_str().to_string(),
        span_from_pair(&name_pair),
    );
    scene.name = name;
}
```

---

### 7. Implement Visitor Pattern for AST

**Files:** New module in `vnsl_core`  
**Effort:** Medium  
**Impact:** Enables validation, transformation, analysis passes

#### Action Items

1. **Create visitor trait in `vnsl_core/src/visitor.rs`:**
```rust
use crate::model::*;

pub trait Visitor {
    type Error;
    
    fn visit_scene(&mut self, scene: &VnslScene) -> Result<(), Self::Error> {
        self.visit_block(&scene.main_block)?;
        for label in scene.labels.values() {
            self.visit_label(label)?;
        }
        Ok(())
    }
    
    fn visit_label(&mut self, label: &VnslLabel) -> Result<(), Self::Error> {
        self.visit_block(&label.block)
    }
    
    fn visit_block(&mut self, block: &VnslBlock) -> Result<(), Self::Error> {
        for stmt in &block.statements {
            self.visit_statement(stmt)?;
        }
        Ok(())
    }
    
    fn visit_statement(&mut self, stmt: &VnslStatement) -> Result<(), Self::Error>;
    fn visit_command(&mut self, cmd: &VnslCommand) -> Result<(), Self::Error>;
    fn visit_choices(&mut self, choices: &VnslChoices) -> Result<(), Self::Error>;
    fn visit_condition(&mut self, cond: &VnslCondition) -> Result<(), Self::Error>;
}
```

2. **Create example validator:**
```rust
// In vnsl_core/src/validator.rs
pub struct JumpValidator<'a> {
    labels: HashSet<&'a str>,
    errors: Vec<ValidationError>,
}

impl Visitor for JumpValidator<'_> {
    type Error = std::convert::Infallible;
    
    fn visit_command(&mut self, cmd: &VnslCommand) -> Result<(), Self::Error> {
        if let VnslCommand::Jump(jump) = cmd {
            if !self.labels.contains(jump.to_label.as_str()) {
                self.errors.push(ValidationError::UndefinedLabel {
                    label: jump.to_label.clone(),
                });
            }
        }
        Ok(())
    }
}
```

---

### 8. Add Parsing Context for Better Error Messages

**Files:** New module, update all parsers  
**Effort:** Medium  
**Impact:** Much better error messages

#### Action Items

1. **Create `ParseContext` struct:**
```rust
// In vnsl_parser/src/context.rs
pub struct ParseContext {
    source: String,
    file_name: Option<String>,
    lines: Vec<(usize, usize)>,  // (start, end) byte offsets per line
}

impl ParseContext {
    pub fn new(source: &str, file_name: Option<String>) -> Self {
        let lines = source
            .lines()
            .scan(0, |offset, line| {
                let start = *offset;
                *offset += line.len() + 1;  // +1 for newline
                Some((start, *offset - 1))
            })
            .collect();
        
        Self {
            source: source.to_string(),
            file_name,
            lines,
        }
    }
    
    pub fn line_at(&self, byte_offset: usize) -> usize {
        self.lines
            .iter()
            .position(|(start, end)| byte_offset >= *start && byte_offset <= *end)
            .unwrap_or(0) + 1
    }
    
    pub fn get_line(&self, line_num: usize) -> &str {
        let (start, end) = self.lines.get(line_num - 1).unwrap_or(&(0, 0));
        &self.source[*start..*end]
    }
    
    pub fn format_error(&self, span: Span, message: &str) -> String {
        let line_num = self.line_at(span.start);
        let line = self.get_line(line_num);
        let col = span.start - self.lines[line_num - 1].0;
        
        format!(
            "error: {}\n  --> {}:{}:{}\n   |\n{:>3}| {}\n   | {}^",
            message,
            self.file_name.as_deref().unwrap_or("<input>"),
            line_num,
            col + 1,
            line_num,
            line,
            " ".repeat(col),
        )
    }
}
```

2. **Thread context through parsing:**
```rust
pub fn parse_scene_with_context(
    script: &str,
    file_name: Option<String>,
) -> Result<VnslScene, ParseErrorWithContext> {
    let ctx = ParseContext::new(script, file_name);
    // ... parsing with context ...
}
```

---

### 9. Consider Raw AST Layer (For Tooling)

**Files:** New module  
**Effort:** High  
**Impact:** Enables formatters, refactoring tools

This is a significant undertaking that should be considered if/when building:
- Code formatter
- Language server (LSP)
- Refactoring tools
- Syntax highlighter

The raw AST preserves all syntactic details (whitespace, comments, punctuation positions) while the semantic AST (current implementation) focuses on meaning.

#### Decision Point
- If only runtime execution is needed: Current AST is sufficient
- If tooling is planned: Consider this enhancement

---

## Module Reorganization (Optional)

If undertaking significant refactoring, consider this structure:

```
vnsl_parser/src/
├── lib.rs              # Public API: parse_scene, parse_scene_name
├── grammar.pest
├── context.rs          # ParseContext for error formatting
├── error.rs            # All error types
├── parse.rs            # Parse trait definition
├── ast/                # Parsing implementations
│   ├── mod.rs          # Re-exports
│   ├── scene.rs        # VnslScene parsing
│   ├── block.rs        # VnslBlock parsing
│   ├── statement.rs    # VnslStatement parsing
│   ├── command.rs      # All VnslCommand variants (combined)
│   ├── choices.rs      # VnslChoices parsing
│   ├── condition.rs    # VnslCondition parsing
│   └── data_type.rs    # VnslDataType parsing
└── utils.rs            # RuleExtractor and helpers
```

---

## Testing Checklist

After each improvement:

- [ ] Run `cargo test` to ensure all snapshot tests pass
- [ ] Run `cargo clippy` for lint warnings
- [ ] Verify error messages are helpful (add tests for error cases)
- [ ] Update snapshot test results if AST structure changes
- [ ] Document any breaking changes to public API

---

## Notes

- The `FORCE_RECORD` constant in tests can be set to `true` to regenerate snapshots
- Consider adding `#[must_use]` attributes to parsing functions
- Consider adding `#[inline]` to hot path functions after profiling
