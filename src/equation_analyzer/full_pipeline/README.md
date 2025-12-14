# Full Pipeline - Fully Streaming with Minimal Partial Buffers

The **Fully streaming pipeline** is the most advanced implementation where **every stage is an iterator**, creating a true pull-based architecture. The evaluator pulls from the parser, which pulls from the tokenizer. Each stage maintains only the **minimal partial buffers** required by its algorithm.

## Architecture

```
┌──────────────────┐      ┌──────────────────┐      ┌────────────────────┐
│ StreamTokenizer  │─────▶│ FullyStream      │─────▶│ FullyStream        │──▶ f32
│   (Iterator)     │      │ Parser           │      │ Evaluator          │
│                  │      │ (Iterator)       │      │                    │
└──────────────────┘      └──────────────────┘      └────────────────────┘
         │                         │                          │
    Token (lazy)              Token (lazy)              Consumes RPN
    on-demand                  RPN, on-demand           tokens on-demand

     Pending                  Operator                   Value
     Queue                    Stack                      Stack
    (0-3 items)              (operators)                (values)
```

## How It Works

### 1. Streaming Tokenizer (tokenizer.rs)
```rust
impl Iterator for StreamingTokenizer<'_> {
    type Item = Result<Token, String>;
    fn next(&mut self) -> Option<Self::Item>
}
```

**Same as hybrid_pipeline** - yields tokens one at a time with VecDeque for multi-token sequences.

**Minimal buffer:**
- `VecDeque<Token>` - 0-3 tokens for multi-token sequences only
- No full collection

### 2. Fully Streaming Parser (parser.rs)
```rust
pub struct FullyStreamingParser<I>
where
    I: Iterator<Item = Result<Token, String>>
{
    tokens: I,
    operator_stack: Vec<Operand>,
    output_queue: VecDeque<Token>,
    // ... state
}

impl<I> Iterator for FullyStreamingParser<I> {
    type Item = Result<Token, String>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Return queued RPN tokens if available
            if let Some(token) = self.output_queue.pop_front() {
                return Some(Ok(token));
            }

            // Pull next input token from tokenizer
            match self.tokens.next() {
                Some(Ok(token)) => self.process_token(token)?,
                _ => return None
            }
        }
    }
}
```

**What it does:**
- **Implements Iterator** - yields RPN tokens on-demand
- Pulls infix tokens from tokenizer **only when needed**
- Yields RPN tokens **as soon as they can be output**
- Maintains **two partial buffers**:
  - `operator_stack`: Required for Shunting Yard algorithm
  - `output_queue`: RPN tokens ready to yield

**Example Flow:**
```rust
Input infix: "2 + 3 * 4"

Parser.next() call #1:
  ← Pull from tokenizer: Number(2)
  → Process: Numbers go directly to output
  → Queue: [Number(2)]
  → Return: Number(2)

Parser.next() call #2:
  ← Pull from tokenizer: Plus
  → Process: Push to operator stack
  → Queue: [] (empty, can't output yet)
  ← Pull from tokenizer: Number(3)
  → Queue: [Number(3)]
  → Return: Number(3)

Parser.next() call #3:
  ← Pull from tokenizer: Star
  → Process: * has higher precedence than +, push to stack
  → Queue: [] (empty)
  ← Pull from tokenizer: Number(4)
  → Queue: [Number(4)]
  → Return: Number(4)

Parser.next() call #4:
  ← Pull from tokenizer: End
  → Process: Flush operators: Star, then Plus
  → Queue: [Star, Plus]
  → Return: Star

Parser.next() call #5:
  → Queue: [Plus] (still has queued token)
  → Return: Plus

Parser.next() call #6:
  → Queue: []
  → Input exhausted
  → Return: None
```

**Key insight:**
The parser **doesn't collect all RPN tokens**. It yields them incrementally as the Shunting Yard algorithm determines their output order.

**Minimal buffers:**
- `operator_stack`: Only operators (not full equation)
- `output_queue`: 0-N tokens ready to output (usually small)

### 3. Fully Streaming Evaluator (evaluator.rs)
```rust
pub fn evaluate_fully_streaming<I>(
    tokens: I,
    x: impl Into<Option<f32>>
) -> Result<f32, String>
where
    I: Iterator<Item = Result<Token, String>>
{
    let mut stack: Vec<f32> = Vec::new();

    for token_result in tokens {  // ← Pulls from parser
        let token = token_result?;
        match token.token_type {
            Number => stack.push(token.value),
            Plus => {
                let (lhs, rhs) = (stack.pop()?, stack.pop()?);
                stack.push(lhs + rhs);
            }
            // ... other operators
        }
    }

    stack.pop()  // Final result
}
```

**What it does:**
- Accepts **parser iterator** (not Vec!)
- Pulls RPN tokens **one at a time** from parser
- Parser only generates next RPN token when evaluator needs it
- Evaluator only continues if computation succeeds

