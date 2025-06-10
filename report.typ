#import "@local/my_hw:0.0.1": *

#show: homework.with(title: "形式化方法大作业")

这是2025Spring的形式化方法大作业，包含ROBDD实现(大作业I)和死锁验证(大作业II)。

#show outline.entry.where(level: 1): set block(above: 2em)
#outline(title: "大纲" ,indent:  n => {n * 2em},)

#import "@preview/algo:0.3.6": algo, i, d, comment, code


= 两个大作业的总目标

我们要完成一个类似NuSMV的程序，它改变了一些NuSMV的语法，并且只取了NuSMV的一小个子集，不妨叫他ssmv(for simple smv)，它的文法会有一些改变(为了更简单地写编译器)，这些改变会带来更严格的程序，并且在使用它完成死锁的证明。

(在编写这一段时还没完成，其实非常可能存在性能问题，理想状态是能完成complicated case，否则我们只能退而求其次完成普通问题，complicated版本交给正统NuSMV)

所以技术上来说，两个作业如下：

+ 自动化死锁证明
+ Simple SMV 求解死锁

= 死锁证明

== 目标

这部分的目标是完成Slides 5.1.4的死锁证明，我们可以将问题描述成一个图：

- 有v个顶点(终端)
- 顶点之间可能有信道（边），共有e个
- 信道上可能正在传输(值为目的地)，也可能为空。

== 算法

我们可以写出算法如下：(为什么不用TRANS: 因为我是先写SSMV才写这个，然而SSMV里根本没有实现TRANS...)

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
  
  这里开始我们用自然语言，因为不是很好写的很简短 \ 
  接着我们先做VAR部分，对每个信道，声明|main|这么长的range \
  我们按数字大小映射到各自的值。 \
  此外，我们还要有一个signal，这是用来手动互斥的，它的值后面会说 \
  接着我们处理INIT部分：全部置为0即可。 \
  对map里的键值对的Vec里的每个元素，我们有三元组 \
  (from, to , cur )，我们计算它共有多少项，并作为signal的取值范围。 \
  并且，我们在next(cur)里加上一个case，处理这部分的逻辑。 \
  最后，我们加上!EF!(所有存在的条件的并)作为CTLSPEC。
]

简单来说，如果按照课件里的模型，取main={1,2,3}：
#image("fig/simple_lock.png", height: 100pt)

我们可以写出这样的片段：(由于SSMV语法与NuSMV不尽相同，我们会有两种输出方式可以生成适合的内容)

```text
MODULE main
VAR
	c1_2: 0..4;
	c2_3: 0..4;
	c3_4: 0..4;
	c4_1: 0..4;
  hidden: 1..15;
INIT
	c1_2 = 0 &
  c2_3 = 0 &
  c3_4 = 0 &
  c4_1 = 0;
ASSIGN
  init(hidden) := 1..15;
  next(hidden) := 1..15;

  next(c1_2) := 
    case
      hidden = 1 & c1_2 = 0 : 2;              
      hidden = 2 & c1_2 = 0 : 3;
    ... //这里省略一大部分
      hidden = 15 & c3_4 = 2 & c4_1 = 0 : 2;
      TRUE : c4_1;                
    esac;
CTLSPEC !EF!(
	(c1_2 = 0) |
  ...
	(c3_4 = 2 & c4_1 = 0) 
)
```

== 实现

如同上面所说，其实我是先写的SSMV，写了一大半才开始写的这部分，所以为什么使用Rust也和#link(<pest>)[这里]一致，这里不再赘述。



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


== Acknowledgements

本实验使用了rust库graphlib/pest作为图的实现和lexer/parser的实现。