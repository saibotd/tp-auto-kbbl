#!/bin/bash
set -e

echo "
[1/6] Bulding release"
cargo build --release
echo "
[2/6] Stopping service"
sudo systemctl stop tp-auto-kbbl.service
echo "
[3/6]Copying binary"
sudo cp -v ./target/release/tp-auto-kbbl /usr/bin/
echo "
[4/6]Copying service unit"
sudo cp -v tp-auto-kbbl.service /etc/systemd/system/
echo "
[5/6] Reloading unit files"
sudo systemctl daemon-reload
echo "
[6/6] Starting service"
sudo systemctl enable tp-auto-kbbl.service
sudo systemctl start tp-auto-kbbl.service
