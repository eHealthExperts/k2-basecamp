# K2 Basecamp

> **K2-Adapter for CTAPI** - Connect CTAPI with K2.

[![Build Status](https://ci.ehex.de/buildStatus/icon?job=K2%20Mountain%20Peak/k2-basecamp/master)](https://ci.ehex.de/job/K2%20Mountain%20Peak/k2-basecamp/master) 



#### _"Build a Microsoft Windows compatible library (DLL) from source code written in Rust and tested with NodeJS."_



## Requirements

- [Rust](https://www.rust-lang.org)

- [Cargo](https://crates.io/)

- [rustfmt](https://github.com/rust-lang-nursery/rustfmt)

- [NodeJS](https://nodejs.org)


## Build

| Command           | Description                              |
| ----------------- | ---------------------------------------- |
| `npm run build`   | Creates a debug build of the library into the folder **target/debug** |
| `npm run release` | A release build wil be located in the folder **target/release**. |
| `npm run test`    | Creates a debug build and runs the Javascript tests angainst the library. |



### Howto create a *Windows DLL*

1. Install [Microsoft Visual C++ Build Tools 2015](https://blogs.msdn.microsoft.com/vcblog/2016/03/31/announcing-the-official-release-of-the-visual-c-build-tools-2015/)

2. Install [OpenSSL](http://slproweb.com/products/Win32OpenSSL.html).

   > Ensure that the following environment variables are set: 
   >
   > DEP_OPENSSL_INCLUDE, OPENSSL_INCLUDE_DIR, OPENSSL_LIB_DIR, OPENSSL_LIBS

3. Install [Rust](https://www.rust-lang.org).

   > Select the desired target triplet, e.g., **i686-pc-windows-msvc** for 32-bit Windows.

4. Run `cargo build —-release`  to create a file **ctehxk2.dll** in the folder **target/release**. 
