# K2 basecamp

[![Latest](https://img.shields.io/github/release/eHealthExperts/k2-basecamp.svg?label=latest)](https://github.com/eHealthExperts/k2-basecamp/releases/latest)
[![Travis Build Status](https://travis-ci.org/eHealthExperts/k2-basecamp.svg?branch=master)](https://travis-ci.org/eHealthExperts/k2-basecamp)
[![Appveyor Build status](https://ci.appveyor.com/api/projects/status/mr7hc26i3nvddi04/branch/master?svg=true)](https://ci.appveyor.com/project/ChriFo/k2-basecamp)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

The dynamic system library *K2 basecamp* is an implementation of the [CT-API](doc/CT-API-1.1.1.pdf) standard.<br/>
[Builds](https://github.com/eHealthExperts/k2-basecamp/releases/latest) are available for Microsoft Windows, (Ubuntu) Linux and macOS.

## Requirements

* [**K2**](http://k2.ehealthexperts.de/) from eHealth Experts GmbH
* [Visual C++ Redistributable for Visual Studio 2015](https://www.microsoft.com/en-US/download/details.aspx?id=48145) (Windows only!)

## Configuration

| Key       | Value                                    |
| --------- | ---------------------------------------- |
| base_url  | URL of the REST endpoint of *K2 peak*.<br/>**Default: http://localhost:8080/k2/ctapi** |
| log_level | Set the verbosity level for logging. Possible values: Off, Error, Warn, Info, Debug, Trace<br/>**Default: Error** |
| log_path  | Target folder of the log file.<br/>**Default: Logging to STDOUT** |
| ctn       | Set card terminal number to use for all requests. *Requires that pn is set!* |
| pn        | Set port number to use for all requests. *Requires that ctn is set!* |

### Environment variable

In order to configure by using envirnoment variables, the above mentioned keys need the prefix **K2_** and has to be uppercase, e.g, **K2_BASE_URL**.

### Config file

Multiple file formats are supported: `JSON` `TOML` `YAMl`

Locate a file with the library name and the corresponding file ending, e.g., **ctehxk2.json** for Windows or **libctehxk2.json** for Linux next to the library.

Use the above mentioned keys in the syntax of the desired file format, e.g., for JSON:

```json
{
	"log_level": "debug",
	"base_url": "http://localhost:5050"
}
```

Please have also a look into this [example](examples/settings-file). 

:exclamation: Both - environment variables and a config file - can coexist where as the environment variables will have higher priority.
