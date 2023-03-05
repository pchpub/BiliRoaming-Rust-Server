# API 开发参考

## 旧版播放链接API相关

+ 适用范围
  + 粉版APP`5.10.0`-`5.36.0`
    + 自`5.37.0`开始, 客户端并没有使用传统方法, 直接使用了gRPC. `6.80.0`及以后直接删除了相关代码
+ UA
  + `Bilibili Freedoooooom/MarkII`
+ 参数(解包6.79.0得到的)
  + `access_key`
  + `ep_id`
  + `cid`
  + `qn`
  + `appkey`
  + `platform` // 和mobi_app一个东西
  + `mobi_app`
  + `build`
  + `buvid`
  + `device`
  + `model` // ?
  + `fnver` // 1
  + `fnval` // 464
  + `fourk` // 4K才需要此项, 默认不需要此项
  + `session` // 遵循客户端设置, 否则不需要此项
  + `force_host` // 遵循客户端设置, 留空设为2, 即仅https
  + `is_preview` // 不需要此项
  + `dl` // 是否为下载, 不是则不需要
  + `track_path` // 似乎是免流相关, 不需要此项
  + `sign` // 算法已经明确
+ 参数(5.36.0) *列出独有的*
  + `otype` //此项总是`json`, 似乎不需要处理
  + `buvid` // XY prefix
  + `mid` 用户mid
  + `expire` 不知道干嘛的
  + `npcybs` 不知道干嘛的, 似乎下载是1播放是0
  + `module` // 为"bangumi"
  + `track_path` // 不知道干嘛的, 默认0即可
  + `unicom_free` // 推测为免流相关
  + `season_type` // 看不出来是什么, 不是必要的
+ **注意**
  + web端的API实在没找到出处

## 客户端信息相关

+ platform_id : android => 3
+ BUVID相关: 均为**首次启动**且**全新设备**的BUVID Prefix
+ 关键API请求Params
  + `app.bilibili.com/x/v2/account/myinfo`
    + Params
      + access_key `{access_key}`
      + appkey `{appkey_auth}` **auth related**
      + build `{app_build}`
      + buvid `{buvid_auth}`
      + c_locale `zh-Hans_CN`
      + channel `master`
      + disable_rcmd `0`
      + local_id `{buvid_auth}`
      + mobi_app `{mobi_app}`
      + platform `{platform}`
      + s_locale `zh-Hans_CN`
      + statistics \{"appId":{(int) app_id},"platform":3,"version":"{app_ver}","abtest":""\}
      + ts
      + sign
    + Headers
      + buvid `{buvid_auth}`
      + env `prod`
      + app-key `android-64` // ???
      + user-agent `Mozilla/5.0 BiliDroid/{版本号} (bbcallen@gmail.com) os/android model/{手机型号} mobi_app/{mobi_app} build/{app_build} channel/master innerVer/{app_inner_ver} osVer/{os_ver} network/2`
      + x-bili-trace-id `***`
      + x-bili-aurora-eid `-` => 留空, 除非已知mid
      + x-bili-aurora-zone `-` => 留空
      + bili-http-engine `cronet`
  + gRPC Related Headers **exclude grpc.biliapi.net!!!** **app.bilibili.com**
    + env `prod`
    + app-key `{appkey_name}`
    + user-agent `Dalvik/2.1.0 (Linux; U; Android {os_ver}; {device_model} Build/{???随机生成算了}) {app_ver} os/android model/{model} mobi_app/{mobi_app} build/{app_build} channel/master innerVer/{app_inner_ver} osVer/{os_ver} network/2`
    + x-bili-trace-id `***`
    + x-bili-aurora-eid `***`
    + x-bili-mid `uid`
    + x-bili-aurora-zone `-` => 留空
    + x-bili-metadata-bin `***`
    + authorization `identify_v1 {access_key}`
    + x-bili-device-bin `***`
    + x-bili-network-bin `CAE`
    + x-bili-restriction-bin  `-` => 留空
    + x-bili-locale-bin `Cg4KAnpoEgRIYW5zGgJDThIOCgJ6aBIESGFucxoCQ04`
    + x-bili-exps-bin `CgIIAQ`
    + buvid `{buvid}`
    + x-bili-fawkes-req-bin `CglhbmRyb2lkX2kSBHByb2QaCDFjYzI4MGE0`
    + bili-http-engine `cronet`
