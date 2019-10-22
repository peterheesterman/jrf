# jrf (JSON rust formatter)

JSON formatter cli - something simple to use jsonist `https://github.com/peterheesterman/jsonist`

Much less useful than jq - also a lot simpler!

## Installation

Cargo installed already? `cargo install jrf`

If not... find out [how to install cargo!](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## Usage

`jrf file_to_format.json`

```
 // file_to_format.json
 { "Key of an obect": ["some", "values", "in", "a", "list"] }
```

## Result

```
// file_to_format.json
{
    "Key of an obect": [
        "some",
        "values",
        "in",
        "a",
        "list"
    ]
}
```

## Interesting

I wanted to test this on a really large file so I got one from here https://github.com/zemirco/sf-city-lots-json and it took about 40 seconds on my computer to take this `189.9mb` file and format it into a `561mb` monster.
