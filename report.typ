#import "@local/my_hw:0.0.1": *

#show: homework.with(title: "形式化方法大作业")

这是2025Spring的形式化方法大作业，包含一个死锁验证(大作业I)和简单的SMV(大作业II)。

#show outline.entry.where(level: 1): set block(above: 2em)
#outline(title: "大纲" ,indent:  n => {n * 2em},)

#import "@preview/algo:0.3.6": algo, i, d, comment, code
#import "@preview/codly:1.3.0": codly

= 两个大作业的总目标

我们要完成一个类似NuSMV的程序，它改变了一些NuSMV的语法，并且只取了NuSMV的一小个子集，不妨叫他ssmv(for simple smv)，它的文法会有一些改变(为了更简单地写编译器)，这些改变会带来更严格的程序，并且在使用它完成死锁的证明。

所以技术上来说，两个作业如下：

+ 自动化死锁证明
+ Simple SMV 求解死锁 (Simple SMV真的写了很久很久...)

所有代码可以在#link("https://github.com/CelesyChen/Formal_Method")[Github]上访问，由于刚开始的设计问题，文件组织有点乱，简单来说src里的代码是SSMV验证部分，deadlock里则是死锁。

= 死锁证明

== 目标

这部分的目标是完成Slides 5.1.4的死锁证明，我们可以将问题描述成一个计算机网络上的图：

- 有v个顶点(主机+路由器)
- 顶点之间可能有信道（边），共有e个
- 信道上可能正在传输(值为目的地)，也可能为空。
- 只有主机(host)可以发送、接受内容

== 算法

我们可以写出算法如下：

#algo(
  title: "自动死锁证明",
  parameters: ("graph", "host"),
  indent-size: 1em,
  breakable: true
)[
  \/\/ n是顶点数 edges是所有信道（有向），他们构成了graph  \
  \/\/ host是所有会发送数据的顶点，也是其它顶点会发往的地方 \
  HashMap< pii, Vec< i32>> map #comment("pii 是pair<i32, i32>") \
  for dst in host { #i \
    for src != dst in 1..n { #i \
      sp =shortest_path(graph, src, dst) \
      for edge in sp { #i \
        map[{src, dst}].push(edge)
        #d \
      } #d \
    } #d \
  } \
  
  这里开始我们用自然语言说明，因为写得形式化后也和真代码很接近了 \ 
  接着我们先做VAR部分，对每个信道，声明|host|这么长的range \
  我们按数字大小映射到各自的值。 \
  此外，我们还要有一个signal，这是用来手动互斥的，它的值后面会说 \
  接着我们处理INIT部分：全部置为0即可。 \
  对map里的键值对的Vec里的每个元素，我们有三元组 \
  (from, to , cur )，我们计算它共有多少项，并作为signal的取值范围。 \
  并且，我们在next(cur)里加上一个case，处理这部分的逻辑。 \
  最后，我们加上AG(所有存在的条件的并)作为CTLSPEC。 \
  在NuXMV，我们直接使用INVARSPEC，它们逻辑一致。
]

#question(title: "为什么不用TRANS")[
因为我是先写SSMV才写这个，然而SSMV里根本没有实现TRANS...
]

简单来说，如果按照课件里的模型，取main={1,2,3}：
#image("fig/simple_lock.png", height: 100pt)<simple>

我们可以生成这样的片段：(由于SSMV语法与NuSMV不尽相同，我们会有两种输出方式可以生成适合的内容)

