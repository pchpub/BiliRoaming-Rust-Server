# 错误码定义

## 官方 BiliUpstreamError

+ 0 正常

### 权限类

+ -1 应用程序不存在或已被封禁 `ApiFatal`
+ -2 AccessKey错误 `LoginInvalid`
+ -3 API校验密匙错误 `ApiSignInvalid`
+ -4 调用方对该Method没有权限
+ -101 用户未登录 `LoginInvalid`
+ -102 账号被封停 `LoginInvalid`
+ 61000 使用登录状态访问了，并且登录状态无效，客服端可以／需要删除登录状态 `LoginInvalid`

### 请求类

+ -400 设备限制? 也发现appkey异常时也是此错误码 `ReqInvalid`
+ -401 请求未授权 `ReqInvalid`
+ -403 访问权限不足 `ReqAccessDenied`
+ -404 `ReqNotFound`
+ -412 用户异常请求 `ReqFatal`
+ -500 服务器错误 `ServerInternal`
+ -503 过载保护,服务暂不可用 `ServerOverload`
+ -504 服务调用超时 `ServerTimeout`
+ -663
  + appkey不对应, message = `鉴权失败，请联系账号组` `ReqAppkeyInvalid`
  + api已经被弃用, message = `-663` `ReqApiDeprecated`
+ -1200 被降级过滤的请求 `ReqCrawlerLimit`


### 资源类

+ -10500 DRM限制用户地区 `ResDrmLimit`
+ 10015002 东南亚区大会员专享限制 `ResVipOnly`
+ -10403
  + 大会员限制, message = `大会员专享限制` `ResVipOnly`
  + 平台限制, message = `抱歉您所使用的平台不可观看！` `ResPlatformLimit`
  + 地区限制, message = `抱歉您所在地区不可观看！` `ResAreaLimit`
+ 6002105
  + 大会员限制, message = `开通大会员观看` `ResVipOnly`
+ 6002003
  + 地区限制, message = `抱歉您所在地区不可观看！` `ResAreaLimit`
+ 6010001 地区限制 `ResAreaLimit`

## 服务器自定义

+ -101 无法获取用户mid(uid)
+ -10403 黑/白名单限制
