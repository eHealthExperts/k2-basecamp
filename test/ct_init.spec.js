'use strict';

var _ = require('lodash');
var __ = require('hamjest');
var express = require('express');
var LIBNAME = require('./library-name.js');
var Library = require('fastcall').Library;
var path = require('path');

describe('CT_init func', () => {

    var CT_init;
    var library;

    beforeEach(() => {
        var location = path.join(__dirname, '../target/debug/', LIBNAME);
        library = new Library(location).asyncFunction({ CT_init: ['int8', ['uint16', 'uint16']]});

        CT_init = library.interface.CT_init;
    });

    afterEach(() => {
        library.release();
        library = null;
    });

    it('without running backend server should return -128', done => {

        CT_init(1, 1).then(response => {

            __.assertThat(response, __.is(-128));
            done();
        });
    });

    describe('with running backend server', () => {
        var requestParameter;
        var server;
        var serverResponseBody;

        beforeEach(() => {
            serverResponseBody = '0'; // init

            var app = express();
            app.post('/k2/ctapi/ct_init/:ctn/:pn', (request, response) => {
                requestParameter = _.assign({}, {
                    ctn: encodeURIComponent(request.params.ctn),
                    pn: encodeURIComponent(request.params.pn)
                });

                response.send(serverResponseBody);
            });
            server = app.listen(8080);
        });

        afterEach(() => {
            server.close();
        });

        it('should call equivalent REST path with ctn and pn parameter', done => {

            CT_init(1, 1).then(() => {

                __.assertThat(requestParameter, __.hasProperties({
                    ctn: '1',
                    pn: '1'
                }));
                done();
            });
        });

        it('should return REST response on valid call', done => {

            serverResponseBody = "-1";
            CT_init(1, 1).then(response => {

                __.assertThat(response, __.is(-1));
                done();
            });
        });

        it('should allow a consecutive CT_init call when sever returned -1 for the first call', done => {

            serverResponseBody = "-1";
            CT_init(1, 1).then(r1 => {

                __.assertThat(r1, __.is(-1));

                serverResponseBody = "0";
                CT_init(1, 1).then(r2 => {

                    __.assertThat(r2, __.is(0));
                    done();
                });
            });
        });

        it('should not allow consecutive CT_init calls', done => {

            CT_init(1, 1).then(r1 => {
                __.assertThat(r1, __.is(0));

                CT_init(1, 1).then(r2 => {
                    __.assertThat(r2, __.is(-1));

                    done();
                });
            });
        });
    });
});
