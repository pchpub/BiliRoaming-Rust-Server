+ 查询数据+地区（1位）+类型（2位）+版本（2位）
+ 查询数据 
    + a access_key
    + u uid/mid
    + e epid
    + c cid
    + v is_vip
    + t is_tv
+ 地区 cn 1
    + hk 2
    + tw 3
    + th 4
    + default 2
+ 类型 app playurl 01
    + app search 02
    + app subtitle 03
    + app season 04
    + user_info 05
    + user_cerinfo 06
    + web playurl 07
    + web search 08
    + web subtitle 09
    + web season 10
    + resign_info 11
    + api 12
    + health 13 eg. 0141301 = playurl th health ver.1
      + health_check_used_access_key: a1301
    + ep_area 14
    + ep_info 15
      + need_vip 01
      + title 02 // not used
      + season_id 03 not_used
+ 版本 ：用于处理版本更新后导致的格式变更
    + now 01