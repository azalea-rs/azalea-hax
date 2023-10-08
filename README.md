# Azalea Hax

An [Azalea](https://github.com/azalea-rs/azalea) plugin with useful features that will probably trigger anticheats.

## Usage

```rust
async fn handle(mut bot: Client, event: azalea::Event, state: State) -> anyhow::Result<()> {
    match event {
        azalea::Event::Init => {
            bot.set_anti_knockback(true);
        }
    }
}
```

## Features

- Anti-knockback
