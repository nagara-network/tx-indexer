MAKEFLAGS	+=	--jobs 1 --silent --environment-overrides
SHELL		:=	/bin/bash
CPU_ARCH	:=	$(shell if [[ "$(shell uname -m)" = "x86_64" ]]; then echo amd64; else echo arm64; fi)
IMAGE_NAME	:=	ghcr.io/nagara-network/tx-indexer:${CPU_ARCH}
RUST_BUILD_ARG	:=	-C target-cpu=generic

.PHONY: all check debug release docker docker-push refresh clean
.ONESHELL: all check debug release docker docker-push refresh clean

all: | release

refresh: | clean check

check:
	@echo -e "\033[34m\nClippy Check...\033[0m"
	@cargo clippy --all -- -D warnings
	@echo -e "\033[34m\nFormatting Check...\033[0m"
	@cargo fmt --all --check
	@echo -e "\033[34m\nUnit & Integration Testing...\033[0m"
	@cargo test --all-features

debug:
	@echo -e "\033[34m\nDebug Build...\033[0m"
	@cargo build

release:
	@echo -e "\033[34m\nRelease Build...\033[0m"
	@cargo build --release

clean:
	@echo -e "\033[34m\nCleaning up...\033[0m"
	@rm -rf data target Cargo.lock
	@cargo clean

docker:
	@echo -e "\033[34m\nDocker Build...\033[0m"
	@echo -e "\033[37m\nRUST_BUILD_ARG=\"${RUST_BUILD_ARG}\"\n\033[0m"
	@docker build --build-arg CPU_ARCH="${CPU_ARCH}" \
		--build-arg RUST_BUILD_ARG="${RUST_BUILD_ARG}" \
		-t ${IMAGE_NAME} \
		.

docker-push: | docker
	@echo -e "\033[34m\nDocker Push...\033[0m"
	@docker push ${IMAGE_NAME}
