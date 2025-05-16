#import "@local/my_hw:0.0.1": *

#show: homework.with(title: "形式化方法大作业")

这是2025Spring的形式化方法大作业。

#show outline.entry.where(level: 1): set block(above: 2em)
#outline(title: "大纲" ,indent:  n => {n * 2em},)

= Simple NuSMV

== 目标

我们要完成一个类似NuSMV的程序，不妨叫他ssmv(for simple smv)，但是它的文法可能会有所改变(为了更简单地写编译器)，并且在最后使用它完成死锁的证明。

形象地说，它的架构如下：

#figure(image("fig/start.svg"), caption: "程序架构图")

== ROBDD 

== ssmv的文法

整个程序的文法如下,注意前有\_的是非终止符号：
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
"_case_list" -> "id = num : (num | {_num_list});_case_list"\
"_define" -> "DEFINE _def_list" \
"_def_list" -> "id := id in {_num_list} _def_list" \
"_spec" -> "CTLSPEC ctl_term _spec" | epsilon \
$
]


其中ctl_term会放到下面的CTL文法里解析。

== CTL的文法

我们按照下面的产生式来定义CTL的文法，不难发现它是一个LL(1)文法，这将有利于我们解析它。

#par("")

#grid(columns: (1fr, 1fr))[$S &-> I \ I &-> O -> I \ I &-> O \
O &-> A | O \ O &-> A \ A &-> N \& A \ A &-> N \
N &-> ! N \ N &-> P \ P &-> "AG"(S) \ P &-> "AF"(S)
\ P &-> "AX"(S)$][$P &-> "EG"(S) \ P &-> "EF"(S) \ P &-> "EX"(S) \ P &-> A[ S "U" S ] \ P &-> E[ S "U" S] \
P &-> (S) \ P &-> T \ P &-> F \ P &-> id$]

== Rust

== 死锁证明

= Other Project
