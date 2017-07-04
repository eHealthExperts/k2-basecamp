'use strict';

var gulp = require('gulp');
var replace = require('gulp-replace');

var packageJson = require('./package.json');

gulp.task('update-version', function(){
  gulp.src(['Cargo.toml'])
    .pipe(replace(/version = ".*"/, `version = "${packageJson.version}"`))
    .pipe(replace(/FileVersion = ".*"/, `FileVersion = "${packageJson.version}"`))
    .pipe(replace(/ProductVersion = ".*"/, `ProductVersion = "${packageJson.version}"`))
    .pipe(gulp.dest('.'));

   gulp.src(['.appveyor.yml'])
    .pipe(replace(/version: .*-{branch}-{build}/, `version: ${packageJson.version}-{branch}-{build}`))
    .pipe(gulp.dest('.'));
});
