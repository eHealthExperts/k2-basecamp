'use strict';

var _ = require('lodash');
var __ = require('hamjest');
var express = require('express');
var bodyParser = require('body-parser');
var LIBNAME = require('./library-name.js');
var fastcall = require('fastcall');
var path = require('path');

var ArrayType = fastcall.ArrayType
var Library = fastcall.Library;
var ref = fastcall.ref;
var UInt8Array = new ArrayType('uint8');

describe('CT_data func', () => {

    var CT_close;
    var CT_data;
    var CT_init;

    var library;
    var params;
    var server;

    var handler;

    beforeEach(() => {
        var location = path.join(__dirname, '../target/debug/') + LIBNAME;
        library = new Library(location)
            .asyncFunction({ CT_init: ['int8', ['uint16', 'uint16']]})
            .asyncFunction({ CT_data: ['int8', ['uint16', 'pointer', 'pointer', 'uint16', UInt8Array, 'pointer', UInt8Array]]})
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

        server = app.listen(8080);
    });

    afterEach(() => {
        handler = null;
        library.release();
        library = null;
        server.close();
    });

    it('should not call REST path but return with -1', done => {

        var called = false;
        handler = (request, response) => {
            called = true;
        };

        var dad = convertToUInt8Pointer(3);
        var sad = convertToUInt8Pointer(2);
        var lenr = convertToUInt16Pointer(0);

        var commands = [1];
        var command = new UInt8Array(commands);
        var lenc = command.length;

        CT_data(1, dad, sad, lenc, command, lenr, new UInt8Array(0)).then(result => {
            __.assertThat(result, __.is(-1));
            __.assertThat(called, __.is(__.falsy()));

            done();
        });
    });

    it('should return -127 when internal server error', done => {

        handler = (request, response) => {
            response.sendStatus(500);
        }

        var dad = convertToUInt8Pointer(3);
        var sad = convertToUInt8Pointer(2);
        var lenr = convertToUInt16Pointer(0);

        var commands = [1];
        var command = new UInt8Array(commands);
        var lenc = command.length;

        CT_init(1, 1).then(result => {
            CT_data(1, dad, sad, lenc, command, lenr, new UInt8Array(0)).then(result => {
                __.assertThat(result, __.is(-128));

                done();
            });
        });
    })

    it('should call equivalent REST path with ctn parameter when CT_init was called before', done => {

        var r = null;
        handler = (request, response) => {
            r = request;
            response.sendStatus(500);
        }

        var dad = convertToUInt8Pointer(3);
        var sad = convertToUInt8Pointer(2);
        var lenr = convertToUInt16Pointer(0);

        var commands = [1];
        var command = new UInt8Array(commands);
        var lenc = command.length;

        CT_init(1, 1).then(result => {
            __.assertThat(result, __.is(0));

            CT_data(1, dad, sad, lenc, command, lenr, new UInt8Array(0)).then(result => {
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
            response.sendStatus(500);
        }

        var dad = convertToUInt8Pointer(3);
        var sad = convertToUInt8Pointer(2);

        var commands = [1, 2, 3, 4, 5];
        var command = new UInt8Array(commands);
        var lenc = command.length;

        var responseSize = 10;
        var lenr = convertToUInt16Pointer(responseSize);
        var response = new UInt8Array(responseSize);

        CT_init(1, 1).then(() => {
            CT_data(1, dad, sad, lenc, command, lenr, response).then(() => {
                __.assertThat(r.body, __.equalTo({
                    dad: 3,
                    sad: 2,
                    lenc,
                    command: "AQIDBAU=",
                    lenr: 10
                }))

                done();
            });
        });
    });

    it('should return server response', done => {

        handler = (request, response) => {
            response.send(JSON.stringify({
                dad: request.body.sad,
                sad: request.body.dad,
                lenr: 1,
                response: 'kAA=',
                responseCode: 0
            }));
        }

        var dad = convertToUInt8Pointer(3);
        var sad = convertToUInt8Pointer(2);

        var commands = [1, 2, 3, 4, 5];
        var command = new UInt8Array(commands);
        var lenc = command.length;

        var responseSize = 1000;
        var lenr = convertToUInt16Pointer(responseSize);
        var response = new UInt8Array(responseSize);

        CT_init(1, 1).then(() => {
            CT_data(1, dad, sad, lenc, command, lenr, response).then(result => {
                __.assertThat(result, __.is(0));

                __.assertThat(dad.readUInt8(0), __.is(2));
                __.assertThat(sad.readUInt8(0), __.is(3));
                __.assertThat(lenr.readUInt16LE(0), __.is(1));
                __.assertThat(response.get(0), __.is(144));
                __.assertThat(response.get(1), __.is(0));

                done();
            });
        });
    });
});

function convertToUInt8Pointer(int) {
    var buf = Buffer.alloc(ref.sizeof.uint8);
    buf.writeUInt8(int, 0);
    return buf;
}

function convertToUInt16Pointer(int) {
    var buf = Buffer.alloc(ref.sizeof.uint16);
    buf.writeUInt16LE(int, 0);
    return buf;
}

function convertToUInt32Pointer(int) {
    var buf = Buffer.alloc(ref.sizeof.uint32);
    buf.writeUInt32LE(int, 0);
    return buf;
}
