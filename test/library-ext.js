'use strict';

module.exports = {
    'linux':  '.so'
    , 'linux2': '.so'
    , 'sunos':  '.so'
    , 'solaris':'.so'
    , 'freebsd':'.so'
    , 'openbsd':'.so'
    , 'darwin': '.dylib'
    , 'mac':    '.dylib'
    , 'win32':  '.dll'
}[process.platform]
