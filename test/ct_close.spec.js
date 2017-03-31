'use strict';

var _ = require('lodash');
var __ = require('hamjest');
var express = require('express');
var EXT = require('./library-ext.js');
var Library = require('fastcall').Library;
var path = require('path');

describe('CT_close func', () => {

    var CT_close;
    var CT_init;

    var library;
    var params;
    var server;

    var result;

    beforeEach(done => {
        var location = path.join(__dirname, '../target/debug/libk2_basecamp') + EXT;
        library = new Library(location)
            .asyncFunction({ CT_init: ['int8', ['uint16', 'uint16']]})
            .asyncFunction({ CT_close: ['int8', ['uint16']]});

        CT_close = library.interface.CT_close;
        CT_init = library.interface.CT_init;

        var app = express();

        app.post("/k2/ctapi/ct_init/:ctn/:pn", (request, response) => {
            response.send('0');
        });

        result = "0"; // valid per default

        app.post("/k2/ctapi/ct_close/:ctn/:pn", (request, response) => {
            params = _.assign({}, {
                ctn: encodeURIComponent(request.params.ctn),
                pn: encodeURIComponent(request.params.pn)
            });

            response.send(result);
        });

        server = app.listen(8080, done);
    });

    afterEach(done => {
        server.close(() => {
            library.release();
            library = null;

            done();
        });
    });

    it('should not call REST path but return with -1', done => {

        CT_close(1).then(result => {
            __.assertThat(result, __.is(-1));
            __.assertThat(params, __.is(__.not(__.defined())));

            done();
        });
    });

    it('should return server response on valid call', done => {

        var expectedResponse = -1;
        result = expectedResponse.toString();

        CT_init(1, 1).then(r1 => {
            __.assertThat(r1, __.is(0));

            CT_close(1).then(r2 => {
                __.assertThat(r2, __.is(expectedResponse));

                done();
            });
        });
    });

    it('should call equivalent REST path with ctn parameter when CT_init was called before', done => {

        CT_init(1, 1).then(r1 => {
            __.assertThat(r1, __.is(0));

            CT_close(1).then(r2 => {
                __.assertThat(r2, __.is(0));
                __.assertThat(params, __.hasProperties({
                    ctn: '1',
                    pn: '1'
                }));

                done();
            });
        });
    });

    it('should return -1 on consecutive close', done => {

        CT_init(1, 1).then(r1 => {
            __.assertThat(r1, __.is(0));

            CT_close(1, 1).then(r2 => {
                __.assertThat(r2, __.is(0));

                CT_close(1, 1).then(r3 => {
                    __.assertThat(r3, __.is(-1));

                    done();
                });
            });
        });
    });
});
