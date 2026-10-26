# Seeing Rust — 配套源码

[English](README.md) | **中文说明**

这里是 **《Seeing Rust: A Visual Guide to Ownership, Borrowing, and Real Systems
Code》**（Horace pei，2026）全书的可运行代码，按章节拆成 35 个 Cargo 包。每个包
都能独立编译运行；同时 35 个包共用一个 workspace，所以也可以一次全部构建。

克隆下来、跑通第 1 章，你就已经在用它了：

```bash
git clone https://github.com/daemonpei/seeing-rust
cd seeing-rust
cargo run -p ch01-hello
```

```
Rust is ready for systems work.
```

---

## 目录

- [快速开始](#快速开始)
- [环境要求](#环境要求)
- [仓库里都有什么](#仓库里都有什么)
- [运行某一章](#运行某一章)
- [章节总表](#章节总表)
- [需要额外操作的章节](#需要额外操作的章节)
- [配合书一起读](#配合书一起读)
- [常见问题排查](#常见问题排查)
- [许可与署名](#许可与署名)

---

## 快速开始

```bash
# 1. 单章运行 —— 只编译这一章和它的依赖
cargo run -p ch09-ownership

# 2. 全部构建 —— 35 个包，首次几分钟
cargo build --workspace

# 3. 跑全书所有测试
cargo test --workspace
```

**跟着书读不需要先全量构建。** 用第 1 条就行，cargo 只会编译该章用到的东西。
`cargo build --workspace` 是用来验证仓库没坏的，不是读第 4 章的前置步骤。

---

## 环境要求

| | |
|---|---|
| Rust | **1.82 及以上**。`rust-toolchain.toml` 锁定 `stable` 并附带 `rustfmt` + `clippy`，`rustup` 会在首次使用时自动装好。 |
| 网络 | 只在首次构建下载依赖时需要。**之后所有程序都能离线运行** —— 需要 HTTP 的章节会在 `127.0.0.1` 上起一个临时服务器。 |
| OpenSSL | 不需要。HTTP 全部走 `rustls`，Windows 上也没有额外要装的东西。 |
| 磁盘 | 全量构建后共享的 `target/` 会有数 GB。源码本身不到 1 MB；`target/` 已被 git 忽略，不会进仓库。 |
| 系统 | Linux / macOS / Windows 均可。第 16 章的路径写法在三边上都能跑。 |

---

## 仓库里都有什么

```
seeing-rust/
├── Cargo.toml                 ← workspace 清单，列出全部 35 个成员
├── Cargo.lock                 ← 锁定的依赖版本，是故意提交的
├── rust-toolchain.toml        ← stable + rustfmt + clippy
├── README.md                  ← 英文说明
├── README.zh-CN.md            ← 本文件
├── ch01-hello/                ← 一章一个包
│   ├── Cargo.toml
│   ├── README.md              ← 这一章在演示什么
│   └── src/
├── ch02-setup/
│   ├── NOTES.md               ← 不属于 Rust 代码的 shell 命令
│   └── src/
├── …
└── ch35-reading/
```

动手前要知道两件事：

- **根目录的 `Cargo.toml` 是 workspace，不是程序。** 它没有 `src/`，作用只是让
  35 个包共用同一个 `target/` 和同一个 `Cargo.lock` —— `cargo build --workspace`
  能跑起来靠的就是它。
- **`Cargo.lock` 是提交的。** 这些是可执行程序而不是库，所以精确版本是交付的一
  部分：你会拿到与书写作时相同的 `serde`、`tokio`、`axum`。（为什么库要反过来
  做，第 2 章有讲。）

---

## 运行某一章

每一章都是可执行程序，所以命令都是同一行：

```bash
cargo run -p <包名>
```

`-p` 后面跟的是包名，也就是目录名 —— 是 `ch09-ownership`，不是 `ch09`，也不是
`ownership`。

### `--` 很重要

`--` **前面**的参数给 cargo，`--` **后面**的参数给你的程序：

```bash
cargo run -p ch19-cli -- --needle 503 -n     # --needle 传给那个命令行工具
cargo run -p ch19-cli --help                 # --help 给了 cargo，不是那个工具
```

如果你给程序的参数被报 "unexpected argument"，十有八九是漏了 `--`。

### 其他常用写法

```bash
cargo run  -p ch27-performance --release -- --bench   # 优化构建
cargo test -p ch26-testing                            # 跑某一章的测试
cargo test -p ch26-testing classify                   # 按名字跑单个测试
cargo test --workspace                                # 全书测试
cargo build --workspace --release                     # 优化版，最慢
cargo clean                                           # 删掉 target/（数 GB）
```

---

## 章节总表

默认命令都是 `cargo run -p <包名>`，最后一列只列**不一样**的地方。

| 章 | 包名 | 章节标题 | 额外说明 |
|---|---|---|---|
| 1 | `ch01-hello` | Rust 是什么，为什么系统工程师用它 | |
| 2 | `ch02-setup` | 搭建 Rust 环境 | 另跑 `--release`；命令在 `NOTES.md` |
| 3 | `ch03-first-program` | 你的第一个 Rust 程序 | |
| 4 | `ch04-values` | 值、变量与类型 | |
| 5 | `ch05-expressions` | 运算符、表达式与基本判断 | |
| 6 | `ch06-loops` | 循环与重复工作 | |
| 7 | `ch07-functions` | 函数与程序分解 | |
| 8 | `ch08-errors` | 错误与显式失败处理 | |
| 9 | `ch09-ownership` | 所有权：Rust 如何管理内存 | |
| 10 | `ch10-borrowing` | 借用、引用与生命周期 | |
| 11 | `ch11-structs` | 结构体、枚举与领域建模 | |
| 12 | `ch12-collections` | 动态数组、字符串与哈希表 | |
| 13 | `ch13-modules` | 包、模块与项目组织 | |
| 14 | `ch14-traits` | trait 与组合 | |
| 15 | `ch15-generics` | 泛型：可复用的类型安全代码 | |
| 16 | `ch16-files` | 文件 I/O 与路径 | 会在当前目录写出文件 |
| 17 | `ch17-text` | 文本处理与正则表达式 | |
| 18 | `ch18-json` | JSON 与结构化数据 | |
| 19 | `ch19-cli` | 命令行程序 | **需要传参数**（见下） |
| 20 | `ch20-http-client` | HTTP 客户端与 API 调用 | 会在 `127.0.0.1` 起本地服务 |
| 21 | `ch21-http-server` | HTTP 服务器与 REST 风格 API | **常驻不退出**（见下） |
| 22 | `ch22-async` | 异步 Rust、Future 与 Tokio | |
| 23 | `ch23-threads` | 线程与并发任务 | |
| 24 | `ch24-channels` | 通道与消息传递 | |
| 25 | `ch25-shared-state` | 共享状态：Arc、Mutex 与原子类型 | 另见 `NOTES.md` |
| 26 | `ch26-testing` | 用 Cargo 做测试 | **要用 `cargo test`** |
| 27 | `ch27-performance` | 基准测试、性能剖析与优化 | **请用 `--release`** |
| 28 | `ch28-observability` | 日志、配置与可观测性 | 需要 `RUST_LOG=…`（见下） |
| 29 | `ch29-healthcheck` | 项目 1：健康检查命令行工具 | **需要传参数**（见下） |
| 30 | `ch30-config-validator` | 项目 2：JSON 配置校验器 | **需要传文件**（见下） |
| 31 | `ch31-rest-api` | 项目 3：小型 REST API 服务器 | **常驻不退出**（见下） |
| 32 | `ch32-dependencies` | 依赖管理与第三方 crate | feature + `cargo tree` |
| 33 | `ch33-safety` | 安全、unsafe Rust 与安全基础 | |
| 34 | `ch34-release` | 构建、发布与运行 Rust 程序 | 另跑 `--release`；见 `NOTES.md` |
| 35 | `ch35-reading` | 阅读 Rust 代码与继续学习 | **需要传文件**（见下） |

按部分划分：1–3 入门导引 · 4–8 语言核心 · 9–10 所有权 · 11–15 数据建模 ·
16–22 实战 · 23–25 并发 · 26–28 工程化 · 29–31 综合项目 · 32–35 交付与进阶。

---

## 需要额外操作的章节

### 需要一直开着的服务器（第 21、31 章）

这两章不会自己退出，会一直监听端口直到你 `Ctrl-C`。在一个终端里跑起来，在另一
个终端里访问：

```bash
# 终端 1
cargo run -p ch21-http-server

# 终端 2
curl localhost:8080/health
curl localhost:8080/items
curl -X POST localhost:8080/items -H 'content-type: application/json' -d '{"name":"widget"}'
curl localhost:8080/items/item-0001
```

两章都用 **8080 端口**，所以不要同时运行。如果不想开第二个终端，它们各自都有不
需要端口的测试：

```bash
cargo test -p ch21-http-server
cargo test -p ch31-rest-api
```

Windows PowerShell 里 `curl` 是 `Invoke-WebRequest` 的别名、参数完全不同，请用
`curl.exe`。

### 需要传参数的章节

```bash
cargo run -p ch19-cli                          # 处理生成的示例文本
cargo run -p ch19-cli -- --needle 503 -n       # 换个关键词，显示行号
cargo run -p ch19-cli -- Cargo.toml            # 对任意文本文件也能跑
cargo run -p ch19-cli -- --help                # 工具自己的帮助

cargo run -p ch29-healthcheck -- --selftest            # 离线：2 个正常 + 2 个异常
cargo run -p ch29-healthcheck -- --selftest --json
printf 'http://127.0.0.1:9/\n' | cargo run -p ch29-healthcheck -- - --workers 1

cargo run -p ch30-config-validator -- examples/good.json ; echo "exit=$?"
cargo run -p ch30-config-validator -- examples/bad.json  ; echo "exit=$?"
cargo run -p ch30-config-validator -- examples/bad.json --json

cargo run -p ch35-reading                      # 输出它自己的源码纲要
cargo run -p ch35-reading -- --self
cargo run -p ch35-reading -- path/to/file.rs
cargo run -p ch35-reading -- --docs
```

`--selftest` 会自己起一个本地服务器，所以第 29 章能演示真实的 HTTP 健康检查，
全程不碰公网。

### 构建档位本身就是重点的章节（第 2、27、34 章）

debug 和 release 在 Rust 里真的是两个不同的程序 —— 整数溢出在前者 panic、在后者
回绕。这几章两种方式都跑一遍再对比：

```bash
cargo run -p ch02-setup
cargo run -p ch02-setup --release

cargo run -p ch27-performance -- --bench
cargo run -p ch27-performance --release -- --bench    # 通常快 10–50 倍

cargo run -p ch34-release
cargo run -p ch34-release --release
```

**永远不要拿 debug 构建的耗时当作性能结论。**

### 日志（第 28、31 章）

```bash
RUST_LOG=debug cargo run -p ch28-observability
RUST_LOG=ch28_observability=trace cargo run -p ch28-observability
RUST_LOG=debug cargo run -p ch31-rest-api
```

Windows 上先设变量再运行：`$env:RUST_LOG="debug"`，然后 `cargo run …`。

### feature 与依赖树（第 32 章）

```bash
cargo run -p ch32-dependencies
cargo run -p ch32-dependencies --no-default-features
cargo run -p ch32-dependencies --features pattern
cargo tree -p ch32-dependencies
cargo tree -p ch32-dependencies --all-features --edges features
```

---

## 配合书一起读

书里印的是代码片段，因为一页纸放不下全部。这里没有删减，所以有几个约定值得先
知道：

- **本来就该被编译器拒绝的代码**（值被移动、借用重叠）以**注释块**的形式保留，
  下面附上真实的 `error[E…]:` 报错信息。取消注释再 `cargo build` 就能复现那个错
  误；紧跟着的可运行版本，就是这一章要论证的修法。**所以如果某一章编译不过，通
  常是你而不是仓库的问题** —— 先检查是不是把演示块取消注释了。
- **shell 命令**（`rustup`、`cargo tree`、`perf`）放在各包的 `README.md` 或
  `NOTES.md` 里，不在 `src/` 里。
- **`Cargo.toml` 与配置片段**，关键处是真实可用的配置，只是举例说明的地方则以
  引用文本出现。
- **涉及网络的章节**会在后台线程起一个 `127.0.0.1` 上的临时服务器，因此能离线
  运行并自行结束。`cargo test` 全程不会访问公网。
- **每个包都有自己的 `README.md`** 解释它在演示什么。那是讲解，`src/` 才是书里
  引用的代码。

---

## 常见问题排查

| 现象 | 原因与处理 |
|---|---|
| `error: no package named ch09` | `-p` 要写完整目录名：`cargo run -p ch09-ownership`。 |
| `unexpected argument '--needle'` | cargo 参数与程序参数之间漏了 `--`。 |
| `Address already in use`（48 / 98 / 10048） | 8080 被占用，多半是第 21 或 31 章还在跑，`Ctrl-C` 掉。 |
| `curl` 打出 PowerShell 帮助 | Windows 把 `curl` 别名成了 `Invoke-WebRequest`，改用 `curl.exe`。 |
| 出现了没预料到的编译错误 | 你取消了某段「这里应该报错」的注释，见上文说明。 |
| 构建极慢 / 磁盘满了 | `cargo clean` 一次清掉整个 `target/`（数 GB），代价是下次全量重编。 |
| 装完仍然 `cargo: command not found` | 重开终端，或执行 `source "$HOME/.cargo/env"`。 |
| 离线机器首次构建 | 先在有网环境构建一次，之后设 `CARGO_NET_OFFLINE=true`。 |
| `rustup` 要下载工具链 | 首次运行的正常行为，`rust-toolchain.toml` 锁定的是 `stable`。 |

---

## 许可与署名

MIT。可以随意用于你自己的项目；注明出处欢迎，但不是必须。

图书：*Seeing Rust: A Visual Guide to Ownership, Borrowing, and Real Systems
Code* — **Horace pei**，2026 — `1091186233@qq.com`
