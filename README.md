[![Latest](https://img.shields.io/github/release/eHealthExperts/k2-basecamp.svg?label=latest)](https://github.com/eHealthExperts/k2-basecamp/releases/latest) [![Build Status](https://travis-ci.org/eHealthExperts/k2-basecamp.svg?branch=master)](https://travis-ci.org/eHealthExperts/k2-basecamp) [![Build status](https://ci.appveyor.com/api/projects/status/mr7hc26i3nvddi04/branch/master?svg=true)](https://ci.appveyor.com/project/ChriFo/k2-basecamp/branch/master)


# K2 basecamp

> CT-API adapter for the gematik Konnektor

*K2 basecamp* is an implementation of the [CT-API](doc/CTAPI.pdf) standard as a dynamic system library.<br/>
Currently [builds](https://github.com/eHealthExperts/k2-basecamp/releases/latest) are available for Microsoft Windows and (Ubuntu) Linux, both as a 32-bit version.


## Requirements

* [**K2**](http://k2.ehealthexperts.de/) from eHealth Experts GmbH

* In case you are using the 32-/64-bit MSVC DLL, you need to install the according 32-/64-bit version of [MS Visual C++ Redistributable](https://www.microsoft.com/de-de/download/details.aspx?id=48145) too.


## Configuration

The library is configurable by the following environment variables.

| Variable     | Description                              |
| ------------ | ---------------------------------------- |
| K2_BASE_URL  | URL of the REST endpoint of *K2 peak*.<br/>**Default: http://localhost:8080/k2/ctapi** <br/> |
| K2_LOG_LEVEL | Set the verbosity level for logging.<br/>Possible values: Off, Error, Warn, Info, Debug, Trace<br/>**Default: Error** |
| K2_LOG_PATH  | Target folder of the log file ctehxk2.log.<br/>**Default: Logging to STDOUT** |
| K2_CTN       | Set card terminal number (CTN) to use for all requests. *Requires that K2_PN is set!* |
| K2_PN        | Set port number (PN) to use for all requests. *Requires that K2_CTN is set!* |


## Build (on Windows)

1. Install a *MSVC*, e.g., by installing the [Microsoft Visual C++ Build Tools 2015](http://landinghub.visualstudio.com/visual-cpp-build-tools).

2. Install [OpenSSL](http://slproweb.com/products/Win32OpenSSL.html).

   > Ensure that the following environment variables are set: 
   >
   > DEP_OPENSSL_INCLUDE, OPENSSL_INCLUDE_DIR, OPENSSL_LIB_DIR, OPENSSL_LIBS

3. Install [Rust](https://www.rust-lang.org).

   > Select the desired target triplet. For example, use **i686-pc-windows-msvc** for 32-bit MSVC (Windows 7+) or **x86_64-unknown-linux-gnu** for 64-bit Linux (2.6.18+) (see [Rust Platform Support](https://forge.rust-lang.org/platform-support.html)).

4. Run `cargo build —-release`  to create **ctehxk2.dll** in the folder **target/release**. 


## License

MIT © [eHealth Experts GmbH](http://ehealthexperts.de)
