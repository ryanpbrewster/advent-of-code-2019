load("@io_bazel_rules_rust//rust:rust.bzl", "rust_test")

rust_test(
    name = "day06",
    srcs = glob(["*.rs"]),
    data = [":input"],
    deps = [
        "//files",
    ],
)

filegroup(
    name = "input",
    srcs = ["input.txt"],
)
