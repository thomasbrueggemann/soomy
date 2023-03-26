rm -R dist
trunk build --release --public-url soomy
rm -R docs
mv dist docs