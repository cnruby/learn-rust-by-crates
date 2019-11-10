
## install MacOS
Install Rust
Install Visual Studio Code 
Install VS Code Extensions: Rust (rls)
Install VS Code Extensions: CodeLLDB

## use vscode on MacOS 
- select Cargo Project
- Menu >> Click Debug -> Add Configuration >> LLDB: Custom Launch >> This should create and open launch.json
- Next, you should verify breakpoints are enabled.
- F5


## install CodeLLDB, comm to result
```
Acquiring platform package for CodeLLDB.
Package is located at https://github.com/vadimcn/vscode-lldb/releases/download/v1.4.0/vscode-lldb-x86_64-darwin.vsix
Downloading...
Downloaded 0%
Downloaded 5%
Downloaded 10%
Downloaded 15%
Downloaded 20%
Downloaded 25%
Downloaded 30%
Downloaded 35%
Downloaded 40%
Downloaded 45%
Downloaded 50%
Downloaded 55%
Downloaded 60%
Downloaded 65%
Downloaded 70%
Downloaded 75%
Downloaded 80%
Downloaded 85%
Downloaded 90%
Downloaded 95%
Downloaded 100%
Installing...
Done.
```

## use gdbgui
https://github.com/cs01/gdbgui/tree/master/examples/rust
https://www.gdbgui.com/


## 参考资料
- [How to Debug Rust with Visual Studio Code](https://www.forrestthewoods.com/blog/how-to-debug-rust-with-visual-studio-code/)
- [rust-language-linux](https://lustysociety.org/programming/rust_language/rust-language-linux.html)
- [Step by step interactive debugger for Rust?](https://stackoverflow.com/questions/37586216/step-by-step-interactive-debugger-for-rust)