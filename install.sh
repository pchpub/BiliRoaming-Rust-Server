#!/bin/bash
source /etc/os-release
case $ID in
debian|ubuntu|devuan)
    sudo apt update && sudo apt upgrade -y
    sudo apt-get install git cargo screen redis -y
    ;;
centos|fedora|rhel)
    sudo yum update -y
    sudo yum install git cargo screen redis -y
    ;;
*)
    exit 1
    ;;
esac
git clone https://github.com/pchpub/BiliRoaming-Rust-Server
cd BiliRoaming-Rust-Server
cargo build --profile=fast
sudo mkdir /opt/BiliRoaming-Rust-Server
sudo cp ./config.example.json /opt/BiliRoaming-Rust-Server/config.json
sudo cp ./target/fast/biliroaming_rust_server /opt/BiliRoaming-Rust-Server
sudo chmod +x /opt/BiliRoaming-Rust-Server/biliroaming_rust_server
echo "请按实际情况修改 config.json"
read -p  "修改好了后按下任意键"
echo "请反代到127.0.0.1:2662(这个端口就是config中的port,默认为2662)"
cat <<'TEXT' > /etc/systemd/system/biliroaming_rust_server.service
[Unit]
Description=Biliroaming Rust Server
After=network.target

[Install]
WantedBy=multi-user.target

[Service]
Type=simple
WorkingDirectory=/opt/BiliRoaming-Rust-Server
ExecStart=/opt/BiliRoaming-Rust-Server/biliroaming_rust_server
Restart=always
ExecStop=/usr/bin/kill -2 $MAINPID
StandardOutput=file:/opt/BiliRoaming-Rust-Server/biliroaming_rust_server.log
TEXT
systemctl enable biliroaming_rust_server.service
systemctl start biliroaming_rust_server.service
