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

    beforeEach(done => {
        var location = path.join(__dirname, '../target/release/libk2_basecamp') + EXT;
        library = new Library(location).asyncFunction({ CT_init: ['int8', ['uint16', 'uint16']]});

        CT_init = library.interface.CT_init;

        var app = express();
        app.post("/k2/ctapi/ct_init/:ctn/:pn", (request, response) => {
            params = _.assign({}, {
                ctn: encodeURIComponent(request.params.ctn),
                pn: encodeURIComponent(request.params.pn)
            });

            response.send('0');
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

        CT_init(1, 1).then(result => {
            __.assertThat(result, __.is(0));
            __.assertThat(params, __.hasProperties({
                ctn: '1',
                pn: '1'
            }));

            done();
        });
    });

    it('should not allow consecutive CT_init calls', done => {

        CT_init(1, 1).then(result => {
            __.assertThat(result, __.is(0));

            CT_init(1, 1).then(result => {
                __.assertThat(result, __.is(-1));

                done();
            });
        });
    })
});