#grid(columns: 2)[
```text
MODULE main
  VAR
    ch1_2: 0..3;
    ch2_3: 0..3;
    ch3_4: 0..3;
    ch4_1: 0..3;
    signal: 0..13;
  INIT
    ch1_2 = 0 &
    ch2_3 = 0 &
    ch3_4 = 0 &
    ch4_1 = 0;
  ASSIGN
    init(signal) := 0..13;
    next(signal) := 0..13;

    next(ch1_2) := 
      case
        signal = 5 & ch1_2 = 0 : 2;
        signal = 6 & ch1_2 = 2 : 0;
        signal = 9 & ch4_1 = 2 & ch1_2 = 0 : 2;
        signal = 10 & ch1_2 = 0 : 3;
        signal = 11 & ch1_2 = 3 & ch2_3 = 0 : 0;
        TRUE : ch1_2;
      esac;

    next(ch4_1) := 
      case
        signal = 3 & ch3_4 = 1 & ch4_1 = 0 : 1;
        signal = 4 & ch4_1 = 1 : 0;
        signal = 8 & ch3_4 = 2 & ch4_1 = 0 : 2;
        signal = 9 & ch4_1 = 2 & ch1_2 = 0 : 0;
        TRUE : ch4_1;
      esac;
```
][
#codly(offset: 34)
```text
    next(ch3_4) := 
      case
        signal = 1 & ch2_3 = 1 & ch3_4 = 0 : 1;
        signal = 2 & ch3_4 = 0 : 1;
        signal = 3 & ch3_4 = 1 & ch4_1 = 0 : 0;
        signal = 7 & ch3_4 = 0 : 2;
        signal = 8 & ch3_4 = 2 & ch4_1 = 0 : 0;
        TRUE : ch3_4;
      esac;

    next(ch2_3) := 
      case
        signal = 0 & ch2_3 = 0 : 1;
        signal = 1 & ch2_3 = 1 & ch3_4 = 0 : 0;
        signal = 11 & ch1_2 = 3 & ch2_3 = 0 : 3;
        signal = 12 & ch2_3 = 0 : 3;
        signal = 13 & ch2_3 = 3 : 0;
        TRUE : ch2_3;
      esac;

  INVARSPEC ( 
    ( ch1_2 = 0 ) | 
    ( ch1_2 = 2 ) | 
    ( ch4_1 = 2 & ch1_2 = 0 ) | 
    ( ch1_2 = 3 & ch2_3 = 0 ) | 
    ( ch3_4 = 1 & ch4_1 = 0 ) | 
    ( ch4_1 = 1 ) | 
    ( ch3_4 = 2 & ch4_1 = 0 ) | 
    ( ch2_3 = 1 & ch3_4 = 0 ) | 
    ( ch3_4 = 0 ) | 
    ( ch2_3 = 0 ) | 
    ( ch2_3 = 3 )
  )

```
]

== 实现

这部分的实现使用C++，这其实很好理解，因为它很简单，并不需要使用任何的库，所以使用更为熟悉的C++。

我们采用下面的输入格式，其中v为顶点数，e为信道数，h为主机数，右边是对#link(<simple>)[这个模型]的简单示例：
#grid(columns: 2)[
```text
v e h
1 2
... 共e个
3 4
1 2 3 (共h个)
```
][
```text
4 4 3
1 2
2 3
3 4
4 1
1 2 3
```
]

我们一步步解释，首先是类的定义

```cpp
class Graph {

public:
  bool ssmv = 0;
  int v; // 0..v-1
  int signal_cnt;
  vector<vector<int>> channels;
  vector<int> hosts;
  // 前面： 序号 后面： 编号
  unordered_map<pii, vector<pii>> path; // 由于这个的存在我们需要给pii定义Hash，具体方法不赘述

  void find_path();
  void VAR();
  void INIT();
  void WRITE();
};
```

在main函数里，直接读入即可，接着调用find_path后调用WRITE。
// #pagebreak()

- find_path: 找出所有host的APSP和路径，显然无权图边权设置为1，使用Dijkstra算法即可，具体程序我们不赘述。
- WRITE: 依次写入VAR的声明，INIT的初始化，ASSIGN部分的转换，以及最后CTLSPEC部分，我们在接下来的部分详细说明。

/ APSP: All Pair Shortest Path

还是刚才的例子，生成出来内容的可以参考#link(<simple>)[这里]

=== VAR

对于每一个信道，或者说边，我们维护一个变量，命名为chx_y，其中x和y为顶点id。

需要注意的是，我们的实现是全双工的。所以对于双向信道，只需视为两个单向即可。

此外，对于每一个可能的状态，维护status里的一个值，实际上它是为了模拟随机值。

=== INIT

对于每一个刚才初始化出来的信道，我们为其赋值0，status则先不管。

=== ASSIGN

在assign里，事情开始复杂了起来，我们遵循以下步骤来生成：

+ 改变数据结构，每一条路径上发送不同可能的值，并且去重。
+ 对所有可能值(from, to, dst)，按照以下逻辑来assign
  - dst = to: 吸收
  - dst $eq.not$ to: 给from加一个转发，给to加一个转发