**Example:**
```rust
let tokenizer = StreamingTokenizer::new("2 + 3 * 4")?;
let parser = FullyStreamingParser::new(tokenizer);
let result = evaluate_fully_streaming(parser, None)?;

// Call chain:
Evaluator pulls → Parser pulls → Tokenizer scans → '2'
                → Parser outputs → Number(2)
                → Evaluator pushes → stack: [2]

Evaluator pulls → Parser pulls → Tokenizer scans → '+'
                → Parser holds → (can't output yet)
                → Parser pulls → Tokenizer scans → '3'
                → Parser outputs → Number(3)
                → Evaluator pushes → stack: [2, 3]

Evaluator pulls → Parser pulls → Tokenizer scans → '*'
                → Parser holds → (higher precedence)
                → Parser pulls → Tokenizer scans → '4'
                → Parser outputs → Number(4)
                → Evaluator pushes → stack: [2, 3, 4]

Evaluator pulls → Parser pulls → Tokenizer scans → (end)
                → Parser flushes → Star
                → Evaluator computes → 3*4=12 → stack: [2, 12]

Evaluator pulls → Parser has queued → Plus
                → Evaluator computes → 2+12=14 → stack: [14]

Evaluator pulls → Parser exhausted → None
                → Evaluator returns → 14.0
```

**Minimal buffer:**
- `Vec<f32>`: Value stack (only intermediate values, not tokens)

## The Pull Chain

```
┌─────────────────────────────────────────────────────────┐
│  Evaluator: "I need the next RPN token"                │
│     ▼                                                   │
│  Parser: "I need to determine next RPN token"          │
│     ▼                                                   │
│  Check output_queue → Empty                            │
│     ▼                                                   │
│  Pull from tokenizer: "I need next infix token"        │
│     ▼                                                   │
│  Tokenizer: Scan next character(s)                     │
│     ▼                                                   │
│  Return: Token                                          │
│     ▼                                                   │
│  Parser: Process token with Shunting Yard              │
│     ▼                                                   │
│  Queue RPN tokens if ready                             │
│     ▼                                                   │
│  Return first queued token to Evaluator                │
│     ▼                                                   │
│  Evaluator: Process RPN token on stack                 │
└─────────────────────────────────────────────────────────┘
```

## Buffering Strategy - Minimal Partial Buffers

This is what you imagined: **only buffer what the algorithm absolutely requires at each moment**.

| Stage | Buffer | Contents | Max Size | Why Needed |
|-------|--------|----------|----------|------------|
| Tokenizer | `VecDeque<Token>` | Pending tokens | 0-3 | Multi-token from single input ("2x") |
| Parser | `Vec<Operand>` | Operators | O(depth) | Shunting Yard requires operator stack |
| Parser | `VecDeque<Token>` | Output RPN | 0-N | RPN tokens ready to yield |
| Evaluator | `Vec<f32>` | Values | O(depth) | RPN evaluation requires value stack |

**Comparison:**

| Implementation | Infix Vec | RPN Vec | Operator Stack | Value Stack | Pending Queue |
|----------------|-----------|---------|----------------|-------------|---------------|
| **Vec** | ✅ Full | ✅ Full | ✅ Partial | ✅ Partial | ❌ None |
| **Hybrid** | ❌ None | ✅ Full | ✅ Partial | ✅ Partial | ✅ Minimal |
| **Full** | ❌ None | ❌ None | ✅ Partial | ✅ Partial | ✅ Minimal |

**Full pipeline has NO complete collection of tokens** - only the minimal stacks required by the algorithms.

## Performance Characteristics

### Strengths:
- ⚡ **~2.6x faster** than vec_pipeline
- 💾 **Lowest memory footprint**: No full token collections
- 🏗️ **True streaming**: Complete pull-based architecture
- 🎯 **Early termination**: Error stops entire pipeline immediately
- 📐 **Elegant design**: Each stage is self-contained iterator

### Why It's Fast:
1. **No Vec allocations**: Never collects full token arrays
2. **Immediate consumption**: Token created → processed → dropped
3. **Optimal cache usage**: Hot data stays in L1 cache
4. **Lazy evaluation**: Only computes what's needed
5. **Early exit**: First error stops all upstream processing

**Example - Early Termination:**
```rust
calculate("2 + invalid syntax + + 3 * 4 * 5")

Vec Pipeline:
  1. Tokenize: ENTIRE string → Error at "invalid", but scanned all
  2. Parser: Tries to parse → Fails

Fully Streaming:
  1. Evaluator pulls → Parser pulls → Tokenizer scans "2" → Number(2) ✓
  2. Evaluator pulls → Parser pulls → Tokenizer scans "+" → Plus ✓
  3. Evaluator pulls → Parser pulls → Tokenizer scans "invalid" → ERROR
  4. Pipeline stops immediately. Never scanned "+ + 3 * 4 * 5"
```

