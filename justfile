[private]
default:
    @just --list

today := "day-`date +%d`"
default_part := "part1"

# Create a Flamegraph
flamegraph day=today part="part1":
    cargo flamegraph --profile flamegraph --package {{day}}  --bin {{part}} -o days/{{day}}/flamegraph-{{day}}-{{part}}.svg
    xdg-open days/{{day}}/flamegraph-{{day}}-{{part}}.svg

# Run with Heap Profiling
dhat day=today part=default_part:
    cargo run --profile dhat --features dhat-heap --package {{day}} --bin {{part}}

# Run Tests
test day=today part=default_part *flags='':
    cargo test --package {{day}} {{part}} {{flags}}

# Solve Input
solve day=today part=default_part *flags='':
    cargo run --package {{day}} --bin {{part}}

# Benchmark Solution
bench day=today part=default_part:
    cargo bench --package {{day}} --bin {{part}}
# Create New Project from Template
new day=today:
    cargo generate --path day-template --name {{day}} --destination days
