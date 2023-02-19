# slowapi

Asynchronous API to simulate slow requests for client testing purposes

- Implements TLS using a self signed certificate generated using [`openssl`](https://www.openssl.org/)
- Requesting client must have an insecure/invalid certificate flag enabled

## Examples

### Python 「[aiohttp](https://github.com/aio-libs/aiohttp)」

```py
import asyncio
from aiohttp import ClientSession, TCPConnector

async def main():
    delay_ms = 2000
    url = f"https://127.0.0.1:8080/v1/slow?delay={delay_ms}"
    session = ClientSession(connector=TCPConnector(ssl=False))

    async with session.get(url) as resp:
        resp = await resp.text()
        print(resp)

asyncio.run(main())

```

### Rust 「[reqwest](https://github.com/seanmonstar/reqwest)・[tokio](https://github.com/tokio-rs/tokio)」

```rs
use reqwest::{Client, Url};

#[tokio::main]
async fn main() -> Result<(), Box<dyn ::std::error::Error + Send + Sync>> {
	let http_client = Client::builder().danger_accept_invalid_certs(true).build()?;
	let delay_ms = 2000u32;
	let url = Url::parse(format!("https://127.0.0.1:8080/v1/slow?delay={delay_ms}").as_str())?;
	let resp = http_client.get(url).send().await?.text().await?;
	println!("{resp}");

	Ok(())
}

```

### Shell 「[curl](https://github.com/curl/curl)」

```sh
curl --insecure https://127.0.0.1:8080/v1/slow\?delay\=2000

```
