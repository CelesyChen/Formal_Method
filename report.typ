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

== ssmv的文法

我曾尝试完整的表达出来这个文法，但是规范地表达太过麻烦，我们不妨直接看pest文件，并且在上面解释：

```text

```

== CTL的文法

我们按照下面的产生式来定义CTL的文法，不难发现它是一个LL(1)文法，这将有利于我们解析它。

#par("")

#grid(columns: (1fr, 1fr))[$S &-> I \ I &-> O -> I \ I &-> O \
O &-> A | O \ O &-> A \ A &-> N \& A \ A &-> N \
N &-> ! N \ N &-> P \ P &-> "AG"(S) \ P &-> "AF"(S)
\ P &-> "AX"(S)$][$P &-> "EG"(S) \ P &-> "EF"(S) \ P &-> "EX"(S) \ P &-> A[ S "U" S ] \ P &-> E[ S "U" S] \
P &-> (S) \ P &-> T \ P &-> F \ P &-> id$]


== Rust & pest

我们将会使用Rust来编写这个程序，尽管已经存在flex和bison这类能高效完成(并且也在编译原理课程上使用过的)lexer/parser工作的工具，这是因为如下考量：

+ Cargo，因为我不怎么会写Cmake，并且可以解决麻烦的依赖问题。
+ 可以编译成WASM： 如果有机会我想把这个工具包装到网站上。
+ 最后，也是最重要的，因为我还没试过使用Rust做过比较大的项目，所以想试试看。

我们将使用#link("https://docs.rs/pest/latest/pest/index.html")[pest. The Elegant Parser]<pest>作为Parser编写时的工具，避免大量且繁琐的重复工作（尽管我已经花了近8个小时在手动编写Parser上，后来太麻烦了所以废弃了，好在并非全部无用功，ctl的解析部分保留了下来）

具体的pest语法可以见#link(<pest>)[上面]，总得来说我们可以写出下面的语法文件：

== ROBDD 


= Simple SMV死锁证明
