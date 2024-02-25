# Rustify

Rustify is an API wrapper for the spotify API.
It is intended to be easy to use while also being safe, and efficient.

## Notable Features:
- Centralized client for each of the main endpoints
- Cache system to reduce the amount of requests 
- Serialization of the data
- Well documented functions
- Error handling
- Rate limiting
- Asynchronous requests
- And much more!

## How to use:
To use rustify you first need to get a client id and client secret. This can be done through this [link](https://developer.spotify.com/documentation/web-api/tutorials/getting-started). While I work on ways to do this through spotify username and password (see the [TODO](#todo) list).
Then in a rust file import the nessisary libraries:
```rust
use rustify::artist::ArtistData;

use dotenv::from_path;

use std::env::var;
```

And add them to Cargo.toml:
```toml
rustify = "0.1.0"
dotenv = "0.15.0"
```

Then inside of a function initialize the ID and secret:

```rust
let path = r#"C:\Users\conno\rustify\tests\.env"#; // Change out for your path
from_path(path).unwrap();

let id: String = var("USER_ID").unwrap();
let secret: String = var("USER_SECRET").unwrap();
```

Inside of the function Initialize a client: 

```rust
let mut result = ArtistData::new(id, secret);
```
You are now able to use the functions related to the artist endpoint (or endpoint of your choosing).

```rust
let value = result.get_artist("0C0XlULifJtAgn6ZNCW2eu");
```
This function takes in the parameter of the artist ID which can be found here in the link to share the artist:
![A box highlighting the area after the "artist/" and before the "?" in the url to share an artist, this area is the artist ID](image.png)

But this does not atcctually make the request to do that you have to add the .run() at the end.
```rust
let value = result.get_artist("0C0XlULifJtAgn6ZNCW2eu").run();
```

You can then either print out the value that is returned as a ArtistResult object or use the inbuilt parsing functions to return speciffic sets of data:

```rust
println!("{:?}", value);
```

or 

```rust
let name = value.name();
println!("{:?}", name);
```

## TODO:
- More comprehensive cache system
- Better and more descriptive error handling
- More coverage of the API
- Alternate authentication methods

## Contributing:
All contributions are welcome! If you want to fix an error or add a feature submit a pull request explaining what you did.

## License:

[MIT](lICENSE) 

## Thanks!


