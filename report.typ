#import "@local/my_hw:0.0.1": *

#show: homework.with(title: "形式化方法大作业")

这是2025Spring的形式化方法大作业。

#show outline.entry.where(level: 1): set block(above: 2em)
#outline(title: "大纲" ,indent:  n => {n * 2em},)

= Simple NuSMV

== 目标

我们要完成一个类似NuSMV的程序，它改变了一些NuSMV的语法，并且只取了NuSMV的一小个子集，不妨叫他ssmv(for simple smv)，但是它的文法可能会有所改变(为了更简单地写编译器)，并且在最后使用它完成死锁的证明。

形象且概括地说，它的架构如下：

#figure(image("fig/start.svg"), caption: "程序架构图")

== ROBDD 

== ssmv的文法

整个程序的文法如下,注意前有\_的是非终止符号，以及\_case\_list处有个\_，这个是default的符号：
#par("")

#grid(columns: (1fr, 1fr))[
$"_program" -> "_vad _spec" \
"_vad" -> "(_var|_assign|_define) _vad" | epsilon \
"_var" -> "VAR _decl" \
"_decl" -> "id: int = _num_range; _decl" | "id: bool _decl" | epsilon \
"_num_range" -> "num..num" | "num"\
"_assign" -> "ASSIGN _ni"\
"_ni" -> "(_init | _next) _ni" | epsilon \
"_init" -> "init(id) := num|id|True|False;"$][
$"_next" -> "next(id) := (_case_stmt | {_num_list});"\
"_num_list" -> "num _num_list" | epsilon \
"_case_stmt" -> "case _case_list esae"\
"_case_list" -> "id = (num | _) : (num | {_num_list});_case_list"\
"_define" -> "DEFINE _def_list" \
"_def_list" -> "id := id in {_num_list} _def_list" \
"_spec" -> "CTLSPEC ctlexpr _spec" | epsilon \
$
]

在词法分析阶段，我们提取出所有的token，并且识别num/id/ctlexpr，ctlexpr将在CTL解析处使用。

== CTL的文法

我们按照下面的产生式来定义CTL的文法，不难发现它是一个LL(1)文法，这将有利于我们解析它。

#par("")

#grid(columns: (1fr, 1fr))[$S &-> I \ I &-> O -> I \ I &-> O \
O &-> A | O \ O &-> A \ A &-> N \& A \ A &-> N \
N &-> ! N \ N &-> P \ P &-> "AG"(S) \ P &-> "AF"(S)
\ P &-> "AX"(S)$][$P &-> "EG"(S) \ P &-> "EF"(S) \ P &-> "EX"(S) \ P &-> A[ S "U" S ] \ P &-> E[ S "U" S] \
P &-> (S) \ P &-> T \ P &-> F \ P &-> id$]

== Rust

我们将会使用Rust来编写这个程序，尽管已经存在flex和bison这类能高效完成(并且也在编译原理课程上使用过的)lexer/parser工作的工具，这是因为如下考量：

+ Cargo，因为我不怎么会写Cmake，并且可以解决麻烦的依赖问题。
+ 可以编译成WASM： 如果有机会我想把这个工具包装到网站上。
+ 最后，也是最重要的，因为我还没试过使用Rust做过比较大的项目，所以想试试看。

== 死锁证明

= Other Project
