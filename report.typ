#import "@local/my_hw:0.0.1": *

#show: homework.with(title: "形式化方法大作业")

这是2025Spring的形式化方法大作业，包含ROBDD实现(大作业I)和死锁验证(大作业II)。

#show outline.entry.where(level: 1): set block(above: 2em)
#outline(title: "大纲" ,indent:  n => {n * 2em},)

= 两个大作业的总目标

我们要完成一个类似NuSMV的程序，它改变了一些NuSMV的语法，并且只取了NuSMV的一小个子集，不妨叫他ssmv(for simple smv)，它的文法会有一些改变(为了更简单地写编译器)，这些改变会带来更严格的程序，并且在最后使用它完成死锁的证明。

= Simple SMV & ROBDD

形象且概括地说，它的架构如下：

#figure(image("fig/start.svg"), caption: "程序架构图")

== Rust & pest

我们将会使用Rust来编写这个程序，尽管已经存在flex和bison这类能高效完成(并且也在编译原理课程上使用过的)lexer/parser工作的工具，这是因为如下考量：

+ Cargo，因为我不怎么会写Cmake，并且可以解决麻烦的依赖问题。
+ 可以编译成WASM： 如果有机会我想把这个工具包装到网站上。
+ 最后，也是最重要的，因为我还没试过使用Rust做过比较大的项目，所以想试试看。

我们将使用#link("https://docs.rs/pest/latest/pest/index.html")[pest. The Elegant Parser]<pest>作为Parser编写时的工具，避免大量且繁琐的重复工作（尽管我已经花了近8个小时在手动编写Parser上，后来太麻烦了所以废弃了，好在并非全部无用功，ctl的解析部分保留了下来）

具体的pest语法可以见#link(<pest>)[上面]，总得来说我们可以写出下面的语法文件：

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

== 状态转移图

我们根据解析得到的ssmv文法，可以以下面的算法绘制状态转移图。

#[
#import "@preview/algo:0.3.6": algo, i, d, comment, code

#algo(
  title: "FSM_build",
  parameters: ("AST",)
)[
  对所有AST里的VarDecl，构造
]

]

== ROBDD-CTL

ROBDD在Slides5.2.1-2中有详细的描述，我们这里只简单的说明。

ROBDD是一种以bool值为函数的压缩表示方法，它具有以下特点：
- Ordered
- Reduced

通俗地说，它能将某些布尔值组成的函数极大压缩地表示，并且我们一般递归地生成它，下面是一个rust的代码段，它是本次实验中生成ROBDD图的方式

= Simple SMV死锁证明
