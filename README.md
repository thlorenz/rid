# Rid [![](https://github.com/thlorenz/rid/workflows/Build+Test/badge.svg?branch=master)](https://github.com/thlorenz/rid/actions)

Rust integrated Dart framework providing an easy way to build Flutter apps with Rust.

## Welcome to Rid

This is the repository in which [_rid_](https://thlorenz.com/rid) development takes place. It contains all the
necessary crates to generate code from macro attributes.

`rid-build` to pull it all together and `rid-template-flutter` is here for now to get started with an app quickly until I build a `rid`
CLI tool which combines the two.

Note that `rid-build` makes platform specific assumptions about paths from which it attempts finding the `libclang` dynamic library required for the FFI generation between Rust and Dart FFI on your system. As part of generating the FFI bindings (`./sh/bindgen` when building the examples in the `rid-examples` repository), you can pass in the environment variable `LIBCLANG_PATH` to specify a custom path from where the Clang library is to be found (amongst the possibly many available on the system) to override the default search paths provided in `rid-build/src/host_props.rs`.

_rid_ documentation will always live on the [main docs
section](https://thlorenz.com/rid-site/docs/getting-started/introduction/).

## Using rid with an M1 Macbook

Building rid for an M1 Macbook requires additional steps:
* ⚠️  no need for Rosetta
* install the appropriate target
  ```sh
  rustup target add x86_64-apple-darwin
  ```
* install LLVM
  make sure LLVM version is `9+`
  ```sh
  brew install llvm
  ```
* install FFI
  ```sh
  arch -x86_64 sudo gem install ffi
  ```
* Dart SDK
  make sure that Dart version is `>=2.14`
  ```sh
  dart --version
  ```
  otherwise install it as specified in the [documentation](https://dart.dev/get-dart)
* configure bindgen script (at `PROJECT_ROOT/sh/bindgen`)
  ```sh
  # ...
  cargo run --target=x86_64-apple-darwin --bin=rid_build
  ```
* configure macos script (at `PROJECT_ROOT/sh/macos`)
  ⚠️  when compiling with `--target` compilation option,
  binary and files, instead of being typically placed in the `/target` folder,
  will be placed in a subfolder named as the specified target arch
  ```sh
  # ...
  # TARGET_DIR="PROJECT_ROOT/target/x86_64-apple-darwin" where PROJECT_ROOT is the root of your project
  # ...
  cargo build --target=x86_64-apple-darwin && \
    cp $LIB_SOURCE_FILE $LIB_TARGET_FILE
  ```
* run and enjoy
  ```sh
  ./sh/bindgen && ./sh/macos && flutter run -d macos
  ```

In the future, all these steps will be automated.

## Sponsors

Thank you very much for sponsoring me to help me keep working on _rid_ as well as open source
it fully as [explained here](https://thlorenz.com/rid-site/docs/contributing/sponsor/).

Feel free to ask any questions.

I just have one favor to ask from you. **Please do not fork this repository yet nor make it, or
contained code available publicly** as that would defeat the
[Sponsorware](https://github.com/sponsorware/docs) approach I chose to make _rid_ development
sustainable.

Additionally in order to keep the repository private yet allow sponsors to access it, I invite
them as collaborators. Please respect however that I expect you to **only read or use the
code**, but not push any updates to the repository. Any contribution should be [provided via a
pull request](https://thlorenz.com/rid-site/docs/contributing/how-to-contribute/).

A huge thanks for your support! 🙏 ❤️

# LICENSE

MIT
