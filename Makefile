#!/usr/bin/make -f

SHELL:=$(shell which bash)

.PHONY: pre-commit watch invoke-v1 invoke-v2 docker-amd64 docker-arm64

IMAGE_PREFIX?=naftulikay/lambda-ecr-rewrite
IMAGE_VERSION?=latest

pre-commit:
	cargo test --workspace
	cargo fmt --check --all
	cargo check --workspace
	cargo clippy --workspace

watch:
	@ECR_REGISTRY_HOST=1234567890.dkr.ecr.us-east-1.amazonaws.com CACHE_MAX_AGE=60 cargo lambda watch

invoke-v1:
	cargo lambda invoke --data-example apigw-request

invoke-v2:
	cargo lambda invoke --data-example apigw-v2-request-no-authorizer

docker-amd64:
	@docker buildx build --load --platform linux/amd64 --build-arg ARCH=amd64 -t $(IMAGE_PREFIX)-amd64:$(IMAGE_VERSION) .

docker-arm64:
	@docker buildx build --load --platform linux/arm64 --build-arg ARCH=arm64 -t $(IMAGE_PREFIX)-arm64:$(IMAGE_VERSION) .