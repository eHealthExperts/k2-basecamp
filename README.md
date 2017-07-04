[![Build Status](https://travis-ci.org/eHealthExperts/k2-basecamp.svg?branch=master)](https://travis-ci.org/eHealthExperts/k2-basecamp) [![Build status](https://ci.appveyor.com/api/projects/status/wki43vn7gouqrh9s/branch/master?svg=true)](https://ci.appveyor.com/project/ChriFo/k2-basecamp/branch/master)

# K2 Basecamp

Environment variables for application configuration settings:

The following table lists environment variables and their effect to basecamp.

| Key          | Description                              |
| ------------ | ---------------------------------------- |
| K2_CTN       | Set CTN to use for all requests. *Requires that K2_PN is set!* |
| K2_PN        | Set PN to use for all requests. *Requires that K2_CTN is set!* |
| K2_BASE_URL  | **Default: http://localhost:8080/k2/ctapi** <br/>The REST endpoint of K2 peak. |
| K2_LOG_LEVEL | **Default: Error** Set the verbosity level for logging. <br/>Possible values: Off, Error, Warn, Info, Debug, Trace |
| K2_LOG_PATH  | **Default: Logging to stout** Target folder of the debug log file: **ctehxk2.log** |
