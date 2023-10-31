# Changelog

## [0.5.0](https://github.com/flipt-io/flipt-rust/compare/flipt-v0.4.6...flipt-v0.5.0) (2023-10-31)


### Features

* add flag_key to VariantEvaluationResponse and BooleanEvaluationResponse ([#29](https://github.com/flipt-io/flipt-rust/issues/29)) ([7651a6b](https://github.com/flipt-io/flipt-rust/commit/7651a6bea8f0817e7f5035d23d52d32c9824079c))


### Miscellaneous Chores

* release 0.5.0 ([662e6bd](https://github.com/flipt-io/flipt-rust/commit/662e6bdaeafe562ec3c44ce2e3b994b2418ce4f6))

## [0.4.6](https://github.com/flipt-io/flipt-rust/compare/flipt-v0.4.5...flipt-v0.4.6) (2023-08-01)


### Features

* **evaluate_v2:** Add evaluate code for new v2 endpoints ([ee77b91](https://github.com/flipt-io/flipt-rust/commit/ee77b91f8aa68fa55bf2c1ad1fae3458c4e2eeb7))
* **rollouts:** feat(rollouts):  ([82407b6](https://github.com/flipt-io/flipt-rust/commit/82407b6101ee0df20eb050f36745f1c612fc26c2))
* **rollouts:** Add CRUD code for rollouts ([4c605a0](https://github.com/flipt-io/flipt-rust/commit/4c605a065c19b118125ddb6099587b080aa9f022))

## [0.4.5](https://github.com/flipt-io/flipt-rust/compare/flipt-v0.4.4...flipt-v0.4.5) (2023-05-24)


### Features

* add datetime comparison type ([#24](https://github.com/flipt-io/flipt-rust/issues/24)) ([e720108](https://github.com/flipt-io/flipt-rust/commit/e720108bcac2e9c7c3aac817a02722a3b7bf6b8c))
* add description to constraint on client ([2a46798](https://github.com/flipt-io/flipt-rust/commit/2a46798105e4459c806f5a9e34e7728d2e8c7d58))

## [0.4.4](https://github.com/flipt-io/flipt-rust/compare/flipt-v0.4.3...flipt-v0.4.4) (2023-05-02)


### Bug Fixes

* use rustls instead of default tls ([#20](https://github.com/flipt-io/flipt-rust/issues/20)) ([4dffb17](https://github.com/flipt-io/flipt-rust/commit/4dffb17bb21bf455b1b08b3e2554492ae768b8ce))

## [0.4.3](https://github.com/flipt-io/flipt-rust/compare/flipt-v0.4.2...flipt-v0.4.3) (2023-05-02)


### Bug Fixes

* support tls ([#18](https://github.com/flipt-io/flipt-rust/issues/18)) ([e6e6645](https://github.com/flipt-io/flipt-rust/commit/e6e6645b3ca4684512ebf578f5283c986d97809e))

## [0.4.2](https://github.com/flipt-io/flipt-rust/compare/flipt-v0.4.1...flipt-v0.4.2) (2023-04-27)


### Features

* impl Clone for Config struct ([#15](https://github.com/flipt-io/flipt-rust/issues/15)) ([4d3f81f](https://github.com/flipt-io/flipt-rust/commit/4d3f81f4ceb1b043e7802d88db26b80f5df77781))

## [0.4.1](https://github.com/flipt-io/flipt-rust/compare/flipt-v0.4.0...flipt-v0.4.1) (2023-04-26)


### Features

* add meta/info client; add crate version to user-agent ([#13](https://github.com/flipt-io/flipt-rust/issues/13)) ([d1158f9](https://github.com/flipt-io/flipt-rust/commit/d1158f93267ea18efc525e26efa06dc8f594de1a))

## [0.4.0](https://github.com/flipt-io/flipt-rust/compare/flipt-v0.3.1...flipt-v0.4.0) (2023-04-11)

### Added

* Namespace functionality to clients [#8](https://github.com/flipt-io/flipt-rust/pull/8)

### Changed

* All methods use Request types as arguments (breaking change) [#8](https://github.com/flipt-io/flipt-rust/pull/8)

## [0.3.1](https://github.com/flipt-io/flipt-rust/compare/flipt-v0.3.0...flipt-v0.3.1) (2022-12-08)


### Added

* Initial implementation of Flipt API. ([9553c63](https://github.com/flipt-io/flipt-rust/commit/9553c630cc2fb8d7bae9eb2cd037b31aad8f2012))
