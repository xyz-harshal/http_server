pipeline {
    agent any

    stages {
        stage('Clone') {
            steps {
                echo "Cloning..."
            }
        }

        stage('Build') {
            steps {
                sh 'echo Building project'
            }
        }

        stage('Test') {
            steps {
                sh 'echo Running tests'
            }
        }

        stage('Deploy') {
            steps {
                sh 'echo Deploying app'
            }
        }
    }
}
