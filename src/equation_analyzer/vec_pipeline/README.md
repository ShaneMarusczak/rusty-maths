# Vec Pipeline - Original Implementation

The **Vec-based pipeline** is the original, straightforward implementation that processes equations in three distinct stages, collecting all intermediate results into vectors.

## Architecture

```
┌─────────────┐      ┌─────────┐      ┌───────────┐
│  Tokenizer  │─────▶│ Parser  │─────▶│ Evaluator │─────▶ f32
└─────────────┘      └─────────┘      └───────────┘
      │                   │                  │
   Vec<Token>         Vec<Token>         Result
   (infix)             (RPN)
```

## How It Works

### 1. Tokenization (core/vec_tokenizer.rs)
```rust
pub fn get_tokens(eq: &str) -> Result<Vec<Token>, String>
```

**What it does:**
- Scans the entire equation string character by character
- Converts characters into tokens (numbers, operators, functions, etc.)
- **Collects ALL tokens into a Vec** before returning
- Handles special cases like "2x" → [Number(2), Star, X]

**Example:**
```rust
Input:  "2 + 3 * 4"
Output: Vec[Number(2), Plus, Number(3), Star, Number(4), End]
```

**Key characteristic:**
- **Push-based**: Tokenizer loops through entire string, pushing all tokens into Vec
- **Full collection**: Must complete before parser starts

### 2. Parsing (parser.rs)
```rust
pub fn parse(tokens: Vec<Token>) -> Result<Vec<Token>, String>
```

**What it does:**
- Accepts the complete Vec<Token> from tokenizer
- Converts infix notation to Reverse Polish Notation (RPN) using Shunting Yard algorithm
- Maintains operator stack internally during conversion
- **Collects ALL RPN tokens into a Vec** before returning

**Example:**
```rust
Input:  Vec[Number(2), Plus, Number(3), Star, Number(4)]
Output: Vec[Number(2), Number(3), Number(4), Star, Plus]
        // RPN: 2 3 4 * +  (evaluates to 14)
```

**Algorithm (Shunting Yard):**
```
Operator Stack: []
Output Queue:   []

Process: 2      → Output: [2]
Process: +      → Stack: [+]
Process: 3      → Output: [2, 3]
Process: *      → Stack: [+, *]  (* has higher precedence)
Process: 4      → Output: [2, 3, 4]
End of input    → Pop stack: [2, 3, 4, *, +]
```

**Key characteristic:**
- **Sequential processing**: Iterates through entire token Vec
- **Full collection**: Builds complete RPN Vec before returning

### 3. Evaluation (evaluator.rs)
```rust
pub fn evaluate(parsed_eq: &[Token], x: impl Into<Option<f32>>) -> Result<f32, String>
```

**What it does:**
- Accepts the complete slice of RPN tokens
- Maintains a value stack
- Processes each token:
  - **Numbers**: Push onto stack
  - **Operators**: Pop operands, compute, push result
  - **Functions**: Pop arguments, compute, push result

**Example:**
```rust
Input: [Number(2), Number(3), Number(4), Star, Plus]

Stack: []
Token: 2     → Stack: [2]
Token: 3     → Stack: [2, 3]
Token: 4     → Stack: [2, 3, 4]
Token: *     → Pop 4, 3 → Compute 3*4=12 → Stack: [2, 12]
Token: +     → Pop 12, 2 → Compute 2+12=14 → Stack: [14]

Result: 14.0
```

**Key characteristic:**
- **Sequential processing**: Iterates through entire RPN token slice
- **Stack-based**: Maintains value stack, final result is last item

## Buffering Strategy

| Stage | Buffer Type | Size | Purpose |
|-------|-------------|------|---------|
| Tokenizer | `Vec<Token>` | Full equation | Collect all infix tokens |
| Parser | `Vec<Operand>` (stack) | Operators only | Shunting Yard algorithm |
| Parser | `Vec<Token>` (output) | Full equation | Collect all RPN tokens |
| Evaluator | `Vec<f32>` (stack) | Intermediate values | RPN evaluation |

**Total collections:** 2 full Vecs (infix tokens + RPN tokens)

## Performance Characteristics

### Strengths:
- ✅ **Simple and clear**: Easy to understand and debug
- ✅ **Predictable**: All data collected upfront
- ✅ **Reusable**: Parsed equation can be evaluated multiple times (plot optimization)

### Trade-offs:
- ⚠️ **Memory allocation**: Two full Vec allocations for every calculation
- ⚠️ **No lazy evaluation**: Must tokenize entire equation even if evaluation fails early
- ⚠️ **Cache locality**: Data spread across multiple allocations

### Benchmark Results:
```
Simple calculate("2 + 3 * 4"): ~80ms per 10,000 iterations
Baseline for comparison: 1.0x
```

## Code Example

```rust
use rusty_maths::equation_analyzer::vec_pipeline::calculator::calculate;

// Simple calculation
let result = calculate("2 + 3 * 4").unwrap();
assert_eq!(result, 14.0);

// With functions
let result = calculate("sin(π / 2)").unwrap();
assert_eq!(result, 1.0);

// Plot optimization (parse once, evaluate many times)
let points = plot("y = x^2", -10.0, 10.0, 0.1).unwrap();
// Internally: tokenize→Vec, parse→Vec (once), then evaluate Vec 200 times
```

## When to Use

**Best for:**
- 📚 **Learning**: Clearest implementation to understand
- 🔍 **Debugging**: Easy to inspect intermediate states
- 📊 **Plotting**: Parse once, reuse for multiple x values
- 🎯 **Baseline**: Reference implementation for testing

**Consider alternatives when:**
- ⚡ Performance is critical (use hybrid or full pipeline)
- 💾 Memory usage is a concern
- 🔄 Processing very long equations rarely evaluated

## Files in this Directory

- **parser.rs** - Vec<Token> → Vec<Token> (infix → RPN) - wraps core/parser
- **evaluator.rs** - Vec<Token> (RPN) → f32 result - wraps core/evaluator
- **calculator.rs** - Public API (calculate, plot)
- **mod.rs** - Module exports

**Note:** This pipeline uses `core/vec_tokenizer.rs` (shared tokenizer implementation).

## Related

- See **hybrid_pipeline/** for streaming tokenizer with Vec parser
- See **full_pipeline/** for fully streaming with minimal buffers
