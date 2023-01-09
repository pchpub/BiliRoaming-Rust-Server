# 旧版播放链接API相关
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
