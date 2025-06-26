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
+ Simple SMV 求解死锁

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
  对所有的Variable，我们分配布尔变量给它 \
  使用DFS分配各个布尔变量的顺序

]


== ROBDD-CTL


== 实现

=== CTL



== Acknowledgements

本实验使用了rust库pest作为lexer/parser的实现。