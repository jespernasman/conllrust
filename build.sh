cargo build --release
mv target/release/libconllrust.dylib conllrust.so
python test.py
