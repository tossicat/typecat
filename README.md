# typecat

MD 파일을 이용한 pdf 출력 구현

## goal

- md -> pdf
- md -> docs
- md -> hwp

## 테스트 방법

현재 개발중이기 때문에 적절하게 작동하지 않습니다. 이 프로젝트는 CLI(Command-Line Interface, 이하 커맨드 라인 인터페이스)에서 작동하는 앱을 만드는 프로젝트이기 때문에 다음과 같이 실행하시면 테스트를 할 수 있습니다.

```rust
cargo run -- test.md default.toml
```

## markdown

마크다운 파서는 아래 페이지를 바탕으로 개발되었습니다.

- [Basic writing and formatting syntax - GitHub Docs](https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax#headings)

## 코드 작성한 다음 정리하기

코딩 스타일을 맞추기 위해서 코드를 올리기 전에 다음 명령어를 이용하여 코드를 정리하여 올립니다.

```console
cargo fmt
```

그런 다음 `cargo clippy` 명령어로 작성하신 코드의 문제점을 파악하고 이를 수정해 주세요.

```cosole
cargo clippy
```

<!-- 그리고 마지막으로 테스트를 실행한 다음 문제가 없다면 코드를 올려주세요.

```cosole
cargo test
``` -->