+ 可用的
  + 国内版(粉版) `Android` app_ver => `7.8.2` app_build => `7082100` app_inner_ver => `7082110`
    + appid: `1`
    + appkey: `783bbb7264451d82` **for auth related only, app_build >= 7082100**
    + appsec: `2653583c8873dea268ab9386918b1d65` **for auth related only, app_build >= 7082100**
    + appkey: `1d8b6e7d45233436`
    + appsec: `560c52ccd288fed045859ed18bffd973`
    + mobi_app: `android`
      <!-- platform will not be ios, default android -->
    + platform: `android`
      <!--  -->
    + buvid_auth: `XX`
    + buvid: `XU`

  + 国际版 `AndroidI` `3.16.0` `70` `7082100`
    + app_id: `14`
    + appkey: `ae57252b0c09105d` **for auth related only, app_build >= 7082100**
    + appsec: `c75875c596a69eb55bd119e74b07cfe3` **for auth related only, app_build >= 7082100**
    + appkey: `bb3101000e232e27` **universial**
    + appsec: `36efcfed79309338ced0380abd824ac1` **universial**
    + mobi_app: `android_i`
    + platform: `android`
    + buvid_auth: `XU`
    + buvid: `XU`
  + 概念版(蓝版) `AndroidB` `7.11.0` `7110300`
    + app_id: `3`
    + appkey: `191c3b6b975af184` **for auth related only, app_build >= 7110300 ?**
    + appsec: `` **for auth related only, app_build >= 7110300 ?**
    + appkey: `07da50c9a0bf829f`
    + appsec: `25bdede4e1581c836cab73a48790ca6e`
    + mobi_app: `android_b`
    + platform: `android`
    + buvid_auth: `XU`
    + buvid: `XU`
  + HD版 `AndroidHD` v`1.36.0` b`1360001`
    + app_id: `5`
    + appkey: `dfca71928277209b`
    + appsec: `b5475a8825547a4fc26c7d518eaaa02e`
    + mobi_app: `android_hd`
    + platform: `android`
    + buvid_auth: `XU`
    + buvid: `XU`
  + TV版 `AndroidTV` **不维护**
    + app_id: `9` ???
    + appkey: `4409e2ce8ffd12b8`
    + appsec: `59b43e04ad6965f34319062b478f83dd`
    + mobi_app: `android_tv_yst`
    + platform: `android`
  + 东南亚版 `BstarA` **for playurl only**
    + appkey: `7d089525d3611b1c`
    + appsec: `acd495b248ec528c2eed1e862d393126`
    + buvid_auth: `XG`
    + buvid: `XG`
  + 第三方授权(油猴脚本) `Web` **like ios**
    + appkey: `27eb53fc9058f8c3`
    + appsec: `c2ed53a74eeefe3cf99fbd01d8c9c375`
+ 其他版本(不维护可用性)
  + Ai4cCreatorAndroid
    + appkey: `9d5889cf67e615cd`
    + appsec: `8fd9bb32efea8cef801fd895bef2713d`
  + AndroidMallTicket
    + appkey: `4c6e1021617d40d9`
    + appsec: `e559a59044eb2701b7a8628c86aa12ae`
  + AndroidOttSdk
    + appkey: `c034e8b74130a886`
    + appsec: `e4e8966b1e71847dc4a3830f2d078523`
  + AndroidBiliThings
    + appkey: `8d23902c1688a798`
    + appsec: `710f0212e62bd499b8d3ac6e1db9302a`
  + AnguAndroid
    + appkey: `50e1328c6a1075a1`
    + appsec: `4d35e3dea073433cd24dd14b503d242e`
  + BiliLink
    + appkey: `37207f2beaebf8d7`
    + appsec: `e988e794d4d4b6dd43bc0e89d6e90c43`
  + BiliScan
    + appkey: `9a75abf7de2d8947`
    + appsec: `35ca1c82be6c2c242ecc04d88c735f31`
  + Iphone
    + appkey: `4ebafd7c4951b366`
    + appsec: `8cb98205e9b2ad3669aad0fce12a4c13`

## BUVID相关

+ APP端生成逻辑
  + 国内版(粉版) `Android` app_ver => `7.8.2` app_build => `7082100` app_inner_ver => `7082110`

