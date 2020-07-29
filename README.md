#gndr: Determine gender programmatically (Rust Edition)

[Original Package](https://github.com/tjhorner/gndr)

`gndr` is a breakthrough in automated gender detection.

It uses advanced techniques and algorithms to determine the gender of a user by **just fucking asking them.**

The API couldn't be simpler. All you need to do is **ask the user what gender they identify as** along with **what pronouns they use**, and the library will give you back which gender they are with 100% accuracy. Incredible.

## Example
```rust
use gndr_rust::*;

fn main() {
    // what is this user's gender and pronouns?
    let gender =  identify_gender("non-binary".to_string(), vec!["he".to_string(), "they".to_string()]);

    /**
    Result:
    Gndr{
        gender: "non-binary",
        pronouns: [ "he", "they" ]
    }
    **/
}
```

## License

MIT, but I hope you aren't actually thinking of using this
