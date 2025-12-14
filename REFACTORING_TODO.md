# Refactoring Completion Checklist

## Context

This document outlines the remaining mechanical work to complete the refactoring started in commit 96e9e52. The architectural changes are complete - the remaining work is applying the same pattern to all pipelines.

## ✅ Completed

- [x] Created `ParamCollector` module with comprehensive tests
- [x] Added `TokenType` helper methods (`is_synthetic()`, `is_variadic_function()`)
- [x] Created `utils` module with `get_x_values()` and `make_synthetic_token()`
- [x] Updated `vec_pipeline/parser.rs` to use `make_synthetic_token()`
- [x] Updated `vec_pipeline/evaluator.rs` to use `ParamCollector`
- [x] All tests passing for vec_pipeline

## 🔲 Remaining Work

### 1. Update hybrid_pipeline/parser.rs

**Pattern to apply:**

```rust
// Add import
use crate::equation_analyzer::utils::make_synthetic_token;

// Replace lines ~36-78 (the match statement for ParamToken)
} else if token.token_type == TokenType::CloseParen {
    output.push(make_synthetic_token(param_token.to_end_token_type()));
    param_token = ParamToken::None;
    continue;
} else {
    return Err("Params can only be numbers".to_string());
}
```

### 2. Update hybrid_pipeline/evaluator.rs

**Pattern to apply:**

```rust
// Add imports
use crate::equation_analyzer::utils::param_collector::{CollectionResult, ParamCollector};

// Remove this block (lines ~25-115):
let mut params = vec![];
let mut collecting_params = false;

for token in parsed_eq {
    if collecting_params {
        // ... all the match statement code ...
    }

    match token.token_type {
        TokenType::Avg | TokenType::Max | ... => {
            collecting_params = true;
        }

// Replace with:
let mut collector = ParamCollector::new();

for token in parsed_eq {
    match collector.process_token(token, x) {
        CollectionResult::NotCollecting => {
            // Fall through to normal token processing
        }
        CollectionResult::Continue => {
            continue;
        }
        CollectionResult::Finished(Ok(value)) => {
            stack.push(value);
            continue;
        }
        CollectionResult::Finished(Err(e)) => {
            return Err(e);
        }
    }

    match token.token_type {
        _ if token.token_type.is_variadic_function() => {
            collector.start_collecting();
        }
```

### 3. Update full_pipeline/parser.rs

Same pattern as hybrid_pipeline/parser.rs

###4. Update full_pipeline/evaluator.rs

Same pattern as hybrid_pipeline/evaluator.rs (but note it takes an Iterator, not a slice)

### 5. Update pipeline/parser.rs

Same pattern as hybrid_pipeline/parser.rs

### 6. Update pipeline/evaluator.rs

Same pattern as hybrid_pipeline/evaluator.rs

### 7. Update pipeline/streaming_parser.rs

Same pattern as hybrid_pipeline/parser.rs

### 8. Update pipeline/streaming_evaluator.rs

Same pattern as hybrid_pipeline/evaluator.rs

### 9. Update pipeline/fully_streaming_parser.rs

Same pattern as hybrid_pipeline/parser.rs

### 10. Update pipeline/fully_streaming_evaluator.rs

Same pattern as full_pipeline/evaluator.rs

### 11. Update all calculators to use utils::get_x_values

**Files to update:**
- `vec_pipeline/calculator.rs`
- `hybrid_pipeline/calculator.rs`
- `full_pipeline/calculator.rs`

**Pattern:**

```rust
// Add import
use crate::equation_analyzer::utils::get_x_values;

// Remove the get_x_values function definition (lines ~63-73)

// In plot() function, replace:
let x_values = get_x_values(x_min, x_max, step_size);

// With:
let x_values = get_x_values(x_min, x_max, step_size);
// (same call, just using the imported version)
```

### 12. Add nested variadic function test

**File:** `src/equation_analyzer/tests.rs`

**Add after mode tests:**

```rust
#[test]
fn test_nested_variadic_not_supported() {
    // Nested variadic functions are not supported
    // Inner function will be treated as parameter and should error
    let result = calculate("avg(1, min(2, 3), 4)");
    assert!(result.is_err(), "Nested variadic functions should error");
    assert!(result.unwrap_err().contains("Invalid token"));
}

#[test]
fn test_nested_variadic_min_in_max() {
    let result = calculate("max(1, min(2, 3))");
    assert!(result.is_err());
}

#[test]
fn test_variadic_with_arithmetic_works() {
    // But arithmetic before variadic is fine
    assert_eq!(calculate("avg(1+1, 2*2, 3^2)").unwrap(), 5.0); // (2+4+9)/3
    assert_eq!(calculate("min(5-2, 10/2, 2*3)").unwrap(), 3.0);
}
```

### 13. Add benchmarks (optional)

**File:** Create `benches/param_collection.rs`

```rust
#![feature(test)]
extern crate test;
use test::Bencher;
use rusty_maths::equation_analyzer::vec_pipeline::calculator::calculate;

#[bench]
fn bench_avg_many_params(b: &mut Bencher) {
    let eq = "avg(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20)";
    b.iter(|| {
        calculate(eq).unwrap()
    });
}

#[bench]
fn bench_nested_arithmetic(b: &mut Bencher) {
    let eq = "((1+2)*(3+4))^2 + ((5-6)/(7-8))";
    b.iter(|| {
        calculate(eq).unwrap()
    });
}

#[bench]
fn bench_mode_large(b: &mut Bencher) {
    let eq = "mode(1,2,3,4,5,1,2,3,4,5,1,2,3,4,5,6,7,8,9,10)";
    b.iter(|| {
        calculate(eq).unwrap()
    });
}
```

## Testing Strategy

After each parser/evaluator update:

```bash
# Test specific pipeline
cargo test --lib equation_analyzer::vec_pipeline
cargo test --lib equation_analyzer::hybrid_pipeline
cargo test --lib equation_analyzer::full_pipeline

# Test all equation analyzer tests
cargo test --lib equation_analyzer

# Run full suite at the end
cargo test --lib
```

## Expected Outcomes

### Lines of Code Reduction
- **Before:** ~720 lines of duplicated parameter collection code across 6 evaluators
- **After:** ~250 lines in ParamCollector + ~15 lines per evaluator = ~340 lines total
- **Savings:** ~380 lines of code (53% reduction)

### Maintainability
- Single source of truth for parameter collection logic
- Adding new variadic functions: add 3 lines to ParamCollector instead of 20+ lines to 6 evaluators
- Bugs only need to be fixed once

### Error Handling
- Better error messages with parameter numbers
- No more `unreachable!()` that could panic
- Graceful handling of invalid inputs

## Estimated Time

- Each parser update: ~5 minutes
- Each evaluator update: ~10 minutes
- Calculator updates: ~15 minutes total
- Tests: ~10 minutes
- **Total:** ~2 hours of mechanical work

## Notes

- All changes follow the same pattern - copy from vec_pipeline and adapt
- Tests should pass after each individual file update
- Commit after completing each pipeline to track progress
- No new functionality - pure refactoring for code quality

## Verification Checklist

After all updates:

- [ ] All 117 tests pass
- [ ] No compiler warnings about unused code
- [ ] ParamCollector tests all pass (12 tests)
- [ ] Nested variadic test fails with clear error
- [ ] No performance regression (run benchmarks if added)
- [ ] Code compiles with no warnings
- [ ] Documentation is up to date
