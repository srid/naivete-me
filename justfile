default:
    @just --list

# Auto-format the source tree
fmt:
    treefmt

# Run the project
run *ARGS:
    trunk serve --port 3000 --open

