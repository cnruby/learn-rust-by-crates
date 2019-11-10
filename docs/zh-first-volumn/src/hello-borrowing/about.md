## 关于软件篋borrowing_exerci

## 安装运行Rust语言脚本工具`cargo-script`

```bash
cargo install cargo-script
```

## 安装本软件箧`borrowing_exerci`

```bash
cargo install borrowing_exerci
```
## 使用方法

### 帮助命令
```bash
bw -h
```

### 运行错误借用代码实例命令

　　比如运行Rust程序`kw_fn.rs`命令如下：

```bash
bw -f kw_fn -m err | bat -l rs
# tip: `f`, Forward  one window
# tip: `b`, Backward  one window
# tip: `q`, Exit.
```

### 运行借用代码实例命令
　　比如运行Rust程序`kw_fn.rs`命令如下：

```bash
bw -f kw_fn -m ok | bat -l rs
```

## 参考资料
- [cargo-test-internal-packages](https://users.rust-lang.org/t/cargo-test-internal-packages/5187/2)