status的值在这里随机赋值，换句话说： \
init(signal) := 0..sig_max; \
next(signal) := 0..sig_max;

=== CTL

将所有出现过的逻辑表达式并，并且加上AG。

== 测试

我们先使用NuXmv来测试正确性，测试命令如下：
```bash
read_model -i temp.smv
flatten_hierarchy
encode_variables
build_boolean_model
check_invar_ic3
```

注意到我们这里使用了invar(不变检测)，其实就是AG，不过这个更快一点

#info(title: "为什么使用INVARSPEC，和正确性")[
  INVARSPEC本质上是不变式性质检查，表示“某个状态谓词在所有可达状态中始终成立”，换句话说：目标是证明“坏状态”不可达。

  CTLSPEC AG是在所有路径上的所有状态都满足，验证需要使用fixed point computation。

  显然，二者在逻辑上是等价的，我们使用更快的(或者说使用CTLSPEC AG在大图上容易状态空间爆炸导致完全算不出来)
]

接着我们对“死锁验证”这个Slides里的所有测试样例进行验证，分别是

#grid(columns: (1fr, 1fr))[
  #image("fig/simple_lock.png")
  host取{1,2,3}
][
  #image("fig/complex_lock.png")
  host取{1, 5, 9, 13} 和 {2, 4, 6}
]

得到下面的结果：

