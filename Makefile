#!/usr/bin/make -f

SHELL:=$(shell which bash)

.PHONY: docker-amd64 docker-arm64

IMAGE_PREFIX?=naftulikay/lambda-ecr-rewrite
IMAGE_VERSION?=latest

docker-amd64:
	@docker buildx build --load --platform linux/amd64 --build-arg ARCH=amd64 -t $(IMAGE_PREFIX)-amd64:$(IMAGE_VERSION) .

docker-arm64:
	@docker buildx build --load --platform linux/arm64 --build-arg ARCH=arm64 -t $(IMAGE_PREFIX)-arm64:$(IMAGE_VERSION) .