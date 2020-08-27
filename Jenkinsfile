pipeline {
    agent any
    stages {
		stage('build') {
			agent {
				docker {
					image 'rust'
					reuseNode true
				}
			}
			steps {
				sh 'cargo build --release'
				stash includes: 'target/release/rollux', name: 'app'
			}
		}
		stage('docker build') {
			steps {
				unstash 'app'
				echo 'Starting docker image build'
				script {
					docker.withRegistry("https://pkg.northcode.no", 'docker-login') {
						def image = docker.build("pkg.northcode.no/rollux")
						image.push()
					}
				}
			}
		}
	}
}