#grid(columns: 2, row-gutter: 20pt,
  [#figure(image("fig/s_lock_test.png"), caption: "simple {1,2,3}")],
  [#figure(image("fig/c_lock_test1.png"), caption: "complex {1, 5, 9, 13}")],
  [#figure(image("fig/c_lock_test2_1.png"), caption: "complex {2, 4, 6} p1") \ 中间省略一部分 ],
  [#figure(image("fig/c_lock_test2_2.png"), caption: "complex {2, 4, 6} p2")]
)

== 小结

在这部分我们完成了死锁的自动化验证生成，得益于NuSMV(nuXmv)的强大性能，我们可以很快地验证它们，在下一节，我们将要使用自己的SSMV来证明这些内容。

#pagebreak()

= Simple SMV & ROBDD

形象且概括地说，它的架构如下：

#info(title: "程序架构图")[#image("fig/start.svg")]

== Rust & pest

我们将会使用Rust来编写这个程序，尽管已经存在flex和bison这类能高效完成(并且也在编译原理课程上使用过的)lexer/parser工作的工具，这是因为如下考量：

#question(title: "为什么使用rust？")[

+ Cargo，因为我不怎么会写Cmake，并且可以解决麻烦的依赖问题。
+ 可以编译成WASM： 如果有机会我想把这个工具包装到网站上。
+ 最后，也是最重要的，因为我还没试过使用Rust做过比较大的项目，所以想试试看。

]
我们将使用#link("https://docs.rs/pest/latest/pest/index.html")[pest. The Elegant Parser]<pest>作为Parser编写时的工具，避免大量且繁琐的重复工作（尽管我已经花了至少8个小时在手动编写Parser上，后来太麻烦了所以废弃了，好在并非全部无用功，ctl的解析部分保留了下来）

具体的pest语法可以见#link(<pest>)[上面]，总地来说我们可以写出下面的语法文件：

== ssmv的文法

我曾尝试完整且规范地表达出来这个文法，但太过麻烦，所以我们不妨直接看pest文件，并且在上面解释：

```text
WHITESPACE = _{ " " | "\t" | NEWLINE }
NEWLINE = _{ "\n" | "\r\n" }
COMMENT = _{ "--" ~ (!NEWLINE ~ ANY)* }

program = { SOI ~ (module_decl)* ~ EOI }

module_decl = { "MODULE" ~ identifier ~ module_body }

module_body = { (var_decl | define_decl | assign_block | init_block | spec_decl)* }

var_decl = _{ "VAR" ~ var_list+ }
define_decl = _{ "DEFINE" ~ define_list+ }
assign_block = _{ "ASSIGN" ~ assign_list+ } // assign 现在不能处理init
init_block = _{ "INIT" ~ init_list+ } 
spec_decl = _{ spec_list+ }

// 我们舍弃了TRANS，现在只能在assign里定义转移，这能提高语义的正交性

var_list = { identifier ~ ":" ~ type ~ ";" }
define_list = { identifier ~ ":=" ~ identifier ~ "in" ~ atom_list ~ ";" } // 这里需要在语义分析时确认类型正确
assign_list = { identifier ~ ":=" ~ (case_assign | single_assign) ~ ";" } 
init_list = { identifier ~ ":=" ~ atom ~ ";" } // 所有init以 ; 结尾
spec_list = { "CTLSPEC" ~ ctl_term ~ ";" }

type = { // 现在需要更明确地定义变量
  Bool
  | Int ~ "[" ~ number ~ ".." ~ number ~ "]"
  | Enum ~ "{" ~ identifier ~ ("," ~ identifier)* ~ "}"
}

Bool = { "bool" }
Int = { "int" }
Enum = { "enum" }

case_assign = { ^"case" ~ case_item+ ~ ^"esac" }
single_assign = { atom } 

case_item = {  expr ~ ":" ~ atom_list ~ ";" } // 为了降低复杂度，我们只支持 & | == !=和()组成的式子

expr    = { or_expr ~ (And ~ or_expr)* }
or_expr     = { eq_expr ~ (Or ~ eq_expr)* }
eq_expr = { primary_expr ~ ((Eq | Neq) ~ primary_expr)* }
primary_expr = {
    "(" ~ expr ~ ")"
  | atom
}

And = { "&" }
Or = { "|" }
Eq = { "==" }
Neq = { "!=" }

atom_list = { atom_set | atom }
atom_set = _{ "{" ~ atom ~ ("," ~ atom)* ~ "}" }

atom = { TRUE | FALSE | identifier | number }

TRUE = { ^"true" }
FALSE = { ^"false" }

identifier = @{ ASCII_ALPHA ~ ( ASCII_ALPHANUMERIC | "_")* }
number = @{ ASCII_DIGIT+ }

ctl_term = { (!";" ~ ANY )* } // 可以看到ctl会被单独处理，并且我们不支持LTL
```

== CTL的文法

我们按照下面的产生式来定义CTL的文法，不难发现它是一个LL(1)文法，这将有利于我们解析它。

#par("")

#grid(columns: (1fr, 1fr))[$S &-> I \ I &-> O -> I \ I &-> O \
O &-> A | O \ O &-> A \ A &-> N \& A \ A &-> N \
N &-> ! N \ N &-> P \ P &-> "AG"(S) \ P &-> "AF"(S)
\ P &-> "AX"(S)$][$P &-> "EG"(S) \ P &-> "EF"(S) \ P &-> "EX"(S) \ P &-> A[ S "U" S ] \ P &-> E[ S "U" S] \
P &-> (S) \ P &-> T \ P &-> F \ P &-> id = "atom" \ "atom"&-> id \ "atom"&-> "num" $]

== ROBDD

ROBDD在Slides5.2.1-2中有详细的描述，我们这里只做简单的说明。

ROBDD是一种以bool值为函数的压缩表示方法，它具有以下特点：
- Ordered
- Reduced

通俗地说，它能将某些布尔值组成的函数极大压缩地表示，并且我们一般递归地生成它，我们可以用下面的逻辑生成。

#algo(
  title: "BDD_build",
  parameters: ("AST",)
)[
  对所有AST里的VarDecl，构造符号表并检查表达式是否合法 \
  使用频率来降序排序，这是一个简单的启发式，为了避免状态空间爆炸 \
  对所有的Variable，我们分配布尔变量给它 \
  开始递归生成状态转换的ROBDD，具体实现我们会在下面说明 \
  调用CTLspec部分，在ROBDD上操作 \
  输出结果的ROBDD，它可以说明证明与否。
]

== ROBDD-CTL

ROBDD就是为了求解CTL的工具，所以在CTL上的适配性很好，以EU为例子，我们只需要（参考Slides 5.2.1）：
```rust
  pub fn eval_ctl_eu(&mut self, phi_l: NodeId, phi_r: NodeId, trans: NodeId) -> NodeId {
    let mut z = phi_r;  
    let mut iter = 0;
    loop {
      let z_prime = self.rename_vars(z, true); 
      // step 1
      let pre_image = self.apply(OpType::And,trans, z_prime);
      // step 2
      let pre_states = self.exist_quantify(pre_image);
      let phi1_and_pre = self.apply(OpType::And, phi_l, pre_states);
      // step 3
      let new_z = self.apply(OpType::Or, phi_r, phi1_and_pre);

      if new_z == z { break; }
      z = new_z;
    }
    z
  }
```

