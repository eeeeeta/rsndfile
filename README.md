rsndfile, `libsndfile` bindings for Rust
========================================
*currently very alpha*

This is a crate to provide basic [libsndfile](http://www.mega-nerd.com/libsndfile/) bindings
for Rust. Currently, it's not very developed and is only useful for extremely basic playback.
It works, though. Sort of.

### Warning: this crate may be terribly unsafe

I don't think it does anything bad, but if your program blows up because you used this crappy alpha
code, I will laugh at you.

## Usage

```rust
extern crate rsndfile;
use rsndfile::SndFile;

fn main() {
    let sf = SndFile::open("file.aiff").unwrap();
    let mut buf = [f32; 500] = [0.0; 500];
    
    sf.into_slice_float(&mut buf, 500);
}
```

## Docs

    $ cargo doc