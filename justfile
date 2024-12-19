[no-cd]
@test:
    cargo nextest run

[no-cd]
@p1:
    cargo run --bin part1

[no-cd]
@p2:
    cargo run --bin part2

create day:
    cargo generate --path ./template/ --name {{day}}
