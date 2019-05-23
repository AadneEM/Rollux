pipeline {
    agent any
    stages {
	stage('docker build') {
	    steps {
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
