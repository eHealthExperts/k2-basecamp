[![Latest](https://img.shields.io/github/release/eHealthExperts/k2-basecamp.svg?label=latest)](https://github.com/eHealthExperts/k2-basecamp/releases/latest) [![Build Status](https://travis-ci.org/eHealthExperts/k2-basecamp.svg?branch=master)](https://travis-ci.org/eHealthExperts/k2-basecamp) [![Build status](https://ci.appveyor.com/api/projects/status/mr7hc26i3nvddi04/branch/master?svg=true)](https://ci.appveyor.com/project/ChriFo/k2-basecamp/branch/master)

# K2 basecamp

> CTAPI adapter for the gematik Konnektor

*K2 basecamp* is an implementation of the [CTAPI](doc/CTAPI.pdf) standard as a dynamic system library.<br/>
Currently [builds](https://github.com/eHealthExperts/k2-basecamp/releases/latest) are available for Microsoft Windows and Linux.


## Requirements

* [**K2**](http://k2.ehealthexperts.de/) from eHealth Experts GmbH
* on Windows: [Visual C++ Redistributable for Visual Studio 2015](https://www.microsoft.com/en-US/download/details.aspx?id=48145)
  > Select installer according to target triplet of the build, e.g., **vc_redist.x86.exe** for **ctehxk2-x86.dll**


## Configuration

The library is configurable by the following environment variables.

| Variable     | Description                              |
| ------------ | ---------------------------------------- |
| K2_BASE_URL  | URL of the REST endpoint of *K2 peak*.<br/>**Default: http://localhost:8080/k2/ctapi** <br/> |
| K2_LOG_LEVEL | Set the verbosity level for logging.<br/>Possible values: Off, Error, Warn, Info, Debug, Trace<br/>**Default: Error** |
| K2_LOG_PATH  | Target folder of the log file ctehxk2.log.<br/>**Default: Logging to STDOUT** |
| K2_CTN       | Set CTN to use for all requests. *Requires that K2_PN is set!* |
| K2_PN        | Set PN to use for all requests. *Requires that K2_CTN is set!* |


## Building the library

1. (On Windows only) Install a *MSVC* , e.g., by installing the [Microsoft Visual C++ Build Tools](http://landinghub.visualstudio.com/visual-cpp-build-tools).

2. Install [Rust](https://www.rust-lang.org).

   > Select the desired [target triplet](https://forge.rust-lang.org/platform-support.html), e.g., **i686-pc-windows-msvc** for a 32-bit Windows build.

3. Run `cargo build —-release`  to create **target/release/ctehxk2.dll**. 


## License

MIT © [eHealth Experts GmbH](http://ehealthexperts.de)
