'use strict';

/**
 * Because the name of the library artifact differs with os
 */
module.exports = {
    'linux':  'libctehxk2.so'
    , 'linux2': 'libctehxk2.so'
    , 'sunos':  'libctehxk2.so'
    , 'solaris':'libctehxk2.so'
    , 'freebsd':'libctehxk2.so'
    , 'openbsd':'libctehxk2.so'
    , 'darwin': 'libctehxk2.dylib'
    , 'mac':    'libctehxk2.dylib'
    , 'win32':  'ctehxk2.dll'
}[process.platform]
