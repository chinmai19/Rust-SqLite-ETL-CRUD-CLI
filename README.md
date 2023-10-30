# Rewrite a Python Script in Rust

[![Build binary release](https://github.com/nogibjj/rmr62_rust_week_8/actions/workflows/release.yml/badge.svg)](https://github.com/nogibjj/rmr62_rust_week_8/actions/workflows/release.yml) [![Clippy](https://github.com/nogibjj/rmr62_rust_week_8/actions/workflows/lint.yml/badge.svg)](https://github.com/nogibjj/rmr62_rust_week_8/actions/workflows/lint.yml) [![Rustfmt](https://github.com/nogibjj/rmr62_rust_week_8/actions/workflows/rustfmt.yml/badge.svg)](https://github.com/nogibjj/rmr62_rust_week_8/actions/workflows/rustfmt.yml) [![Tests](https://github.com/nogibjj/rmr62_rust_week_8/actions/workflows/tests.yml/badge.svg)](https://github.com/nogibjj/rmr62_rust_week_8/actions/workflows/tests.yml)

## Performance comparison report
![A good starting point for a new Rust project](https://user-images.githubusercontent.com/36940292/277520394-6b82d493-3c4e-4fd5-bfee-d75cccda81a3.png)

The screenshot above shows that Rust outperforms Python in terms of both speed and resource usage. When calculating the 35th Fibonacci sequence, Python took significantly longer to execute and used much more CPU compared to Rust. Although the memory usage was comparable, Rust generally has better memory performance. However, in the case of the recursive Fibonacci function, the memory usage in Rust and Python is similar because both implementations use a recursive algorithm that creates a large number of stack frames, which can consume a significant amount of memory.

## Rust Vs Python (Speed and Resource)

Rust is faster and more resource-efficient than Python because Rust is a compiled language, while Python is an interpreted language.

In a compiled language like Rust, the source code is translated into machine code by a compiler, which can optimize the code for performance and memory usage. The resulting executable code can be directly executed by the computer's processor, without the need for an interpreter.

In contrast, an interpreted language like Python requires an interpreter to execute the code. The interpreter reads the source code line by line and executes it on the fly, which can result in slower performance and higher memory usage compared to a compiled language.

Additionally, Rust has a number of features that make it more memory-efficient than Python. Rust uses a strict ownership and borrowing model, which ensures that memory is managed efficiently and prevents common memory-related bugs like null pointer dereferences and memory leaks. Rust also has a built-in package manager, Cargo, which makes it easy to manage dependencies and ensure that only the necessary code is included in the final executable.

Overall, Rust's compiled nature, performance optimizations, and memory management features make it faster and more resource-efficient than Python for many types of applications.
