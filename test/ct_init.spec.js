'use strict';

var _ = require('lodash');
var __ = require('hamjest');
var express = require('express');
var EXT = require('./library-ext.js');
var Library = require('fastcall').Library;
var path = require('path');

describe('CT_init func', () => {

    var CT_init;

    var library;
    var params;
    var server;

    var result;

    beforeEach(done => {
        var location = path.join(__dirname, '../target/release/libk2_basecamp') + EXT;
        library = new Library(location).asyncFunction({ CT_init: ['int8', ['uint16', 'uint16']]});

        CT_init = library.interface.CT_init;

        result = '0'; // VALID per default

        var app = express();
        app.post('/k2/ctapi/ct_init/:ctn/:pn', (request, response) => {
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

    it('should call equivalent REST path with ctn and pn parameter', done => {

        CT_init(1, 1).then(() => {
            __.assertThat(params, __.hasProperties({
                ctn: '1',
                pn: '1'
            }));

            done();
        });
    });

    it('should return REST response on valid call', done => {

        var expectedResponse = -1;
        result = expectedResponse.toString();

        CT_init(1, 1).then(response => {
            __.assertThat(response, __.is(expectedResponse));

            done();
        });
    });

    it('should allow a consecutive CT_init call when sever returned -1 for the first call', done => {

        var expectedResponse = -1;
        result = expectedResponse.toString();

        CT_init(1, 1).then(r1 => {
            __.assertThat(r1, __.is(expectedResponse));

            expectedResponse = 0;
            result = expectedResponse.toString();

            CT_init(1, 1).then(r2 => {
                __.assertThat(r2, __.is(expectedResponse));

                done();
            });
        });

    })

    it('should not allow consecutive CT_init calls', done => {

        CT_init(1, 1).then(r1 => {
            __.assertThat(r1, __.is(0));

            CT_init(1, 1).then(r2 => {
                __.assertThat(r2, __.is(-1));

                done();
            });
        });
    })
});
