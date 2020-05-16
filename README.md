# Literumilo - A spell checker and morphological analyzer for Esperanto, written in Rust.

Literumilo checks the spelling of Esperanto words, and divides them into morphemes.
For example, 'miskomprenita' is divided as 'mis.kompren.it.a'.

Literumilo can analyze individual Esperanto words, or an entire file of Esperanto text.

## Requirements

[Install Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## Build

Clone this project. In a terminal window, run:

```
git clone https://github.com/Indrikoterio/literumilo-rust
```

Move to the project's folder.

```
cd literumilo-rust
```

Assuming your PC has Rust and Cargo installed, you can build the project by  running:

```
cargo build --release
```

The executable (literumilo) will be written to `literumilo-rust/target/release`.

## Usage

To list misspelled words from a file, open a terminal window and run the following command.

```
./literumilo file.txt
```

To divide words from a file into morphemes, add an -m option to the command.

```
./literumilo -m file.txt
```

To check the spelling of a single word run:

```
./literumilo ĉiutage
```

Accents can be represented by 'x'.

```
./literumilo cxiutage
```

## Developer

Literumilo was developed by Cleve (Klivo) Lendon.

## Contact

To contact the developer, send email to indriko@yahoo.com . Preferred languages are English and Esperanto. Comments, suggestions and criticism are welcomed.

## History

First release, May 2020.

## License

Literumilo is free software. It is distributed free of charge, without conditions, and without guarantees. You may use, modify and distribute it as you wish. There is no need to ask for permission. If you use Literumilo's code in your own project, and publish it, I request only that you acknowledge the source.
