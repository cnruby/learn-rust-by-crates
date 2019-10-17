# 题外话

## Cargo工具第三方插件

```bash
# --------------------------
# install
cargo install cargo-update
# --------------------------
# install
cargo install cargo-audit
# or upgrade with the crate `cargo`
cargo install --force cargo-audit
# or upgrade with the crate `cargo-update`
cargo install-update cargo-audit
# using
cargo audit
# --------------------------
# install
cargo install cargo-bloat
# using
# only 'bin' and 'cdylib' crate types are supported.
cargo bloat --release -n 10
cargo bloat --release --crates
# --------------------------
# install
cargo install cargo-edit
# using
cargo add <CRATE>
cargo rm <CRATE>
cargo upgrade
# --------------------------
# install
cargo install cargo-asm
cargo asm <CRATE_NAME>::<MOD_NAME>::<FUNCTION_NAME>
cargo llvm-ir <CRATE_NAME>::<MOD_NAME>::<FUNCTION_NAME>
```

## 参考资料
- [Keeping-secure-with-cargo-audit-0.9](https://blog.rust-lang.org/inside-rust/2019/10/03/Keeping-secure-with-cargo-audit-0.9.html)
- [cargo-audit](https://crates.io/crates/cargo-audit)
- [cargo-update](https://crates.io/crates/cargo-update)
- [cargo-watch](https://crates.io/crates/cargo-edit)
- [cargo-asm](https://crates.io/crates/cargo-asm)
- [how-to-get-assembly-output-from-building-with-cargo](https://stackoverflow.com/questions/39219961/how-to-get-assembly-output-from-building-with-cargo)