即可。

== 实现

#strike("实现真的费时又费力，并且在最开始的实现中总是遇到状态空间爆炸问题（对一个非常简单的图算了4GB的数据）")

我们一步一步来解释

=== main

看main函数可以知道我们每一步干了什么：
```rust
fn main() {
  let unparse_file = fs::read_to_string("tests/test.ssmv").expect("cannot open file"); // read file
  let pairs = SSVMParser::parse(Rule::program, &unparse_file).expect("parse failed"); // parse
  let ast = build_ast(pairs.peek().unwrap()); // build ast
  let a = table::to_symbol_table(&ast); // build symbol table
  for st in a {
    let order = st.suggest_variable_order(); // get order 
    let bv = st.to_bv_with_order(order); // 映射变量到虚拟变量上(虚拟变量是有序的)
    let mut manager = BddManager::new(bv);
    let trans = manager.encode_whole_transition(&st); // 构造整个程序的状态转换ROBDD
    for ctl in st.specs { // 对每一个spec验证
      let a = manager.ctl_to_bdd(&ctl, trans);
      manager.print_bdd(a, 0); // 打印结果
    }
  }
}
```

=== Lexer / Parser

作为一个语言，虽然是简单版，我们仍然需要一个编译器，在一开始，我选择手搓编译器，事实证明这并不是个好想法，在最终成品里，只有CTL部分的编译器是手搓的，SSVM主体的编译器使用了pest库，这样就不用从tokenizer开始一个一个完成，可以直接跳到构造AST的部分。

=== AST 生成——SSVM部分

在生成AST时，我们实际上是想要生成一个能描述整个状态转换过程的数据结构，源码在src/parser/ssvmparser.rs文件里，我们重点介绍结构：

#grid(columns: 2)[
```rust
#[derive(Debug, Clone)]
pub enum AstNode {
  Program(Vec<AstNode>),
  ModuleDecl { name: String, body: Vec<AstNode> },
  VarDecl { id: String, ty: SVMType },
  DefineDecl(String, String, Vec<Atom>), // id := id in {...}
  Assign(String, AssignExpr),            // next(x) := ...
  Init(String, Atom),
  Spec(String),
}

#[derive(Debug, Clone)]
pub enum SVMType {
  Bool,
  Int(u32, u32), // from to
  Enum(Vec<String>),
}

#[derive(Debug, Clone)]
pub enum AssignExpr {
  Case(Vec<CaseItem>),
  Single(Vec<Atom>),
}
```
][
#codly(offset: 23)
```rust
#[derive(Debug, Clone)]
pub struct CaseItem {
  pub expr: Expr,
  pub result: Vec<Atom>,
}

#[derive(Debug, Clone)]
pub enum Atom {
  Bool(bool),
  Id(String),
  Num(u32),
}

#[derive(Debug, Clone)]
pub enum Expr {
  Or(Box<Expr>, Box<Expr>),
  And(Box<Expr>, Box<Expr>),
  Eq(String, Atom),
  Ne(String, Atom),
  True(bool),
}
```
]

可以见到，一个程序由若干个Module组成（不过虽然有Module，但是我没有做任何Module相关的调用的内容）；Module内则是各种声明，在这个部分，我们只是简单地将信息全部收集起来，并且在接下来放到SymbolTable的生成里来得到我们想要的“状态转换描述”。

=== AST 生成——CTL部分

CTL的生成很简单，就是嵌套而已：

```rust
pub enum CtlAst {
    And(Box<CtlAst>, Box<CtlAst>),
    Or(Box<CtlAst>, Box<CtlAst>),
    Implies(Box<CtlAst>, Box<CtlAst>),
    Not(Box<CtlAst>),
    AG(Box<CtlAst>),
    EG(Box<CtlAst>),
    AX(Box<CtlAst>),
    EX(Box<CtlAst>),
    AF(Box<CtlAst>),
    EF(Box<CtlAst>),
    AU(Box<CtlAst>, Box<CtlAst>),
    EU(Box<CtlAst>, Box<CtlAst>),
    Expr(String, Atom),
    True,
    False,
}
```

