# 编写自己的（玩具）编程语言

本文将记录一个小型的玩具语言开发过程。作为学习实践 Rust 的副产品。

本过程将覆盖下面一些要点：

1. 解析文本格式的源代码。
2. 构建 AST（语法树）。
3. 从语法树翻译为可执行的目标代码。
4. 执行目标代码（作为验证）。

```mermaid
flowchart LR
    A[文本源代码] -->B[AST语法树]
    B-->C[可执行目标代码]
    C-->D["通用运行时中执行"]
```

另外，也可能尝试改变为解释执行。

```mermaid
flowchart LR
    A[文本源代码] -->B[AST语法树]
    B-->D["定制解释器中执行"]
```

## 语言设计目标

第一阶段。语法非常简单，只要实现简单的算术表达式，类似于下面。

```
1 + 2
1 + 2 - 3
```

第二阶段， 实现变量。

```
let a = 2
let b = a + 2
b
```

第三阶段，实现简单的控制流。

```
let a = 0
let b = 0
if (a > 0) b = a + 2
b
```

第三阶段， 实现函数。

```
fn add(a) {
    if (a == 1) return 1
    return add(a - 1)
}
let b = add(10)
b
```

第四阶段， 实现简单的类型。

```
fn add(a:int) {
    if (a == 1) return 1
    return add(a - 1)
}
let b = add("10") // 编译错。
b
```

## 技术选型

1. 主体语言使用 Rust - 整个文章本就是作者学习 Rust 过程中的副产品。
2. 解析器使用 nom (https://github.com/Geal/nom)- nom 是 combinator 类解析库里面比较有代表性的。combinator 是以函数作为解析单位，反复组合形成整体解析。还有一种解析库是 PEG 类型的，也就是直接写出 PEG 文件定义语法，由库直接生成整体解析器。比如 pest (https://bitbegin.github.io/pest-rs/)。之所以选择combinator主要是想多用Rust写代码。
3. 通用运行时 WASM - WASM 相对新潮、通用。能在多种平台上方便地运行。社区讨论也比较活跃。相关工具比较充足。
4. WASM 构建使用 parity_wasm (https://github.com/paritytech/parity-wasm) - 很方便的底层 WASM 构建。

## 一些要点

开发过程中遵循一些原则：
1. 每个语法阶段都完成全流程，从解析到可运行。
2. AST 驱动，而不是以 parser 的实现作为先决条件。因为 parser 实现复杂，容易陷入细节。
3. 多写测试。

## 第一阶段

本阶段的语法比较简单，概念上如下图。

```mermaid
flowchart TB
    Z[表达式] --> BinaryExpr
    subgraph BinaryExpr
    A[操作符] --> B[左操作数]
    A --> C[右操作数]
    end
```

对于

```
1 + 2
1 + 2 - 3
```

```mermaid
flowchart TB
    A[+] --> B[1]
    A --> C[2]
```

```mermaid
flowchart TB
    A[-] --> B[+]
    A --> C[3]
    B --> D[1]
    B --> E[2]
```
（图一）
```rust
#[derive(PartialEq, Debug)]
pub enum Operator {
    Plus,
    Minus,
}
#[derive(PartialEq, Debug)]
pub enum Expr {
    Int(i32),
    BinaryExpr {
        op: Operator,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}
```

注意，AST 的结构与解析结果不一定是完全一一对应的。
比如， `1+2-3` 和 `(1+2)-3` 的 AST 结构都是简单的 BinaryExpr 来表示。
并不一定要把括号带到 AST 中。在 AST 构成的过程中已经处理了计算顺序的问题，
比如如果是`1+(2-3)`，就会是下面的不同 AST 结构，虽然最后计算这个算式的结果还是一样的。

```mermaid
flowchart TB
    A[+] --> B[1]
    A --> C[-]
    C --> D[2]
    C --> E[3]
```

大概在专业术语中这是两个不同的领域，分别叫编程语言的词法和语法，我没有认真学习过编译原理，此处简单提一下，不再纠结。


```
├── Cargo.lock
├── Cargo.toml
├── README.md
├── src
│   ├── ast.rs
│   ├── main.rs
│   └── parsers
│       ├── mod.rs
│       └── test.rs
```

