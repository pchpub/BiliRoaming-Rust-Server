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
* /x/intl/passport-login/oauth2/refresh_token?
* /intl/gateway/v2/app/subtitle?
* local black&white list
* check sign
* support config.json & config.yaml
* support http,https,socks5.. proxy 
* remove need_vip & need_login
* support auto_update
* 向后兼容

## [TODO] 

* ~~/intl/gateway/v2/app/search/v2?~~ web脚本已弃用
* ~~/intl/gateway/v2/ogv/view/app/season2?~~ web脚本已弃用
* ~~/intl/gateway/v2/ogv/view/app/episode?~~ web脚本已弃用
* ~~/pgc/view/web/season?~~ web脚本已弃用
* to be faster

## [使用说明]

### 1. 使用已编译的二进制文件
* 下载二进制文件(使用Action编译的较新)
  * 从[Release](https://github.com/pchpub/BiliRoaming-Rust-Server/releases)中下载二进制文件及 config.json
  * 从[Action](https://github.com/pchpub/BiliRoaming-Rust-Server/actions/workflows/ci.yml)中下载二进制文件,从仓库中下载config.json
* 安装 Redis
  * 使用宝塔安装 Redis
  * `apt install redis` #Ubuntu&Debian
  * `yum install redis` #CentOS
* 填写 config.json
*  `./biliroaming_rust_server` 启动服务端
* 使用 Nginx 反代 `http://127.0.0.1:2662` (端口号可在 config.json 中修改)
* Enjoy~

### 2. 自行编译二进制文件
*  `git clone https://github.com/pchpub/BiliRoaming-Rust-Server.git` 下载源代码
* 安装 Cargo
  * `apt install cargo` #Ubuntu&Debian
  * `yum install cargo` #CentOS
* `cd BiliRoaming-Rust-Server` 进入源代码目录下
* `cargo build --profile=fast` 编译二进制文件
* 安装 Redis
  * 使用宝塔安装 Redis
  * `apt install redis` #Ubuntu&Debian
  * `yum install redis` #CentOS
* `cp config.example.json config.json` 复制配置文件并重命名
* `cp target/fast/biliroaming_rust_server biliroaming_rust_server`将编译好的二进制文件复制至项目根目录
* 填写 config.json
* 使用 `./biliroaming_rust_server` 启动服务端
* 使用 Nginx 反代 `http://127.0.0.1:2662` (端口号可在 config.json 中修改)
* Enjoy~

### 3. 使用一键安装脚本
*  `curl -sSO https://raw.githubusercontent.com/pchpub/BiliRoaming-Rust-Server/main/install.sh && sudo bash install.sh` 
* 按提示操作 默认选y（yes）
* 使用 Nginx 反代 `http://127.0.0.1:2662` (端口号可在 config.json 中修改)
* Enjoy~
## [温馨提示]
* config中code为0时的缓存请勿设置为1.9h以下(低于30min会导致缓存就和没缓存一样) 建议保持默认
* 非常不建议将缓存时间设为0（永久缓存）,可能会导致后续错误无法自动恢复
* 目前服务端只是小范围测试,有已知但未修复的严重bug,可能会导致您的机子更容易-412