值得一提的是，我们在CtlAst的基础上直接进行了原本应在ROBDD验证时进行的一系列变换(例如 $"AG" phi.alt => !"EF"!(phi.alt) => !"E"( top "U" phi.alt )$)这类，这可以避免更大的开销。

=== SymbolTable生成

在src/table.rs里，我们将ssvm ast转化为一个SymbolTable，它在事实上存储了所有状态变换信息，并且可以很容易地编码成BDD形式，这对接下来的步骤非常有用。

我们一样来看它的数据结构有哪些:
#grid(columns: 2)[
```rust
#[derive(Debug)]
pub struct SymbolTable {
  pub contain: HashMap<String, Variable>,
  pub specs: Vec<CtlAst>
}

#[derive(Debug)]
pub struct Variable { 
  pub domain: Range,
  pub init: u32,
  pub next: HashMap<Val, Expr>,
  pub mapping: Option<HashMap<String, u32>>, // only for enum
}

```
][
  #codly(offset: 13)
```rust
#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Val {
  Num(u32),
  Id(String),
}


#[derive(Debug)]
pub struct Range {
  pub start: u32,
  pub end: u32,
} 
```
]

组织应该很明确，将AST变成一系列Variable，并且都有初始值，状态转换值，范围等，enum类型在这里会被映射到整数上，这是为了后面处理的统一性。

=== ROBDD

接下来是本次实验的重点，ROBDD的生成，事实上，在SymbolTable的生成结束后，我们会运行一次简单的程序分析，提取出词频最高的变量，并且降序地给出变量顺序列表，这会被我们在生成布尔变量时采纳：

```rust
impl SymbolTable {
  pub fn suggest_variable_order(&self) -> Vec<String> {
    let mut freq: HashMap<String, usize> = HashMap::new();

    for (name, var) in &self.contain {
      *freq.entry(name.clone()).or_insert(0) += var.next.len();

      for (val, cond) in &var.next {
        collect_vars_from_expr(cond, &mut freq);
        if let Val::Id(other_name) = val {
          *freq.entry(other_name.clone()).or_insert(0) += 1;
        }
      }
      *freq.entry(name.clone()).or_insert(0) += 1;
    }

    let mut vars: Vec<_> = freq.into_iter().collect();
    vars.sort_by(|a, b| b.1.cmp(&a.1)); 

    vars.into_iter().map(|(v, _)| v).collect()
  }
}
```

紧接着，我们采纳这个顺序，并映射到虚拟的变量上，其实没有什么好讨论的，最重要的就是我们采用了一种启发式的顺序，以及我们的映射方式是吧x和x'的每一位交错放在一起的，这是因为它们有很大概率相关(或者说必定相关)，紧凑的排布可以让状态空间爆炸的可能降低。
```rust 
#[derive(Debug)]
pub struct BoolVariables {
  pub var_to_idx: HashMap<String, Range>,
  pub idx_to_var: Vec<String>,
}
impl SymbolTable {
  pub fn to_bv_with_order(&self, order: Vec<String>) -> BoolVariables {
    let mut next: usize = 0;
    let mut bv = BoolVariables {
      var_to_idx: HashMap::new(),
      idx_to_var: Vec::new(),
    };
    for name in order {
      let name_prime = name.clone() + "'";
      let var = self.contain.get(&name).expect("var not found in SymbolTable");
      let len = ((var.domain.end - var.domain.start + 1) as f32).log2().ceil() as usize;
      for _ in 0..len {
        bv.idx_to_var.push(name.clone());
        bv.idx_to_var.push(name_prime.clone()); 
      }
      bv.var_to_idx.insert(name.clone(), Range { start: next as u32, end: (next + 2 * len - 1) as u32 });
      next += 2 * len;
    }
    bv
  }
}
```

这个BoolVariable会被放到BddManager里，BddManager则是重头戏，它处理了所有ROBDD的构造和CTL验证。

由于这部分太多了(有600多行都在实现这个部分)，我们只挑选几个比较重要的：

