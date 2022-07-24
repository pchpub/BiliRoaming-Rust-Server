#!/bin/bash
source /etc/os-release
case $ID in
debian|ubuntu|devuan)
    sudo apt-get install git
    sudo apt-get install cargo
    sudo apt-get install screen
    sudo apt-get install redis
    ;;
centos|fedora|rhel)
    sudo yum install git
    sudo yum install cargo
    sudo yum install screen
    sudo yum install redis
    ;;
*)
    exit 1
    ;;
esac
git clone https://github.com/pchpub/BiliRoaming-Rust-Server
cd BiliRoaming-Rust-Server
cargo build --profile=fast
mkdir /root/rust
cp ./target/fast/biliroaming_rust_server /root/rust/biliroaming_rust_server
cp ./config.example.json /root/rust/config.json
sudo chmod 777 /root/rust/biliroaming_rust_server
sudo chmod 777 /root/rust/config.json
cd /root/rust/
echo "请去按实际情况修改/root/rust/config.json 修改好再来"
read -p  "修改好了后按下任意键"
screen -dmS "biliroaming_rust_server" ./biliroaming_rust_server
echo "请反代到127.0.0.1:2662(这个端口就是config中的port,默认为2662)"
