_default:
  @just --list

# Run locally
dev:
    zellij --layout dev_layout.kdl

# Run all Hurl tests
test:
    hurl tests/*.hurl --test

# Deploy to Shuttle
deploy:
    cargo shuttle deploy --allow-dirty
