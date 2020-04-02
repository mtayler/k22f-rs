# k22f
This crate provides an autogenerated API for access to FRDM-K22F peripherals.
The API is generated using [svd2rust] with SVD files from
[Keil Device Family Packs].

[svd2rust]: https://github.com/japaric/svd2rust
[Keil Device Family Packs]: https://www.keil.com/dd2/pack

## Usage

The `rt` feature is optional and brings in support for `cortex-m-rt`.

In your code:

```rust
use k22f;

let peripherals = k22f::Peripherals::take().unwrap();

// PortA clock enable
let sim = &peripherals.SIM;
sim.scgc5.write(|w| w.porta().set_bit());

// Set GPIO output
let porta = &peripherals.PORTA;
porta.pcr2.write(|w| w.mux()._001());

// Set green LED low
let gpioa = &peripherals.GPIOA;
gpioa.pddr.write(|w| w.pdd()._0());
```

For full details on the autogenerated API, please see:
https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api
