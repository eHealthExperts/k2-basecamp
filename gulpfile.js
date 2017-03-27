'use strict';

var gulp = require('gulp');
var replace = require('gulp-replace');

var packageJson = require('./package.json');

gulp.task('update-version', function(){
  gulp.src(['Cargo.toml'])
    .pipe(replace(/version = ".*"/g, `version = "${packageJson.version}"`))
    .pipe(gulp.dest('.'));
});
