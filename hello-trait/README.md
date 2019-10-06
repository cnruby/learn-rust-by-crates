## Project
- name: hello-trait
- crate name: trait_exerci
- description: how to understand the rust feature trait

## Subproject: lib-hello
- folder name: lib-hello
- description: the crate 'trait_exerci'

## Subproject: bin-local-hello
- folder name: bin-local-hello
- description: examples apps for the crate 'trait_exerci' in local develop

## Subproject: bin-hello
- folder name: bin-hello
- description: examples apps for the crate 'trait_exerci' with crates.io

### Create a project
```bash
mkdir ./docs/hello-trait
mkdir hello-trait && cd hello-trait
touch README.md
```

### Resources
- https://doc.rust-lang.org/book/ch10-02-traits.html
- https://doc.rust-lang.org/book/ch17-02-trait-objects.html
- http://idubrov.name/rust/2018/06/16/dynamic-casting-traits.html
- https://blog.mgattozzi.dev/avoiding-logic-errors/
- https://facility9.com/2016/04/talking-about-rusts-traits/
- https://guiand.xyz/blog-posts/unboxed-trait-objects.html
- 