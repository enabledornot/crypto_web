# crypto_web

A live demo is available on Github pages [here](https://enabledornot.github.io/crypto_web/)

## Project Descrption
This contains the source code for the `crypto_web` web site which contains 3 primality tests. Primality tests determine whether a given number is prime and is used as the basis for many cryptography algorithms. The fermat and miller rabin primality tests are probabilistic in nature however when ran for enough inputs they are highly likely to produce a correct result. AKS is the only deterministic algorithm which works in poly time. Hense it is far more complicated than the other two and typically not implemented.

## Implementation Details
The website itself is written in HTML with flask. Flask freezer is also used to generate a static web page. Rust is also used to compile to webassembly in order to produce a fully static website.

## Extra Resources
Wikipedia does a great job at describing these tests in detail so I suggest you take a look for further info. You could also look at the code located in `src/`.
 - [Fermat](https://en.wikipedia.org/wiki/Fermat_primality_test)
 - [Miller Rabin](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test)
 - [AKS](https://en.wikipedia.org/wiki/AKS_primality_test)

## Building
Building instructions are available [documentation.md](./documentation.md)