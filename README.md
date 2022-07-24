# BiliRoaming-Rust-Server
## [Features]

* /pgc/player/api/playurl?
* /pgc/player/web/playurl?
* /intl/gateway/v2/ogv/playurl?
* read the cfg from json
* /intl/gateway/v2/app/search/type?
* /x/v2/search/type?
* /x/web-interface/search/type?
* /intl/gateway/v2/ogv/view/app/season?
* local black&white list

## [TODO]

* /intl/gateway/v2/app/search/v2?
* /intl/gateway/v2/app/subtitle?
* /intl/gateway/v2/ogv/view/app/season2?
* /intl/gateway/v2/ogv/view/app/episode?
* /pgc/view/web/season?
* /x/intl/passport-login/oauth2/refresh_token?

## [使用说明]

### 1. 使用已编译的二进制文件
* 从[Release](https://github.com/pchpub/BiliRoaming-Rust-Server/releases)中下载二进制文件及 config.json
* 安装 Redis
  * 使用宝塔安装 Redis
  * `apt install redis #Ubuntu&Debian`
  * `yum install redis #CentOS`
* 填写 config.json
*  `./biliroaming_rust_server` 启动服务端
* 使用 Nginx 反代 `http://127.0.0.1:2662`   (这个port在config.json里)
* Enjoy~

### 2. 自行编译二进制文件
*  `git clone https://github.com/pchpub/BiliRoaming-Rust-Server.git` 下载源代码
* 安装 Cargo
  * `apt install cargo #Ubuntu&Debian`
  * `yum install cargo #CentOS`
* `cd BiliRoaming-Rust-Server` 进入源代码目录下
* `cargo build --profile=fast` 编译二进制文件
* 安装 Redis
  * 使用宝塔安装 Redis
  * `apt install redis #Ubuntu&Debian`
  * `yum install redis #CentOS`
* `cp config.example.json config.json` 复制配置文件并重命名
* 将编译好的二进制文件移动至与 config.json 文件相同目录下
* 填写 config.json
* 使用 `./biliroaming_rust_server` 启动服务端
* 使用 Nginx 反代 `http://127.0.0.1:2662`   (这个port在config.json里)
* Enjoy~