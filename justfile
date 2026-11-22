package := "csvsentry-cli"

check:
    cargo test --workspace

# 0.1.1 -> 0.1.2
patch: check
    cargo release -p {{ package }} patch
    cargo release -p {{ package }} patch --execute

# 0.1.1 -> 0.2.0
minor: check
    cargo release -p {{ package }} minor
    cargo release -p {{ package }} minor --execute

# 0.1.1 -> 1.0.0
major: check
    cargo release -p {{ package }} major
    cargo release -p {{ package }} major --execute
