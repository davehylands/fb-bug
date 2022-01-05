
all: app/src/fb_generated.rs
	cargo run --manifest-path app/Cargo.toml

app/src/fb_generated.rs: fb.fbs
	flatc --gen-mutable --rust -o app/src $<
