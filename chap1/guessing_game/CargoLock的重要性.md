### Cargo Lock的重要性

- 指明了一个Cargo项目究竟需要哪些依赖包（通过使用Cargo build时调用Cargo.toml写入）。
- Cargo Lock可以确保Cargo项目的构建是可重现的：即当因更新内部依赖而产生了致命错误时，可使用cargo.lock重现以往的稳定版本。
- 当Cargo.lock一旦生成，在将来构建项目时，除非在Cargo.toml中指定了某个依赖的版本变化，否则Cargo将直接从Cargo.toml中获取依赖版本并进行构建。
