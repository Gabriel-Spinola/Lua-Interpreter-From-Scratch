print "Hi, Mom!"

-- # Building Interpetrer a Lua Interpreter in Rust
-- https://wubingzheng.github.io/build-lua-in-rust/en/

-- ## General Compilation Principle
--          Lexical Analysis      Syntax Analysis     Semantic Analysis
-- Character stream --> Token Stream --> Syntax Tree --> Intermediate Code

-- ## Our Implementation (Referencing the lua official implementation)
--             Lexical Analysis         Syntax Analysis
-- Character Stream --------> Token Stream --------> Bytecode
--                                                      ^
--                                                      |
--                                               virtual machine

-- The combination of lexical and syntax analysis will be called the "parsing" process
-- And the virtual machine is the "execution" process, then the bytecode is the link
-- connecting this two processes.

-- ## Bytecode
--       generate           execute
-- parse -------> bytecode <------- virtual machine