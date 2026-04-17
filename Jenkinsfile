pipeline {
    agent any
    options {
        timestamps()
    }

    stages {
        stage('Setup Rust') {
            steps {
                sh '''
                    set -eu
                    if ! command -v cargo >/dev/null 2>&1; then
                      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
                    fi
                '''
            }
        }

        stage('Build') {
            steps {
                sh '. "$HOME/.cargo/env" && cargo build --release'
            }
        }

        stage('Test') {
            steps {
                sh '. "$HOME/.cargo/env" && cargo test'
            }
        }
    }
}
