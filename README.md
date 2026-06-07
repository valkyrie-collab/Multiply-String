# High-Precision String Multiplication Engine in Rust

An arbitrary-precision multiplication framework written in Rust. This system circumvents native CPU register width boundaries (`u64`, `u128`) to multiply numeric values of infinite size by simulating grade-school columnar multiplication digit-by-digit directly across memory slices.

---

## 🏗️ Architectural Flow & Preprocessing

The execution path enforces clean handling of mathematical identities (constants like `0` and `1`) before passing full workloads to the internal multiplication loops.

```text
┌──────────────────────────────────────────────┐
│       User Input (via io::stdin)             │
└──────────────────────┬───────────────────────┘
                       │
                       ▼
┌──────────────────────────────────────────────┐
│        remove_constant Preprocessor         │
├──────────────────────────────────────────────┤
│ Checks for trivial properties:               │
│ - Identity Property:  N × 1 = N              │
│ - Zero Property:      N × 0 = 0              │
└──────────────────────┬───────────────────────┘
                       │
                       ▼ Non-Trivial Workloads
┌──────────────────────────────────────────────┐
│           String Reversal Phase              │
├──────────────────────────────────────────────┤
│ .chars().rev().collect()                     │
│ Maps Little-Endian indexing: index 0 shifts  │
│ from the most-significant to the least-      │
│ significant digit (units place).             │
└──────────────────────┬───────────────────────┘
                       │
                       ▼
┌──────────────────────────────────────────────┐
│            find_product Engine               │
│    (O(N × M) Columnar Convolution Layer)     │
└──────────────────────────────────────────────┘

```

---

## 📊 Visualizing the Grid Multiplication Mechanics

Your code maps the multiplication grid using a **Little-Endian positional tracking design**. By passing the inputs reversed, `string_one[0]` and `string_two[0]` represent the units place ($10^0$), index 1 represents tens ($10^1$), index 2 represents hundreds ($10^2$), and so on.

### Index Mapping Matrix: `total = xoi + xti`

The code tracks spatial offsets via `total = xoi + xti`. This represents the mathematical invariant where multiplying a position digit of magnitude $10^A$ by a position digit of magnitude $10^B$ guarantees that the resulting product land at positional magnitude $10^{A+B}$.

When multiplying `string_one = "123"` and `string_two = "456"`, the preprocessor passes them to the engine reversed:

* `string_one` $\rightarrow$ `['3', '2', '1']`
* `string_two` $\rightarrow$ `['6', '5', '4']`

```text
               xoi=0         xoi=1         xoi=2
             [ '3' ]       [ '2' ]       [ '1' ]   (string_one)
             
xti=0 [ '6' ]  18            12             6      ◄── Partial row sums matching
xti=1 [ '5' ]  15            10             5          overlapping positional
xti=2 [ '4' ]  12             8             4          magnitudes
  ▲
(string_two)

               ─────────────────────────────────
total offsets:   0             1             2             3             4
               (10⁰)         (10¹)         (10²)         (10³)         (10⁴)

```

---

## 🔬 In-Depth Code Walkthrough

### 1. Zero and Identity Short-Circuiting

```rust
fn remove_constant(num_one: String, num_two: String) -> String {
    if num_one == "0" { return num_one; }
    if num_one == "1" { return num_two; }
    if num_two == "0" { return num_two; }
    if num_two == "1" { return num_one; }

    find_combo(num_one.chars().rev().collect(), num_two.chars().rev().collect())
}

```

* **Optimized Routing:** Bypasses deep loops immediately for constant scalar transformations.
* **The Reversal Optimization:** By reversing characters prior to execution, the code ensures index increments map sequentially to standard numeric positional values, completely eliminating complex offset tracking equations like `len - 1 - i`.

### 2. Nested Columnar Processing & Memory Modification

```rust
let pro: u64 = ((xoc as u8 - 48) * (xtc as u8 - 48)) as u64 + carry;
let rem: u64 = pro % 10;
carry = pro / 10;

```

