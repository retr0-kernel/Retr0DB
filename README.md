# KrishDB

Taking inspiration from this (https://cstack.github.io/db_tutorial/parts/part1.html), where someone built a Databse from scratch in C using SQLite as reference. This inspired me to replicate something similar in RUST.

I would follow the architecture of SQLite, giving me the freedom to diverge in some aspects.

According to this blog, and the official SQLite documentation, this is roughly what we need to replicate:

* A REPL
* A tokenizer
* A parser
* A code generator
* A virtual machine that interprets the generated code
* A B+ Tree
* Pager
* OS Interface

Here's an overview of architecture,

[KrishDB](/images/KrishDB.png)