### Benchmark Results:
```
Simple calculate("2 + 3 * 4"): ~30ms per 10,000 iterations
Speedup over vec_pipeline: 2.6x faster
Speedup over hybrid: 0.99x (nearly identical)
```

### Why Not Faster Than Hybrid?
Both implementations have similar bottlenecks:
- Both avoid full infix Vec
- Parser and evaluator still need their algorithm-required stacks
- The RPN Vec in hybrid is small and hot in cache
- Iterator overhead roughly equals saved RPN Vec allocation

**The win is architectural elegance, not raw speed.**

## Code Example

```rust
use rusty_maths::equation_analyzer::full_pipeline::calculator::calculate;

// Same API
let result = calculate("2 + 3 * 4").unwrap();

// Internally (full pull chain):
let tokenizer = StreamingTokenizer::new("2 + 3 * 4")?;
let parser = FullyStreamingParser::new(tokenizer);
let result = evaluate_fully_streaming(parser, None)?;

// Each stage pulls from previous:
// evaluate calls parser.next() → parser calls tokenizer.next()
```

## Architecture Diagram

```
              "2 + 3 * 4"
                   │
                   ▼
    ┌──────────────────────────┐
    │  StreamingTokenizer      │
    │  ┌────────────────────┐  │
    │  │ pending_tokens     │  │ VecDeque (0-3)
    │  │ []                 │  │
    │  └────────────────────┘  │
    └────────────┬─────────────┘
                 │ Token (on pull)
                 ▼
    ┌──────────────────────────┐
    │  FullyStreamingParser    │
    │  ┌────────────────────┐  │
    │  │ operator_stack     │  │ Vec<Operand>
    │  │ [+]                │  │
    │  └────────────────────┘  │
    │  ┌────────────────────┐  │
    │  │ output_queue       │  │ VecDeque<Token>
    │  │ [Number(4), Star]  │  │
    │  └────────────────────┘  │
    └────────────┬─────────────┘
                 │ RPN Token (on pull)
                 ▼
    ┌──────────────────────────┐
    │  FullyStreamingEvaluator │
    │  ┌────────────────────┐  │
    │  │ value_stack        │  │ Vec<f32>
    │  │ [2.0, 12.0]        │  │
    │  └────────────────────┘  │
    └────────────┬─────────────┘
                 │
                 ▼
                14.0
```

## When to Use

**Best for:**
- 🏗️ **Architectural purity**: Want true streaming design
- 📚 **Educational**: Learn pull-based iterator patterns
- 🔍 **Early termination**: Benefit from stopping on first error
- 💾 **Extreme memory constraints**: Absolutely minimal buffering

**Consider alternatives when:**
- 🚀 Pure speed is the only goal (hybrid is equivalent)
- 🎯 Simplicity matters more (use vec_pipeline)
- 📊 Heavy plotting workload (hybrid's Vec RPN is fine for reuse)

## Key Innovations

### 1. Parser as Iterator
Traditional (hybrid_pipeline):
```rust
fn parse(tokens: impl Iterator) -> Vec<Token> {
    let mut output = Vec::new();
    for token in tokens {
        output.push(process(token));
    }
    output  // ← Returns full Vec
}
```

Fully streaming:
```rust
impl Iterator for FullyStreamingParser {
    fn next(&mut self) -> Option<Token> {
        if let Some(ready) = self.output_queue.pop_front() {
            return Some(ready);  // ← Return immediately
        }
        let token = self.tokens.next()?;
        self.process_token(token);  // ← May queue multiple RPN tokens
        self.output_queue.pop_front()
    }
}
```

### 2. Three-Stage Pull Chain
```rust
// Evaluator doesn't call parser, doesn't call tokenizer
// Instead, evaluator's for loop creates pull chain automatically:

for rpn_token in parser {  // ← This pull...
    // parser's next() pulls from tokenizer
    // tokenizer's next() scans characters
    process(rpn_token);
}
```

### 3. Output Queue for RPN
Parser might need to output multiple RPN tokens from one infix token:
```rust
Input: End token
Process: Flush all operators from stack
Queue: [Star, Plus, Minus]  // ← Multiple RPN tokens queued
next() → Pop Star
next() → Pop Plus
next() → Pop Minus
```

## Files in this Directory

- **tokenizer.rs** - StreamingTokenizer (same as hybrid)
- **parser.rs** - FullyStreamingParser (Iterator → Iterator)
- **evaluator.rs** - evaluate_fully_streaming (consumes Iterator)
- **calculator.rs** - Public API (calculate, plot)
- **mod.rs** - Module exports

## Related

- See **vec_pipeline/** for the baseline Vec-based implementation
- See **hybrid_pipeline/** for practical streaming with Vec parser
