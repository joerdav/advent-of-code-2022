# Advent of Code 2022

## Tasks

### run

Run all exercises.

```sh
ls -d */ | grep -v template | xargs -I % sh -c 'cd %;cargo run -q main.rs;'
```