* **ASCII Demodulation:** Reinterprets the underlying character byte pointer. Subtracting `48` (decimal equivalent of the ASCII character `'0'`) shifts the byte representation cleanly to its literal decimal representation (`u8`).
* **Carry Decomposition:** Uses truncation division (`/ 10`) and remainder modulo operations (`% 10`) to slice the raw calculation into standard decimal base-10 segments.

```rust
total = xoi + xti;
sum_arr[total] += rem;

if sum_arr[total] > 9 {
    let sum: u64 = sum_arr[total];
    sum_arr[total] = sum % 10;
    sum_arr[total + 1] = sum_arr[total + 1] + sum / 10;
}

```

* **Dynamic Memory Accumulation:** Adds the single-digit remainder value directly to the storage bucket at `sum_arr[total]`.
* **In-Place Overflow Correction:** If historical row accumulation pushes the value of a target bucket past single-digit bounds (`> 9`), a secondary carry event handles the overflow, updating the immediate target and cascading the excess values forward to the `total + 1` bucket.

### 3. High-Order Carry Flush

```rust
    sum_arr[total + 1] += carry;
    carry = 0;
    total = 0;
}

```

* **End-of-Row Stabilization:** Once an entire row iteration over `string_one` terminates, any lingering `carry` leftover from the most significant bit calculation is dumped into the trailing buffer index (`total + 1`). The tracking invariants are then reset back to `0` to accept the next column wave.

### 4. Zero-Stripping & Re-Serialization Loop

```rust
let mut xi: usize = sum_arr.len() - 1;
let mut saw_digit: bool = false;

loop {
    if sum_arr[xi] == 0 && !saw_digit { xi -= 1; continue; }
    saw_digit = true;
    
    // Convert decimal values back to explicit ASCII characters
    pro_string.push((sum_arr[xi] as u8 + 48) as char);
    if xi == 0 { break; }
    xi -= 1;
}

```

* **Leading-Zero Evaluation:** The maximum possible allocation buffer size for a product array is $N + M + 1$. Because many operations do not fill this highest slot, the string would render with dead leading zeros (e.g., `00456`).
* **State Filtering:** The `saw_digit` state flag monitors scanning elements from top to bottom (`len - 1` down to `0`). It skips over elements until it encounters the first non-zero true integer digit, turning the tracking flag to `true` and allowing all following zeros (such as the interior zeros in `1002`) to write normally.

---

## ⚙️ Memory Mapping

The framework prioritizes a zero-cost stack-to-heap allocation boundary profile.

```text
STACK STORAGE                               HEAP STORAGE
┌────────────────────────────┐              ┌─────────────────────────────────┐
│ sum_arr: Vec<u64>          ├─────────────►│ [0, 0, 0, 0, 0, 0, 0]           │
│ carry: u64 = 0             │              │ Fixed buffer of Size N + M + 1  │
│ total: usize = 0           │              └─────────────────────────────────┘
├────────────────────────────┤              ┌─────────────────────────────────┐
│ pro_string: String         ├─────────────►│ "56088"                         │
│                            │              │ Dynamic Output String Array     │
└────────────────────────────┘              └─────────────────────────────────┘

```

1. **`sum_arr` Accumulator:** Instantiated as an explicit heap-allocated array initialized to zeros. The processing layer never alters the physical boundaries of this buffer block, ensuring clean cache lookups during linear reads.
2. **ASCII Remapping Zone:** Reassembling the output string passes the raw scalar integers directly through `(val + 48) as char`. This maps the internal decimal results cleanly back into valid string text arrays ready for transmission to output streams.

---

## ⏱️ Mathematical Complexity Profiles

### Time Complexity: $\mathcal{O}(N \times M)$

* **Loop Boundaries:** The system operates via strict nested loops. The outer loop executes exactly $M$ steps (length of `string_two`), while the inner loop executes exactly $N$ steps (length of `string_one`).
* **Internal Operations:** Element parsing, bit adjustments, index lookup steps, and overflow evaluations operate via fixed register operations inside the core loops, calculating down to a constant execution layout of $\mathcal{O}(1)$.

### Space Complexity: $\mathcal{O}(N + M)$

* **Buffer Footprint:** The vector array size scales proportionally with the combined length profile of the two source values ($N + M + 1$), allocating a stable, deterministic space profile that easily handles exceptionally large input sets.
