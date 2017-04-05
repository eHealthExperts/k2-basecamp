#!/usr/bin/env groovy

pipeline {
    agent {
        dockerfile {
            args "-v /etc/ssl/certs:/etc/ssl/certs:ro"
        }
    }

    options {
        buildDiscarder(logRotator(numToKeepStr:'5'))
        disableConcurrentBuilds()
        skipDefaultCheckout()
    }

    triggers {
        pollSCM('')
    }

    stages {
        stage("Checkout source") {
            steps {
                deleteDir()
                checkout scm
            }
        }

        stage('Fetch dependencies') {
            steps {
                configFileProvider([configFile(
                    fileId: 'be5bdcbb-d40a-44ea-864a-dcc5d543319d',
                    targetLocation: '.npmrc')
                ]) {
                    sh 'npm i'
                    sh 'cargo update'
                }
            }
        }

        stage('Check linting') {
            steps {
                sh 'npm run lint'
            }
        }

        stage('Run integration tests') {
            steps {
                sh 'npm run test'
            }
        }

        stage('Publish artifact') {
            steps {
                configFileProvider([configFile(
                    fileId: 'be5bdcbb-d40a-44ea-864a-dcc5d543319d',
                    targetLocation: '.npmrc')
                ]) {
                    script {
                        def currentBranch = sh(script: 'git name-rev --name-only HEAD', returnStdout: true).trim()
                        echo "Branch: ${currentBranch}"

                        def publish = currentBranch.endsWith('master')
                        def latestTag = sh(script: 'git tag --sort version:refname | tail -1', returnStdout: true).trim()
                        echo "Latest tag: ${latestTag}"

                        def name = getName(readFile('package.json'))
                        def latestVersion = sh(script: "npm show ${name} version 2>/dev/null || echo 0.0.0", returnStdout: true).trim()
                        echo "Latest version: ${latestVersion}"

                        if (publish && isNewVersion(latestTag, latestVersion)) {
                            sh 'npm publish'
                        }
                    }
                }
            }
        }
    }

    post {
        failure {
            mattermostSend color: '#FF0000', message: "Pipeline **${env.JOB_NAME}** broken.\nDetails: ${env.BUILD_URL}"
        }
    }
}

@NonCPS
def isNewVersion(latestTag, latestVersionInRepo) {
  def tagVersion = latestTag.substring(1).tokenize('.')*.toInteger()
  def repoVersion = latestVersionInRepo.tokenize('.')*.toInteger()

  def tvSize = tagVersion.size()
  def rvSize = repoVersion.size()
  def minSize = (tvSize <= rvSize) ? tvSize : rvSize

  for (i = 0; i < minSize; i++) {
    if (tagVersion[i] > repoVersion[i]) {
      return true
    }
  }

  return false
}

@NonCPS
def getName(packageJson) {
  def json = new groovy.json.JsonSlurper().parseText(packageJson)
  return json.name
}
