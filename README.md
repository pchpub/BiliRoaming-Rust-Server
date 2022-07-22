## BiliRoaming-Rust-Server
* [Features]
    * /pgc/player/api/playurl?
    * /pgc/player/web/playurl?
    * /intl/gateway/v2/ogv/playurl?
    * read the cfg from json
    * /intl/gateway/v2/app/search/type?
    * /x/v2/search/type?
    * /x/web-interface/search/type?
    * /intl/gateway/v2/ogv/view/app/season?
    * local black&white list
* [TODO]
    * /intl/gateway/v2/app/search/v2?
    * /intl/gateway/v2/app/subtitle?
    * /intl/gateway/v2/ogv/view/app/season2?
    * /intl/gateway/v2/ogv/view/app/episode?
    * /pgc/view/web/season?
    * /x/intl/passport-login/oauth2/refresh_token?
* [使用说明]
    * 安装 redis
    * 填写 config.json
    * 启动
    * 反代到 http://127.0.0.1:2662   (这个port在config.json里)
    * 完事
