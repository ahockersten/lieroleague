# lieroleague
Software for managing a league for Liero

# Install instructions (Ubuntu)

## Install Rust (follow their instructions to add rust commands to your path)
    curl https://sh.rustup.rs -sSf | sh
## Setup Rust nightly
    cd backend
    rustup override set nightly
## Install Node
    curl -sL https://deb.nodesource.com/setup_12.x | sudo -E bash -
    sudo apt-get install -y nodejs
## Install Yarn
    curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | sudo apt-key add -
    echo "deb https://dl.yarnpkg.com/debian/ stable main" | sudo tee /etc/apt/sources.list.d/yarn.list
    sudo apt-get update && sudo apt-get install yarn

# Running frontend (Ubuntu)
    cd frontend
    yarn start

# Running backend (Ubuntu)
    cd backend
    cargo run
