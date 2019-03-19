# K2 basecamp

[![Latest](https://img.shields.io/github/release/eHealthExperts/k2-basecamp.svg?label=latest)](https://github.com/eHealthExperts/k2-basecamp/releases/latest)
[![Build Status](https://dev.azure.com/ehex/K2-Basecamp/_apis/build/status/eHealthExperts.k2-basecamp)](https://dev.azure.com/ehex/K2-Basecamp/_build/latest?definitionId=1)
[![Coverage](https://codecov.io/gh/eHealthExperts/k2-basecamp/branch/master/graph/badge.svg)](https://codecov.io/gh/eHealthExperts/k2-basecamp)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

The dynamic system library *K2 basecamp* is an implementation of the [CT-API](doc/CT-API-1.1.1.pdf) standard.<br/>
[Builds](https://github.com/eHealthExperts/k2-basecamp/releases/latest) are available for Microsoft Windows, (Ubuntu) Linux and macOS.

## Requirements

* [**K2**](http://k2.ehealthexperts.de/) from eHealth Experts GmbH

## Configuration

| Key       | Value                                    |
| --------- | ---------------------------------------- |
| base_url  | URL of the REST endpoint of *K2 peak*.<br/>**Default: http://localhost:8088/k2/ctapi** |
| timeout   | Timeout in seconds for each http request. <br/>**Default: 0 (disabled)** |
| log_level | Set the verbosity level for logging. Possible values: Off, Error, Info, Debug<br/>**Default: Error** |
| log_path  | Target folder of the log file.<br/>**Default: Logging to STDOUT** |
| ctn       | Set card terminal number to use for all requests. *Requires that pn is set!* |
| pn        | Set port number to use for all requests. *Requires that ctn is set!* |

### Environment variable

In order to configure by using envirnoment variables, the above mentioned keys need the prefix **K2_** and has to be uppercase, e.g, **K2_BASE_URL**.

### Config file

Multiple file formats are supported: `INI` `JSON` `YAML`

Locate a file with the library name and the corresponding file ending, e.g., **ctehxk2.json** for Windows or **libctehxk2.json** for Linux in the working directory.

Use the above mentioned keys in the syntax of the desired file format, e.g., for JSON:

```json
{
	"log_level": "debug",
	"base_url": "http://localhost:5050"
}
```

:exclamation: Both - environment variables and a config file - can coexist where as the environment variables will have higher priority.
