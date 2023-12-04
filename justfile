_default:
  @just --list

# Run locally
dev:
    zellij --layout dev_layout.kdl

# Run all Hurl tests
test:
    hurl tests/*.hurl --test --verbose

# Deploy to Shuttle
deploy:
    cargo fmt
    cargo clippy -- -Dwarnings
    cargo shuttle deploy --allow-dirty

# Generate files for $day-number (ex 1)
generate day_number:
    touch tests/day{{day_number}}.hurl
    cp src/challenges/day0.rs src/challenges/day{{day_number}}.rs
    echo -e "\npub mod day{{day_number}};" >> src/challenges/mod.rs