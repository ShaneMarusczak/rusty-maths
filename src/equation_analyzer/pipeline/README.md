# Hybrid Pipeline - Streaming Tokenizer Implementation

The **Hybrid streaming pipeline** optimizes the first stage by using an iterator-based tokenizer that yields tokens on-demand, while keeping the proven Vec-based parser and evaluator.

## Architecture

```
┌──────────────────┐      ┌─────────┐      ┌───────────┐
│ StreamTokenizer  │─────▶│ Parser  │─────▶│ Evaluator │─────▶ f32
│   (Iterator)     │      │         │      │           │
└──────────────────┘      └─────────┘      └───────────┘
         │                     │                  │
    Token (lazy)           Vec<Token>          Result
    on-demand               (RPN)
```

## How It Works

### 1. Streaming Tokenization (core/streaming_tokenizer.rs)
```rust
pub struct StreamingTokenizer<'a> {
    chars: Peekable<Chars<'a>>,
    pending_tokens: VecDeque<Token>,
    // ... state
}

impl Iterator for StreamingTokenizer {
    type Item = Result<Token, String>;
    fn next(&mut self) -> Option<Self::Item>
}
```

**What it does:**
- Implements the **Iterator trait** - yields tokens one at a time
- Scans characters **lazily** only when next() is called
- Uses **VecDeque for pending tokens** when one input produces multiple tokens
- **Pull-based**: Parser drives tokenizer by calling next()

**Example:**
```rust
let tokenizer = StreamingTokenizer::new("2x + 3")?;

// Parser pulls:
tokenizer.next() → Some(Ok(Number(2)))
tokenizer.next() → Some(Ok(Star))       // From pending queue
tokenizer.next() → Some(Ok(X))          // From pending queue
tokenizer.next() → Some(Ok(Plus))
tokenizer.next() → Some(Ok(Number(3)))
tokenizer.next() → Some(Ok(End))
tokenizer.next() → None
```

**Pending Token Queue:**
When tokenizer sees "2x", it needs to emit 3 tokens:
1. Number(2)
2. Star (implicit multiplication)
3. X

```rust
Process '2':  Scan digit → Number(2)
Process 'x':  Need Star + X
             → pending_tokens.push_back(Star)
             → pending_tokens.push_back(X)
             → Return Number(2)
Next call:   → Pop Star from pending queue
Next call:   → Pop X from pending queue
```

**Key characteristics:**
- **Lazy evaluation**: Only scans what's needed
- **Partial buffer**: VecDeque for multi-token sequences only
- **Memory efficient**: No full token Vec allocation
- **UTF-8 aware**: Tracks byte positions for π, e characters

### 2. Streaming Parser (core/parser.rs)
```rust
pub fn parse<I>(tokens: I) -> Result<Vec<Token>, String>
where
    I: IntoIterator<Item = Result<Token, String>>
```

**What it does:**
- Accepts **any iterator** of tokens (not just Vec)
- Pulls tokens one at a time from iterator
- Still collects output into Vec<Token> (RPN)
- Same Shunting Yard algorithm as vec_pipeline

**Example:**
```rust
let tokenizer = StreamingTokenizer::new("2 + 3 * 4")?;
let rpn_tokens = parse(tokenizer)?;

// Parser loop:
for token_result in tokens {              // ← Pulls from tokenizer
    let token = token_result?;
    match token.token_type {
        Number => output.push(token),     // ← Still builds Vec
        Plus | Star => /* shunting yard */
        End => break
    }
}
```

**Flow:**
```
Parser calls next() → Tokenizer scans "2" → Returns Number(2)
                   → Parser pushes to output Vec
Parser calls next() → Tokenizer scans "+" → Returns Plus
                   → Parser pushes to operator stack
... continues pulling until End token
```

**Key characteristics:**
- **Pull-driven**: Parser's for loop drives tokenizer
- **Hybrid approach**: Accepts Iterator, returns Vec
- **No tokenizer blocking**: Don't wait for all tokens upfront

### 3. Evaluation (evaluator.rs)
```rust
pub fn evaluate_streaming(parsed_eq: &[Token], x: impl Into<Option<f32>>) -> Result<f32, String>
```

**What it does:**
- **Identical to vec_pipeline evaluator**
- Accepts RPN token slice
- Stack-based RPN evaluation

**Why not streaming?**
RPN evaluation fundamentally requires buffering:
- Must hold operands on stack until operator arrives
- "2 3 +" requires storing 2 and 3 before seeing +
- No performance gain from streaming this stage

## Buffering Strategy

| Stage | Buffer Type | Size | When Allocated |
|-------|-------------|------|----------------|
| Tokenizer | `VecDeque<Token>` | 0-3 tokens | Only for multi-token sequences |
| Tokenizer | State vars | 1 char | Current position tracking |
| Parser | `Vec<Operand>` | Operators only | Shunting Yard stack |
| Parser | `Vec<Token>` | Full RPN | Output collection |
| Evaluator | `Vec<f32>` | Values only | RPN evaluation stack |

