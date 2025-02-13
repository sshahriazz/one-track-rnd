PROJECT_DIR = /Users/polash/Desktop/one-track-rnd/ot-server

build-server:
	cd $(PROJECT_DIR) && cargo build

run-server:
	cd $(PROJECT_DIR) && cargo run

clean-server:
	cd $(PROJECT_DIR) && cargo clean
	rm -rf $(PROJECT_DIR)/target

check-deps:
	cd $(PROJECT_DIR) && cargo check
add-deps:
	cd $(PROJECT_DIR) && cargo add