```rust
#[inline]
pub fn gen_buvid(client_info: &ClientInfo) -> String {
    // 逆向工程: com.bilibili.lib.buvid.internal
    // 确定: BUVID在首次启动时生成, 然后客户端向云端请求是否有匹配的BUVID, 有就使用云端的, 否则使用本地生成的. BUVID和drmId, AndroidId绑定, 不过可以随机生成.
    // 升级APP似乎不会改变BUVID, 优先使用local_buvid
    /*
        androidId ***
        appkey 1d8b6e7d45233436
        brand google
        build 10729724 //  FoundationAlias.getFapps().getFawkesBuildSN();
        buvid *** // 本地生成
        channel master
        drmId *** // 非常重要, 32位, 安卓9以后主要看这个?
        fawkesAppKey android64
        first 1
        firstStart 1
        imei
        internalVersionCode 7082110
        mac //wifi mac
        model Pixel 2 XL
        neuronAppId 1 // app_id, 产品编号，由数据平台分配，粉=1，白=2，蓝=3，直播姬=4，HD=5，海外=6，OTT=7，漫画=8，TV野版=9，小视频=10，网易漫画=11，网易漫画lite=12，网易漫画HD=13,国际版=14
        neuronPlatformId 3 // android: 3
        oaid
        ts 1672157861
        versionCode 7082100
        versionName 7.8.2
        sign e79d7f36b62734ef6b6ac0a3f73d3ad4
    */

    // input_str, 可以是drmId, androidId, wifi mac, etc
    let input_str = { "***" }; // 输入drmId. 这个值应该可以随机生成, 反正没见过drm

    // 计算生成input_str的md5
    let c2 = {
        let mut md5_instance = crypto::md5::Md5::new();
        md5_instance.input_str(input_str);
        md5_instance.result_str().to_ascii_uppercase()
    };
    //从md5的结果抽取第2, 12, 22位, 失败就是000
    // rust没有try catch错误处理机制?
    let d2 = {
        let steps = || -> Option<String> {
            let c2_bytes = c2.as_bytes();
            if c2_bytes.len() < 22 {
                return None;
            }
            let d2_bytes = vec![c2_bytes[2], c2_bytes[12], c2_bytes[22]];
            if let Ok(value) = String::from_utf8(d2_bytes.to_vec()) {
                Some(value)
            } else {
                None
            }
        };
        steps().unwrap_or(String::from("000"))
    };
    // prefix, drmId均为XU. androidid为XX, wifi mac为XY
    let buvid_prefix = "XU";
    let mut final_str_vec = vec![];
    // 按format!("{buvid_prefix}{&d2}{&c2}")连接起来, 共37位, 2+3+32
    final_str_vec.push(buvid_prefix);
    final_str_vec.push(&d2);
    final_str_vec.push(&c2);
    final_str_vec.join("")
}
```

## 东南亚 `2.23.0` `2230100` `2230100` appkey_name -> `bstar_a2022`

+ UA
  + `Mozilla/5.0 BiliDroid/{app_ver} (bbcallen@gmail.com) os/android model/{device_model} mobi_app/{mobi_app} build/{app_build} channel/{app_channel} innerVer/{app_build_inner} osVer/{os_ver} network/2`
+ 请求头
  + `app-key`: `{appkey_name}`
  + `env`: `prod`
  + `bili-bridge-engine`: `cronet`
  + `buvid` `XG` prefix
+ playurl
  + `aid`: `0`
  + `appkey`: `7d089525d3611b1c`
  + `build`: `2230100`
  + `c_locale`: `-` 留空
  + `channel`: `master`
  + `ep_id`: `{ep_id}`
  + `fnval`: `464`
  + `fnver`: `0`
  + `force_host`: `2`
  + `fourk`: `true`
  + `lang` `-` 留空
  + `locale`: `en_IN`
  + `market`: `google`
  + `mobi_app`: `bstar_a`
  + `net_type`: `1`
  + `platform`: `android`
  + `prefer_code_type`: `1`
  + `qn`: `112`
  + `s_locale`: `en_IN`
  + `sim_code` `-` 留空
  + `statistics`: `{{"appId":30,"platform":3,"version":"2.23.0","abtest":""}}`
  + `timezone`: `GMT+08:00`
  + `ts`: `{ts}`
  + `user_qn`: `112`
  + `sign`: `{sign}`
+ search
  + `appkey`: `7d089525d3611b1c`
  + `build`: `2230100`
  + `c_locale`: `-` 留空
  + `channel`: `master`
  + `fnval`: `16`
  + `fnver`: `0`
  + `fourk`: `1`
  + `highlight`: `1`
  + `keyword`: `{keyword}`
  + `lang`: ``
  + `locale`: `en_IN`
  + `market`: `google`
  + `mobi_app`: `bstar_a`
  + `net_type`: `1`
  + `platform`: `android`
  + `pn`: `1`
  + `prefer_code_type`: `1`
  + `ps`: `20`
  + `qid`: `25509608210040832`
  + `s_locale`: `en_IN`
  + `sim_code`: `-` 留空
  + `statistics`: `{"appId":30,"platform":3,"version":"2.23.0","abtest":""}`
  + `timezone`: `GMT+08:00`
  + `ts`: `{ts(sec)}`
  + `type`: `7`
  + `user_qn`: `0`
  + `sign`: `{sign}`
+ season

## BUVID生成逻辑

+ XU drmId
+ XX androidId
+ XW auto generated uuid
+ XY wifi mac
+ XF FBID 即Facebook id
+ XG GAID 即Google Advertising ID

## 本地搜索(TODO)

+ 本地分词可参考<https://github.com/messense/jieba-rs>
+ Bilibili每次搜索都会返回分词结果, 可以参考

## 本地番剧插入/替换(TODO)

+ ### 插入索引

  + 已有的番剧, 采用原有的`ep_id`, 但是区分`season_id`
  + 没有的番剧, 为了区分已有的, 自编`season_id`, 范围`2_000_000` - `2_999_999`, 自编`ep_id`则为`{season_id}{ep_no}`, `ep_no`范围为`001` - `999`