**Comparison to Vec Pipeline:**
- ❌ No full infix token Vec
- ✅ Still has full RPN token Vec
- ✅ Minimal tokenizer overhead (VecDeque only when needed)

## Performance Characteristics

### Strengths:
- ⚡ **~2.7x faster** than vec_pipeline
- 💾 **Lower memory usage**: No full infix token Vec
- 🔄 **Better cache locality**: Tokens consumed immediately after creation
- ✅ **Familiar pattern**: Parser and evaluator unchanged

### How It's Faster:
1. **No allocation overhead**: Tokenizer doesn't allocate full Vec
2. **Immediate consumption**: Parser uses tokens as they're created
3. **Better cache utilization**: Token created → used → dropped (hot in L1 cache)
4. **Early termination**: Parser error stops tokenization immediately

**Example:**
```rust
// Vec pipeline:     Tokenize ALL → Parse ALL → Evaluate → Error
// Hybrid pipeline:  Tokenize → Parse → Error (stops early!)

calculate("2 + + 3")  // Syntax error at second +
// Vec:    Tokenizes entire string, then parser fails
// Hybrid: Tokenizer yields tokens until parser hits error, stops immediately
```

### Benchmark Results:
```
Simple calculate("2 + 3 * 4"): ~30ms per 10,000 iterations
Speedup over vec_pipeline: 2.7x faster
```

### Trade-offs:
- ⚠️ **Still buffers RPN**: Full Vec of RPN tokens
- ⚠️ **Iterator complexity**: Slightly more complex than simple Vec

## Code Example

```rust
use rusty_maths::equation_analyzer::hybrid_pipeline::calculator::calculate;

// Same API as vec_pipeline
let result = calculate("2 + 3 * 4").unwrap();
assert_eq!(result, 14.0);

// Internally:
// 1. StreamingTokenizer yields: Number(2), Plus, Number(3), Star, Number(4), End
// 2. Parser pulls and converts to RPN Vec: [2, 3, 4, *, +]
// 3. Evaluator processes RPN Vec → 14.0

// Complex equation with π
let result = calculate("sin(π / 2)").unwrap();
// Tokenizer correctly handles multi-byte UTF-8 character π
```

## Architecture Diagram

```
┌─────────────────────────────────────────────┐
│         "2 + 3 * 4"                         │
└─────────────────┬───────────────────────────┘
                  │
                  ▼
      ┌───────────────────────┐
      │  StreamingTokenizer   │
      │  ┌─────────────────┐  │
      │  │ pending_tokens  │  │ ← VecDeque (0-3 items)
      │  │ [Star, X]       │  │
      │  └─────────────────┘  │
      └───────────┬───────────┘
                  │ yields Token
                  │ (one at a time)
                  ▼
      ┌───────────────────────┐
      │    Parser             │
      │  ┌─────────────────┐  │
      │  │ operator_stack  │  │ ← Vec<Operand>
      │  │ [+, *]          │  │
      │  └─────────────────┘  │
      │  ┌─────────────────┐  │
      │  │ output (RPN)    │  │ ← Vec<Token>
      │  │ [2, 3, 4]       │  │
      │  └─────────────────┘  │
      └───────────┬───────────┘
                  │ Complete RPN Vec
                  ▼
      ┌───────────────────────┐
      │    Evaluator          │
      │  ┌─────────────────┐  │
      │  │ value_stack     │  │ ← Vec<f32>
      │  │ [2, 12]         │  │
      │  └─────────────────┘  │
      └───────────┬───────────┘
                  │
                  ▼
                 14.0
```

## When to Use

**Best for:**
- ⚡ **Performance-critical**: Need speed without full rewrite
- 🔄 **Drop-in replacement**: Same API as vec_pipeline
- 💾 **Memory constraints**: Reduce allocation overhead
- 📊 **Plotting**: Still supports parse-once, eval-many optimization

**Consider alternatives when:**
- 📚 Want absolute simplest code (use vec_pipeline)

## Key Innovations

### 1. Iterator-Based Tokenization
Traditional approach (vec_pipeline):
```rust
fn get_tokens(eq: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    for char in eq.chars() {
        tokens.push(scan(char));  // ← Pushes all
    }
    tokens  // ← Returns after collecting all
}
```

Streaming approach (hybrid_pipeline):
```rust
impl Iterator for StreamingTokenizer {
    fn next(&mut self) -> Option<Token> {
        self.scan_token()  // ← Scans one, returns immediately
    }
}
```

### 2. Pull-Based Architecture
Parser controls the flow:
```rust
for token in tokenizer {  // ← Parser pulls
    process(token);        // ← Use immediately
}  // ← No intermediate Vec
```

## Files in this Directory

- **evaluator.rs** - Vec<Token> (RPN) → f32 result - wraps core/evaluator
- **calculator.rs** - Public API (calculate, plot)
- **mod.rs** - Module exports

**Note:** This pipeline uses `core/streaming_tokenizer.rs` and `core/parser.rs` (shared across pipelines).

## Related

- See **vec_pipeline/** for the baseline implementation
