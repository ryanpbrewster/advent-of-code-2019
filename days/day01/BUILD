load("@io_bazel_rules_rust//rust:rust.bzl", "rust_test")

rust_test(
  name = "day01",
  srcs = glob(["*.rs"]),
  data = [ ":input" ],
)

filegroup(
  name = "input",
  srcs = ["input.txt"],
)