- BddManager， 我们着重介绍它的几个成员：
  - unique_table: 使用这个，我们保证了每个BDDNode的唯一性，避免了大量冗余。
  - nodes: 所有BDDNode的集合。
  - computed_table: 使用这个来避免重复计算。
  - bv: 刚才说的BoolVariable
  - restrict_table: 也是避免重复计算，不过这个是CTL验证时用的。

```rust
#[derive(Debug)]
pub struct BddManager {
  unique_table: HashMap<(usize, NodeId, NodeId), NodeId>,
  nodes: Vec<BddNode>,
  computed_table: HashMap<(OpType, NodeId, NodeId), NodeId>, 
  bv: BoolVariables,
  restrict_cache: HashMap<(NodeId, usize, bool), NodeId>
}
```

- get_or_create_node： 所有新增节点的操作都要经过这个函数，它保证了每次新增都是必要的，代码略。

- encode_val_to_bdd: 这是一个专门负责完成a := 2这类赋值的函数，它的实现很具参考意义：

#grid(columns: 2)[
```rust
pub fn encode_val_to_bdd(&mut self, var: &str, val: u32, is_prime: bool) -> NodeId {
  let bits = self.mk_var(var, is_prime);
  let result = self.encode_val_rec( &bits, 0, 0, val,
  );
  result
}
fn encode_val_rec(
  &mut self,
  bits_indices: &[usize], 
  current_bit_idx: usize, 
  current_path_val: u32,  
  target_val: u32,       
) -> NodeId {    
  if current_bit_idx == bits_indices.len() {
    if current_path_val == target_val {
      return self.get_true_node();
    } else {
      return self.get_false_node();
    }
  }
```
][
#codly(offset: 26)
```rust
  let var_index = bits_indices[current_bit_idx]; 

  let low_child = self.encode_val_rec(
      bits_indices,
      current_bit_idx + 1, 
      current_path_val,  
      target_val,
  );
  let high_child = self.encode_val_rec(
      bits_indices,
      current_bit_idx + 1, 
      current_path_val | (1 << current_bit_idx), 
      target_val,
  );

  self.get_or_create_node(var_index, low_child, high_child) // 看！新增时调用了这个
}
```
]

类似地，我们有同样递归完成的"两个变量相等"比较。

- encode_var_next: 每个变量的变化可以被这个函数编码，想要整个状态图只需要对每一个变量调用一次在合起来即可。

```rust
pub fn encode_var_next(&mut self, var_name: &str, var: &Variable) -> NodeId {
  let mut res = self.get_false_node();

  for (val, cond_expr) in &var.next {
    let cond_bdd = self.expr_to_bdd(cond_expr); // 条件
    let val_bdd = match val { // 目标
      Val::Num(c) => {
        self.encode_val_to_bdd(var_name, *c, true)
      }
      Val::Id(other_var) => {
        self.encode_eq_var(var_name, true, other_var, false)
      }
    };
    let guarded = self.apply(OpType::And, cond_bdd, val_bdd); // 更新： 条件 And 目标
    res = self.apply(OpType::Or, res, guarded); // 所有更新的并
  }
  res
}
```

- apply: 同样参考Slides 5.2.1里面的做法，并且加上了剪枝，避免状态空间爆炸（尽管原本的实现有问题，但当时在加上后占用瞬间变为 1/30，可见它的强大）

