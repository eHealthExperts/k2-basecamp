'use strict';

var _ = require('lodash');
var __ = require('hamjest');
var express = require('express');
var bodyParser = require('body-parser');
var EXT = require('./library-ext.js');
var fastcall = require('fastcall');
var path = require('path');

var ArrayType = fastcall.ArrayType
var Library = fastcall.Library;
var U8Array = new ArrayType('uint8');

describe('CT_data func', () => {

    var CT_close;
    var CT_data;
    var CT_init;

    var library;
    var params;
    var server;

    var handler;

    beforeEach(done => {
        var location = path.join(__dirname, '../target/release/libk2_basecamp') + EXT;
        library = new Library(location)
            .asyncFunction({ CT_init: ['int8', ['uint16', 'uint16']]})
            .asyncFunction({ CT_data: ['int8', ['uint16', 'uint8', 'uint8', 'uint16', U8Array, 'uint16', U8Array]]})
            .asyncFunction({ CT_close: ['int8', ['uint16']]});

        CT_close = library.interface.CT_close;
        CT_data = library.interface.CT_data;
        CT_init = library.interface.CT_init;

        var app = express();
        var jsonParser = bodyParser.json();

        app.post('/k2/ctapi/ct_init/:ctn/:pn', (request, response) => {
            response.send('0');
        });

        app.post('/k2/ctapi/ct_data/:ctn/:pn', jsonParser, function (request, response) {
            handler(request, response);
        });

        app.post('/k2/ctapi/ct_close/:ctn/:pn', (request, response) => {
            response.send('0');
        });

        server = app.listen(8080, done);
    });

    afterEach(done => {
        handler = null;

        server.close(() => {
            library.release();
            library = null;

            done();
        });
    });

    it('should not call REST path but return with -1', done => {

        var called = false;
        handler = (request, response) => {
            called = true;
        };

        CT_data(1, 1, 1, 1, new U8Array([]), 0, new U8Array([])).then(result => {
            __.assertThat(result, __.is(-1));
            __.assertThat(called, __.is(__.falsy()));

            done();
        });
    });

    it('should call equivalent REST path with ctn parameter when CT_init was called before', done => {

        var r = null;
        var responseCode = 0;
        handler = (request, response) => {
            r = request;
            response.send(responseCode.toString());
        }

        CT_init(1, 1).then(result => {
            __.assertThat(result, __.is(0));

            CT_data(1, 1, 1, 1, new U8Array([]), 0, new U8Array([])).then(result => {
                __.assertThat(result, __.is(responseCode));
                __.assertThat(r.params, __.hasProperties({
                    ctn: '1',
                    pn: '1'
                }));

                done();
            });
        });
    });

    it('should map the parameter data to the http body as JSON', done => {

        var r;
        handler = (request, response) => {
            r = request;
            response.send('0');
        }

        var dad = 3;
        var sad = 2;

        var commands = [1, 2, 3, 4, 5];
        var command = new U8Array(commands);
        var lenc = command.length;

        var lenr = 10;
        var response = new U8Array(lenr);

        CT_init(1, 1).then(() => {
            CT_data(1, dad, sad, lenc, command, lenr, response).then(() => {

                __.assertThat(r.body, __.equalTo({
                    ctn: 1,
                    dad, sad, lenc,
                    command: commands,
                    lenr
                }))

                done();
            });
        });
    });
});
