# foget

A simple program to remind one about other programs. Could also be used as a quick terminal note taking app.
Also, written in Rust so it's blazing fast 🔥🔥🔥🚀

## How to use

Just simply build with Cargo: `cargo build --release`.

You may want to copy one base toml configuration file from the `descriptions` folder to either your home directory or `$home/.config/foget` directory.
You may use the `FOGET_DESCRIPTIONS` environment variable to define the path to the descriptions file to use.
You may just use the `--descriptions` option and manually add the path to your descriptions file.

Finally just call the executable from the `foget/target/release/` directory.

## Contributing

I am open for any feature or pull requests. I only ask to use the default rust format provided by the rust-analyzer and to please be patient with me.

## License

The project uses an MIT license because I can't be bothered to deal with more complicated ones or to think about it for long.