```rust
fn apply(&mut self, op: OpType, a: NodeId, b: NodeId) -> NodeId {
    // Early reduction rules
    let t = self.get_true_node();
    let f = self.get_false_node();

    match op {
      OpType::And => {
        if a == f || b == f {
          return f;
        } else if a == t {
          return b;
        } else if b == t {
          return a;
        }
      }
      OpType::Or => {
        if a == t || b == t {
          return t;
        } else if a == f {
          return b;
        } else if b == f {
          return a;
        }
      }
      OpType::Imply => {
        if a == f || b == t {
          return t;
        } else if a == t {
          return b;
        }
      }
      _ => {}
    }

    let key = (op, a, b);
    if let Some(&cached) = self.computed_table.get(&key) {
      return cached;
    }

    let a_node = self.get_node(a).clone();
    let b_node = self.get_node(b).clone();

    let res = match (a_node, b_node) {
      (BddNode::Terminal(x), BddNode::Terminal(y)) => {
        let result = match op {
          OpType::And => x && y,
          OpType::Or => x || y,
          OpType::Imply => !x || y,
          OpType::Eq => x == y,
          _ => unreachable!(),
        };
        if result {
          self.get_true_node()
        } else {
          self.get_false_node()
        }
      }
      (BddNode::NonTerminal { variable: v1, low: l1, high: h1 },
      BddNode::NonTerminal { variable: v2, low: l2, high: h2 }) => {
        let var = v1.min(v2);
        let (l1p, h1p) = if v1 == var { (l1, h1) } else { (a, a) };
        let (l2p, h2p) = if v2 == var { (l2, h2) } else { (b, b) };

        let low = self.apply(op, l1p, l2p);
        let high = self.apply(op, h1p, h2p);
        self.get_or_create_node(var, low, high)
      }
      (BddNode::Terminal(_), BddNode::NonTerminal{ variable, low, high }) => {
        let low = self.apply(op, a, low);
        let high = self.apply(op, a, high);
        self.get_or_create_node(variable, low, high)
      }
      (BddNode::NonTerminal{ variable, low, high }, BddNode::Terminal(_)) => {
        let low = self.apply(op, low, b);
        let high = self.apply(op, high, b);
        let temp = self.get_or_create_node(variable, low, high);
        // self.print_bdd(temp, 0);
        temp
      }
    };

    self.computed_table.insert(key, res);
    res
  }
```

=== CTL验证

这个刚才其实看过了，不过我们仍然可以简单介绍一下它的部分辅助函数：

- exist_quantify: 这是用来处理step2的，主要功能就是 将各个变量附上不同的值并并在一起。

```rust
fn exist_quantify(&mut self, bdd: NodeId) -> NodeId {
  let mut result = bdd;
  let vars = self.next_vars();
  for v in vars {
    let low = self.restrict(result, v, false);
    let high = self.restrict(result, v, true);
    result = self.apply(OpType::Or, low, high);
  }
  result
}
```

- restrict: 为变量赋值，并返回新节点
```rust
fn restrict(&mut self, node: NodeId, var: usize, value: bool) -> NodeId {
  if let Some(&cached) = self.restrict_cache.get(&(node, var, value)) {
    return cached;
  }
  let node_data = self.get_node(node).clone();
  let res = match node_data {
    BddNode::Terminal(_) => node,
    BddNode::NonTerminal { variable, low, high } => {
      if variable == var {
        if value {
          self.restrict(high, var, value)
        } else {
          self.restrict(low, var, value)
        }
      } else {
        let new_low = self.restrict(low, var, value);
        let new_high = self.restrict(high, var, value);
        self.get_or_create_node(variable, new_low, new_high)
      }
    }
  };
  self.restrict_cache.insert((node, var, value), res);
  res
}
```

== 测试

在测试环节，我们使用第一个实验的死锁生成器生成的ssmv代码来检测：

#figure(image("fig/simple_lock.png"), caption: "最简单的情况")

cargo run后可以得到(正常竖着的图太长了，所以我只能倒过来)：

#image("fig/simple1.svg")

这显然是有死锁的，可以随便找到一条路径通往0.

我们看一个没有死锁的版本：

例如我们把2去掉，只剩1 3，这个可以被nuXmv验证:

#image("fig/custom_nolock.png")

我们的程序可以生成：

#image("fig/simple2.svg")

#question(title: "为什么存在0?")[
  事实上这是因为11是不合法的输入，所以11可能到0（可以见到只有实线会到0），对于所有合法的输入都到1，没有死锁存在。
  
  可见最高只有10：
  ```text
  VAR
    ch1_2: int[0..2];
    ch2_3: int[0..2];
    ch3_4: int[0..2];
    ch4_1: int[0..2];
    signal: int[0..5];
  ```
]

当然，我们也可以验证其他CTLSPEC的内容，不过我们不再赘述。

== 小结

在这个部分我们完成了SSVM并且使用它证明了死锁，虽然它的性能较差，但是仍能算是一次收获很大的实验。

#experiment(title: "总共有多少代码？")[
  挺令我讶异的，我以为远远超过这么点：
  #figure(image("fig/wc.png"), caption: "一共2038行")
]

#pagebreak()

= Acknowledgements

本实验使用了rust库pest作为lexer/parser的实现。

感谢老师和助教在本学期的付出！