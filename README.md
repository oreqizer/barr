# `barr`

Barrel your **JavaScript** files! :oil_drum:

- `cargo install barr`

## Usage

- `barr [OPTIONS] [FILES...]`

To barrel `src/file.js`, run `barr src/file.js`. This will create:

```
src/
  file/
    file.js
    index.js
```

The `index.js` file is a simple barrel that looks like this:

```js
export { default } from "./file";
```

Flags:
- `--extension <ext>` or `-e <ext>` to change the barrel file's extension
- `--help` or `-h` to print usage information
- `--version` or `-v` to print version

## License

MIT
