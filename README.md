# Secrets
Secrets is a simple library for retrieving environment specific variables. It's basically `dotenv` so you probably wanna use that instead.

## Important:
This library is designed to only work with utf8 characters.

## Usage
Create a secret instance and use like so:
```
use secrets::Secret;

let secrets = Secret::new("./tests/data/.secret");

let secret_value: String = secrets.tell("PASSWORD");

```