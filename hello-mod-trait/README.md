## Project
- name: hello-mod-trait
- crate name: mod_trait_exerci
- description: how to understand the rust feature trait with mod

## Subproject: lib-hello
- folder name: lib-hello
- description: the crate 'mod_trait_exerci'

## Subproject: bin-local-hello
- folder name: bin-local-hello
- description: with local crate version to use examples apps for the crate 'mod_trait_exerci'

## Subproject: bin-hello
- folder name: bin-hello
- description: with the crates.io version to use examples apps for the crate 'mod_trait_exerci'

### Create a project
```bash
mkdir ./docs/hello-mod-trait
mkdir hello-mod-trait && cd hello-mod-trait
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