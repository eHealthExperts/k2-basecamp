#!/usr/bin/env groovy

properties([
  buildDiscarder(logRotator(artifactNumToKeepStr: '0', numToKeepStr: '5')),
  disableConcurrentBuilds(),
  pipelineTriggers([[$class: 'BitBucketTrigger'], pollSCM('')])
])

node {

  try {
    stage('Checkout and prepare source') {
      // cleanup
      deleteDir()

      checkout scm

      // set nodejs version
      nodeVersion = readFile('.node-version')
      sh "sed -i 's/%VERSION%/${nodeVersion}/' Dockerfile"

      kibanaVersions = getKibanaVersions(readFile('package.json'))

      stash name: 'source'
      stash excludes: '**/__tests__/**', name: 'source-without-tests'
    }

    // create a kibana image for each supported version
    def kibanaStages = [:]
    for (version in kibanaVersions) {
      kibanaStages[version] = {
        ws {
          stage("Setup Kibana ${version}") {
            // cleanup
            deleteDir()

            dir('plugin') {
              unstash 'source'
              docker.build("kibana-branch:${version}", "--build-arg=branch=v${version} .")
            }
          }

          stage("Test and validate against Kibana ${version}") {
            configFileProvider([
                configFile(
                  fileId: 'be5bdcbb-d40a-44ea-864a-dcc5d543319d',
                  targetLocation: 'plugin/.npmrc')
            ]) {
              docker.image("kibana-branch:${version}").inside('-v /etc/ssl/certs:/etc/ssl/certs:ro') {
                sh """
                ln -s /kibana kibana

                cd plugin

                PLUGIN_PATH=\$(pwd)

                npm i
                npm run lint .
                npm run test:server

                cd ../kibana

                npm run test:browser -- --kbnServer.plugin-path=\${PLUGIN_PATH} --kbnServer.tests_bundle.pluginId=cockpit --browser=PhantomJS
                """
              }
            }
          }

          stage("Build plugin for Kibana ${version}") {
            configFileProvider([
              configFile(
                fileId: 'be5bdcbb-d40a-44ea-864a-dcc5d543319d',
                targetLocation: '.npmrc')
            ]) {
              docker.image("kibana-branch:${version}").inside('-v /etc/ssl/certs:/etc/ssl/certs:ro') {
                sh """
                ln -s /kibana kibana

                cd plugin

                PLUGIN_PATH=\$(pwd)

                find . -type d -name __tests__ -exec rm -rf {} +

                npm run build -- --kibana-version ${version} --build-destination \${PLUGIN_PATH}/build
                """
              }

              dir('plugin/build') {
                stash name: "plugin-for-${version}"
              }
            }
          }
        }
      }
    }

    parallel kibanaStages

    stage('Publish artifacts') {
      unstash 'source-without-tests'

      def currentBranch = sh(script: 'git name-rev --name-only HEAD', returnStdout: true).trim()
      def publish = currentBranch.endsWith('master')
      def latestTag

      configFileProvider([
        configFile(
          fileId: 'be5bdcbb-d40a-44ea-864a-dcc5d543319d',
          targetLocation: '.npmrc')
        ]) {
        docker.image("node:${nodeVersion}").inside('-v /etc/ssl/certs:/etc/ssl/certs:ro') {
          def name = getName(readFile('package.json'))
          latestTag = sh(script: 'git tag --sort version:refname | tail -1', returnStdout: true).trim()
          def latestVersion = sh(script: "npm show ${name} version 2>/dev/null || echo 0.0.0", returnStdout: true).trim()

          publish = publish && isNewVersion(latestTag, latestVersion)

          if (publish) {
            sh 'npm publish'
          }
        }
      }

      if (publish) {
        for (version in kibanaVersions) {
          deleteDir()
          unstash name: "plugin-for-${version}"

          withCredentials([
            usernameColonPassword(
              credentialsId: 'a24c724c-4a4e-4d9b-bb61-5ea3d98a4c6f',
              variable: 'NEXUS_CREDENTIALS')
          ]) {
            sh """
            BUILD_FILE=\$(ls)
            REPO_URL=https://artifacts.ehex.de/repository/raw-npm-internal/kibana-cockpit-plugin/kibana-${version}-cockpit-plugin-${latestTag.substring(1)}.zip
            curl -k -s --user ${env.NEXUS_CREDENTIALS} --upload-file \$BUILD_FILE \$REPO_URL
            """
          }
        }
      }
    }
  } catch(error) {
    mattermostSend color: '#FF0000', message: "Pipeline **${env.JOB_NAME}** broken.\nDetails: ${env.BUILD_URL}"
    throw error;
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
def getKibanaVersions(packageJson) {
  def json = new groovy.json.JsonSlurper().parseText(packageJson)
  return json.kibana.supported
}

@NonCPS
def getName(packageJson) {
  def json = new groovy.json.JsonSlurper().parseText(packageJson)
  return json.name
}
