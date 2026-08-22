# Gothab status canary

This public repository is a deterministic end-to-end canary for Gothab's Git and search services.

It is intentionally tiny. The marker and symbol in `src/canary.rs` are consumed by the independent synthetic monitor on monitor-1. Do not rename or remove them without updating the monitor configuration in the Gothab infrastructure repository.

This branch is retained as a stable public pull-request fixture.
