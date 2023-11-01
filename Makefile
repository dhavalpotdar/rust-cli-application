install:
	pip install --upgrade pip &&\
		pip install -r requirements.txt

	cargo install mdbook
#install node
#curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash - &&\
#sudo apt-get install -y nodejs
#npm install -g @githubnext/github-copilot-cli
	
	echo 'eval "$(github-copilot-cli alias -- "$0")"' >> ~/.bashrc

build:
	cargo build --release

publish:
	cp crud/target/release/crud ./"Binary"

serve:
	mdbook serve -p 8000 -n 127.0.0.1 data-eng-rust-tutorial 

test:
# python -m pytest -vv --cov=main --cov=src tests/test_*.py
	@echo "Testing all projects with cargo"
	bash ./test.sh

format:	
# black src/lib/*.py src/*.py tests/*.py
	@echo "Formatting all projects with cargo"
	bash ./format.sh

lint:
	# pylint --disable=R,C --ignore-patterns=test_.*?py src/lib/*.py src/*.py
	@echo "Linting all projects with cargo"
	@rustup component add clippy 2> /dev/null
	bash ./lint.sh

run: 
	cargo run

release:
	cargo build --release

container-lint:
	docker run --rm -i hadolint/hadolint < Dockerfile

refactor: 
	format lint

deploy:
#deploy goes here
		
all: format lint test run