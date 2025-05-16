#!/bin/bash

picture=(start )

for i in "${picture[@]}"; do
    t="dot $i.dot -Tsvg > $i.svg"
    echo "$t"
    eval "$t"
done