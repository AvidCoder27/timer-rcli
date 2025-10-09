echo "Building in release mode..."
cargo build --release

echo "Removing old version"
sudo rm /usr/bin/timer

echo "Copying new version"
sudo cp ./target/release/timer-rust /usr/bin/timer

echo "Successfully installed timer to /usr/bin/timer!"
