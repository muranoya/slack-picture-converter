# slack-picture-converter

# Requires
- ImageMagick
- Rust

# Build
1. cd slack-picture-converter
2. pushd rsc && ./generator.sh
3. popd && cargo build --release

# How to use
1. Register rsc/\*.png to slack custom emoji.
2. Execute `./target/release/slack-picture-converter <path to image file> <tile size>`
3. Output copy and post to slack!
