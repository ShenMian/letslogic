# letslogic

[![docs.rs](https://img.shields.io/docsrs/letslogic)](https://docs.rs/letslogic)
[![Test status](https://img.shields.io/github/actions/workflow/status/ShenMian/letslogic/test.yml?label=test)](https://github.com/ShenMian/letslogic/actions/workflows/test.yml)
[![Code coverage](https://img.shields.io/codecov/c/github/ShenMian/letslogic)](https://app.codecov.io/gh/ShenMian/letslogic)

A library providing interaction with the [Let's Logic API].

> [!WARNING]
>
> - The request implementation uses GET instead of POST.
> - Let's Logic API may incorrectly return empty responses.
> - Avoid concurrent requests.

## Features

- Get collection list.
- Get levels in collection.
- Submit level solution.
- Get records for completed levels.

## License

Licensed under either of

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT license](LICENSE-MIT)

at your option.

[Let's Logic API]: <https://letslogic.com/api>
