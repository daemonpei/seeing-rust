# Chapter 30 — Project 2: A JSON Configuration Validator

```bash
cargo run -p ch30-config-validator -- examples/good.json ; echo "exit=$?"
cargo run -p ch30-config-validator -- examples/bad.json  ; echo "exit=$?"
cargo run -p ch30-config-validator -- examples/bad.json --json
cargo test -p ch30-config-validator
```

```
  - service must not be empty
  - port 0 is outside 1..=65535
  - timeoutSeconds 9999 must be between 1 and 300
  - environment "prod" is not development|staging|production
  - replicas 99 exceeds 10
exit=1
```

## Report all problems, not the first

`validate` returns `Vec<Problem>`, not `Result<(), Problem>`. An operator with
five mistakes fixes them in one pass instead of five. The `Result` idiom is right
for control flow; it is wrong for a validation report.

The `Problem` enum (rather than `String`) is what lets `tests` assert with
`contains(&Problem::TooManyReplicas(99))`. A test asserting on message text
breaks every time someone improves a message.

## Three layers, three jobs

| Layer | Question | Failure mode |
|---|---|---|
| `serde_json::from_str` | is this the right *shape*? | `"port": "80"` → parse error |
| `validate` | are the values *meaningful*? | `port: 0` → `Problem` |
| the service | is this config *usable here*? | port already in use → bind error |

Collapsing the first two into one is the usual mistake: a custom `Deserialize`
that rejects `port == 0` produces a parse error with a byte offset, when what
the operator needed was a sentence. Let `serde` do shape, let `validate` do
meaning.

## Exit codes again

```
0  valid
1  parsed, but invalid
2  could not read or could not parse
```

`2` for "the input is not even JSON" is the difference between a CI step that
says "your config is wrong" and one that says "you pointed me at the wrong
file".

## `--json` mode

```json
{ "valid": false, "problems": ["service must not be empty", "..."] }
```

The same `run` function, one branch. Emitting both a human table and a JSON
object from the same validation is what makes a tool usable in a pipeline and
readable in a terminal — and it costs about eight lines.
