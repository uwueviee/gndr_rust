//! [Original Package](https://github.com/tjhorner/gndr)
//!
//! `gndr` is a breakthrough in automated gender detection.
//!
//! It uses advanced techniques and algorithms to determine the gender of a user by **just fucking asking them.**
//!
//! The API couldn't be simpler. All you need to do is **ask the user what gender they identify as** along with **what pronouns they use**, and the library will give you back which gender they are with 100% accuracy. Incredible.

/**
The mystical gender container.
**/
pub struct Gndr {
    pub gender: String,
    pub pronouns: Vec<String>
}

/**
Calculates the user's gender using advanced techniques and algorithms.

Determines the gender of a user by **fucking asking them**.

Returns the gender in a Gndr struct with 100% accuracy. Incredible.

```rust
use gndr_rust::*;

fn main() {
    // what is this user's gender and pronouns?
    let gender =  identify_gender("non-binary".to_string(), vec!["he".to_string(), "they".to_string()]);
}
```
Result:
```
Gndr{
    gender: "non-binary",
    pronouns: [ "he", "they" ]
}
```
**/
pub fn identify_gender (gender: String, pronouns: Vec<String>) -> Gndr {
    return Gndr{
        gender,
        pronouns
    }
}