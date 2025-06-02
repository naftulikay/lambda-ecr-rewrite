# lambda-ecr-rewrite [![Build Status][build.svg]][build]

A simple Lambda function in Rust, available as a Docker image, for rewriting all incoming requests to an ECR registry.
This makes it easy to create an "alias" for ECR such as `docker.mycompany.com` as opposed to the awful default name.

 - 🤢 `123456789012.dkr.ecr.us-east-1.amazonaws.com`
 - 😎 `docker.mycompany.com`

## Configuration

Two configuration parameters are available as environment variables:

 1. `ECR_REGISTRY_HOST`: set this to the FQDN of your ECR registry, such as `123456789012.dkr.ecr.us-east-1.amazonaws.com`
 2. `CACHE_MAX_AGE`: set this to a positive integer in seconds to be used with `Cache-Control`'s `max-age` parameter for
    HTTP responses.
 3. `DEBUG`: set this to any of `y | yes | true` to enable debug logging of request and response payloads to standard
    error.

If you receive an HTTP 500, it is most likely that you did not configure `ECR_REGISTRY_HOST`.

## Deployment

Lambda can only pull images _from ECR_. To that end, we build and push a Docker image to public ECR for your use. Images
are tagged with their corresponding semantic version such as `0.1.0`. Set your Lambda function's Docker image URI as:

#### [x86_64/amd64](https://gallery.ecr.aws/naftulikay/lambda-ecr-rewrite-amd64)

```text
public.ecr.aws/naftulikay/lambda-ecr-rewrite-amd64:0.1.0
```

#### [ARM64](https://gallery.ecr.aws/naftulikay/lambda-ecr-rewrite-arm64)

```text
public.ecr.aws/naftulikay/lambda-ecr-rewrite-arm64:0.1.0
```

A Terraform module implementing all of this is forthcoming.

## License

Licensed at your discretion under either:

 - [Apache Software License, Version 2.0](./LICENSE-APACHE)
 - [MIT License](./LICENSE-MIT)

 [build]:     https://github.com/naftulikay/lambda-ecr-rewrite/actions/workflows/rust.yml
 [build.svg]: https://github.com/naftulikay/lambda-ecr-rewrite/actions/workflows/rust.yml/badge.svg
