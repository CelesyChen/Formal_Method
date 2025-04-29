#import "@local/my_hw:0.0.1": *

#show: homework.with(title: "形式化方法——大作业")

= CTL的文法

$S &-> I \ I &-> O -> I \ I &-> O \
O &-> A | O \ O &-> A \ A &-> N \& A \ A &-> N \
N &-> ! N \ N &-> P \ P &-> "AG"(S) \ P &-> "AF"(S)
\ P &-> "AX"(S) \ P &-> "EG"(S) \ P &-> "EF"(S) \ P &-> "EX"(S) \ P &-> A[ S U S ] \ P &-> E[ S U S] \
P &-> (S) \ P &-> T \ P &-> F \ P &-> id$

= 