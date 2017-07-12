'use strict';

var _ = require('lodash');
var __ = require('hamjest');
var express = require('express');
var LIBNAME = require('./library-name.js');
var Library = require('fastcall').Library;
var path = require('path');

describe('CTAPI status', () => {

    var CT_init;

    var library;
    var params;
    var server;

    var statusCode;

    beforeEach(() => {
        var location = path.join(__dirname, '../target/release/', LIBNAME);
        library = new Library(location).asyncFunction({ CT_init: ['int8', ['uint16', 'uint16']]});

        CT_init = library.interface.CT_init;

        var app = express();
        app.post('/k2/ctapi/ct_init/:ctn/:pn', (request, response) => {
            response.send(statusCode);
        });
        server = app.listen(8080);
    });

    afterEach(() => {
        library.release();
        library = null;
        server.close();
    });

    _.forEach([0, -1, -8, -10, -11, -127, -128], value => {
        it(`should return ${value} when the servers response contains ${value}`, done => {
            
            statusCode = value.toString();
            CT_init(1, 1).then(response => {
                __.assertThat(response, __.is(value));

                done();
            });
        });
    });

    it('should return -128 when the servers response is not a valid CTAPI status code', done => {

        statusCode = '1';
        CT_init(1, 1).then(response => {
            __.assertThat(response, __.is(-128));

            done();
        });
    });
});
