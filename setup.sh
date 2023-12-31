#!/usr/bin/env bash
virtualenv ~/.venv
source ~/.venv/bin/activate
make install

#append it to bash so every shell launches with it 
echo 'source ~/.venv/bin/activate' >> ~/.bashrc

## Install rustup and common components
curl https://sh.rustup.rs -sSf | sh -s -- -y 
source "$HOME/.cargo/env"
