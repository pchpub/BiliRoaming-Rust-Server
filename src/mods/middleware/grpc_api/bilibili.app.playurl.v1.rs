///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ab {
    ///
    #[prost(message, optional, tag = "1")]
    pub glance: ::core::option::Option<Glance>,
    ///
    #[prost(int32, tag = "2")]
    pub group: i32,
}
/// 配置项
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArcConf {
    /// 是否支持
    #[prost(bool, tag = "1")]
    pub is_support: bool,
    ///
    #[prost(bool, tag = "2")]
    pub disabled: bool,
    ///
    #[prost(message, optional, tag = "3")]
    pub extra_content: ::core::option::Option<ExtraContent>,
}
/// Chronos灰度管理
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Chronos {
    /// 资源md5
    #[prost(string, tag = "1")]
    pub md5: ::prost::alloc::string::String,
    /// 资源文件
    #[prost(string, tag = "2")]
    pub file: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ButtonStyle {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub text_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub bg_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub jump_link: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudConf {
    /// 是否展示功能
    #[prost(bool, tag = "1")]
    pub show: bool,
    /// 设置类型
    #[prost(enumeration = "ConfType", tag = "2")]
    pub conf_type: i32,
    ///
    #[prost(message, optional, tag = "3")]
    pub field_value: ::core::option::Option<FieldValue>,
    ///
    #[prost(message, optional, tag = "4")]
    pub conf_value: ::core::option::Option<ConfValue>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfValue {
    #[prost(oneof = "conf_value::Value", tags = "1, 2")]
    pub value: ::core::option::Option<conf_value::Value>,
}
/// Nested message and enum types in `ConfValue`.
pub mod conf_value {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        ///
        #[prost(bool, tag = "1")]
        SwitchVal(bool),
        ///
        #[prost(int64, tag = "2")]
        SelectedVal(i64),
    }
}
/// dash条目
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DashItem {
    /// 清晰度
    #[prost(uint32, tag = "1")]
    pub id: u32,
    /// 主线流
    #[prost(string, tag = "2")]
    pub base_url: ::prost::alloc::string::String,
    /// 备用流
    #[prost(string, repeated, tag = "3")]
    pub backup_url: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// 带宽
    #[prost(uint32, tag = "4")]
    pub bandwidth: u32,
    /// 编码id
    #[prost(uint32, tag = "5")]
    pub codecid: u32,
    /// md5
    #[prost(string, tag = "6")]
    pub md5: ::prost::alloc::string::String,
    /// 大小
    #[prost(uint64, tag = "7")]
    pub size: u64,
    /// 帧率
    #[prost(string, tag = "8")]
    pub frame_rate: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub widevine_pssh: ::prost::alloc::string::String,
}
/// dash视频流
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DashVideo {
    /// 主线流
    #[prost(string, tag = "1")]
    pub base_url: ::prost::alloc::string::String,
    /// 备用流
    #[prost(string, repeated, tag = "2")]
    pub backup_url: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// 带宽
    #[prost(uint32, tag = "3")]
    pub bandwidth: u32,
    /// 编码id
    #[prost(uint32, tag = "4")]
    pub codecid: u32,
    /// md5
    #[prost(string, tag = "5")]
    pub md5: ::prost::alloc::string::String,
    /// 大小
    #[prost(uint64, tag = "6")]
    pub size: u64,
    /// 伴音质量id
    #[prost(uint32, tag = "7")]
    pub audio_id: u32,
    /// 是否非全二压
    #[prost(bool, tag = "8")]
    pub no_rexcode: bool,
    /// 码率
    #[prost(string, tag = "9")]
    pub frame_rate: ::prost::alloc::string::String,
    /// 宽度
    #[prost(int32, tag = "10")]
    pub width: i32,
    /// 高度
    #[prost(int32, tag = "11")]
    pub height: i32,
    ///
    #[prost(string, tag = "12")]
    pub widevine_pssh: ::prost::alloc::string::String,
}
/// 杜比伴音信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DolbyItem {
    /// 杜比类型
    #[prost(enumeration = "dolby_item::Type", tag = "1")]
    pub r#type: i32,
    /// 音频流
    #[prost(message, optional, tag = "2")]
    pub audio: ::core::option::Option<DashItem>,
}
/// Nested message and enum types in `DolbyItem`.
pub mod dolby_item {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Type {
        /// NONE
        None = 0,
        /// 普通杜比音效
        Common = 1,
        /// 全景杜比音效
        Atmos = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::None => "NONE",
                Type::Common => "COMMON",
                Type::Atmos => "ATMOS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NONE" => Some(Self::None),
                "COMMON" => Some(Self::Common),
                "ATMOS" => Some(Self::Atmos),
                _ => None,
            }
        }
    }
}
/// 事件
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    /// 震动
    #[prost(message, optional, tag = "1")]
    pub shake: ::core::option::Option<Shake>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtraContent {
    ///
    #[prost(string, tag = "1")]
    pub disabled_reason: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "2")]
    pub disabled_code: i64,
}
/// 配置字段值
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldValue {
    #[prost(oneof = "field_value::Value", tags = "1")]
    pub value: ::core::option::Option<field_value::Value>,
}
/// Nested message and enum types in `FieldValue`.
pub mod field_value {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// 开关
        #[prost(bool, tag = "1")]
        Switch(bool),
    }
}
/// 清晰度描述
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FormatDescription {
    /// 清晰度
    #[prost(int32, tag = "1")]
    pub quality: i32,
    /// 清晰度格式
    #[prost(string, tag = "2")]
    pub format: ::prost::alloc::string::String,
    /// 清晰度描述
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// 新描述
    #[prost(string, tag = "4")]
    pub new_description: ::prost::alloc::string::String,
    /// 选中态的清晰度描述
    #[prost(string, tag = "5")]
    pub display_desc: ::prost::alloc::string::String,
    /// 选中态的清晰度描述的角标
    #[prost(string, tag = "6")]
    pub superscript: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Glance {
    ///
    #[prost(bool, tag = "1")]
    pub can_watch: bool,
    ///
    #[prost(int64, tag = "2")]
    pub times: i64,
    ///
    #[prost(int64, tag = "3")]
    pub duration: i64,
}
/// 禁用功能配置
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayAbilityConf {
    /// 后台播放
    #[prost(message, optional, tag = "1")]
    pub background_play_conf: ::core::option::Option<CloudConf>,
    /// 镜像反转
    #[prost(message, optional, tag = "2")]
    pub flip_conf: ::core::option::Option<CloudConf>,
    /// 投屏
    #[prost(message, optional, tag = "3")]
    pub cast_conf: ::core::option::Option<CloudConf>,
    /// 反馈
    #[prost(message, optional, tag = "4")]
    pub feedback_conf: ::core::option::Option<CloudConf>,
    /// 字幕
    #[prost(message, optional, tag = "5")]
    pub subtitle_conf: ::core::option::Option<CloudConf>,
    /// 播放速度
    #[prost(message, optional, tag = "6")]
    pub playback_rate_conf: ::core::option::Option<CloudConf>,
    /// 定时停止
    #[prost(message, optional, tag = "7")]
    pub time_up_conf: ::core::option::Option<CloudConf>,
    /// 播放方式
    #[prost(message, optional, tag = "8")]
    pub playback_mode_conf: ::core::option::Option<CloudConf>,
    /// 画面尺寸
    #[prost(message, optional, tag = "9")]
    pub scale_mode_conf: ::core::option::Option<CloudConf>,
    /// 赞
    #[prost(message, optional, tag = "10")]
    pub like_conf: ::core::option::Option<CloudConf>,
    /// 踩
    #[prost(message, optional, tag = "11")]
    pub dislike_conf: ::core::option::Option<CloudConf>,
    /// 投币
    #[prost(message, optional, tag = "12")]
    pub coin_conf: ::core::option::Option<CloudConf>,
    /// 充电
    #[prost(message, optional, tag = "13")]
    pub elec_conf: ::core::option::Option<CloudConf>,
    /// 分享
    #[prost(message, optional, tag = "14")]
    pub share_conf: ::core::option::Option<CloudConf>,
    /// 截图
    #[prost(message, optional, tag = "15")]
    pub screen_shot_conf: ::core::option::Option<CloudConf>,
    /// 锁定
    #[prost(message, optional, tag = "16")]
    pub lock_screen_conf: ::core::option::Option<CloudConf>,
    /// 相关推荐
    #[prost(message, optional, tag = "17")]
    pub recommend_conf: ::core::option::Option<CloudConf>,
    /// 播放速度
    #[prost(message, optional, tag = "18")]
    pub playback_speed_conf: ::core::option::Option<CloudConf>,
    /// 清晰度
    #[prost(message, optional, tag = "19")]
    pub definition_conf: ::core::option::Option<CloudConf>,
    /// 选集
    #[prost(message, optional, tag = "20")]
    pub selections_conf: ::core::option::Option<CloudConf>,
    /// 下一集
    #[prost(message, optional, tag = "21")]
    pub next_conf: ::core::option::Option<CloudConf>,
    /// 编辑弹幕
    #[prost(message, optional, tag = "22")]
    pub edit_dm_conf: ::core::option::Option<CloudConf>,
    /// 小窗
    #[prost(message, optional, tag = "23")]
    pub small_window_conf: ::core::option::Option<CloudConf>,
    /// 震动
    #[prost(message, optional, tag = "24")]
    pub shake_conf: ::core::option::Option<CloudConf>,
    /// 外层面板弹幕设置
    #[prost(message, optional, tag = "25")]
    pub outer_dm_conf: ::core::option::Option<CloudConf>,
    /// 三点内弹幕设置
    #[prost(message, optional, tag = "26")]
    pub inner_dm_disable: ::core::option::Option<CloudConf>,
    /// 一起看入口
    #[prost(message, optional, tag = "27")]
    pub inner_dm_conf: ::core::option::Option<CloudConf>,
    /// 杜比音效
    #[prost(message, optional, tag = "28")]
    pub dolby_conf: ::core::option::Option<CloudConf>,
    /// 颜色滤镜
    #[prost(message, optional, tag = "29")]
    pub color_filter_conf: ::core::option::Option<CloudConf>,
}
/// 播放控件稿件配置
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayArcConf {
    /// 后台播放
    #[prost(message, optional, tag = "1")]
    pub background_play_conf: ::core::option::Option<ArcConf>,
    /// 镜像反转
    #[prost(message, optional, tag = "2")]
    pub flip_conf: ::core::option::Option<ArcConf>,
    /// 投屏
    #[prost(message, optional, tag = "3")]
    pub cast_conf: ::core::option::Option<ArcConf>,
    /// 反馈
    #[prost(message, optional, tag = "4")]
    pub feedback_conf: ::core::option::Option<ArcConf>,
    /// 字幕
    #[prost(message, optional, tag = "5")]
    pub subtitle_conf: ::core::option::Option<ArcConf>,
    /// 播放速度
    #[prost(message, optional, tag = "6")]
    pub playback_rate_conf: ::core::option::Option<ArcConf>,
    /// 定时停止
    #[prost(message, optional, tag = "7")]
    pub time_up_conf: ::core::option::Option<ArcConf>,
    /// 播放方式
    #[prost(message, optional, tag = "8")]
    pub playback_mode_conf: ::core::option::Option<ArcConf>,
    /// 画面尺寸
    #[prost(message, optional, tag = "9")]
    pub scale_mode_conf: ::core::option::Option<ArcConf>,
    /// 赞
    #[prost(message, optional, tag = "10")]
    pub like_conf: ::core::option::Option<ArcConf>,
    /// 踩
    #[prost(message, optional, tag = "11")]
    pub dislike_conf: ::core::option::Option<ArcConf>,
    /// 投币
    #[prost(message, optional, tag = "12")]
    pub coin_conf: ::core::option::Option<ArcConf>,
    /// 充电
    #[prost(message, optional, tag = "13")]
    pub elec_conf: ::core::option::Option<ArcConf>,
    /// 分享
    #[prost(message, optional, tag = "14")]
    pub share_conf: ::core::option::Option<ArcConf>,
    /// 截图
    #[prost(message, optional, tag = "15")]
    pub screen_shot_conf: ::core::option::Option<ArcConf>,
    /// 锁定
    #[prost(message, optional, tag = "16")]
    pub lock_screen_conf: ::core::option::Option<ArcConf>,
    /// 相关推荐
    #[prost(message, optional, tag = "17")]
    pub recommend_conf: ::core::option::Option<ArcConf>,
    /// 播放速度
    #[prost(message, optional, tag = "18")]
    pub playback_speed_conf: ::core::option::Option<ArcConf>,
    /// 清晰度
    #[prost(message, optional, tag = "19")]
    pub definition_conf: ::core::option::Option<ArcConf>,
    /// 选集
    #[prost(message, optional, tag = "20")]
    pub selections_conf: ::core::option::Option<ArcConf>,
    /// 下一集
    #[prost(message, optional, tag = "21")]
    pub next_conf: ::core::option::Option<ArcConf>,
    /// 编辑弹幕
    #[prost(message, optional, tag = "22")]
    pub edit_dm_conf: ::core::option::Option<ArcConf>,
    /// 小窗
    #[prost(message, optional, tag = "23")]
    pub small_window_conf: ::core::option::Option<ArcConf>,
    /// 震动
    #[prost(message, optional, tag = "24")]
    pub shake_conf: ::core::option::Option<ArcConf>,
    /// 外层面板弹幕设置
    #[prost(message, optional, tag = "25")]
    pub outer_dm_conf: ::core::option::Option<ArcConf>,
    /// 三点内弹幕设置
    #[prost(message, optional, tag = "26")]
    pub inner_dm_conf: ::core::option::Option<ArcConf>,
    /// 一起看入口
    #[prost(message, optional, tag = "27")]
    pub panorama_conf: ::core::option::Option<ArcConf>,
    /// 杜比音效
    #[prost(message, optional, tag = "28")]
    pub dolby_conf: ::core::option::Option<ArcConf>,
    /// 屏幕录制
    #[prost(message, optional, tag = "29")]
    pub screen_recording_conf: ::core::option::Option<ArcConf>,
    /// 颜色滤镜
    #[prost(message, optional, tag = "30")]
    pub color_filter_conf: ::core::option::Option<ArcConf>,
}
/// 编辑播放界面配置-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayConfEditReply {}
/// 编辑播放界面配置-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayConfEditReq {
    /// 播放界面配置
    #[prost(message, repeated, tag = "1")]
    pub play_conf: ::prost::alloc::vec::Vec<PlayConfState>,
}
/// 获取播放界面配置-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayConfReply {
    /// 播放控件用户自定义配置
    #[prost(message, optional, tag = "1")]
    pub play_conf: ::core::option::Option<PlayAbilityConf>,
}
/// 获取播放界面配置-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayConfReq {}
/// 播放界面配置
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayConfState {
    /// 设置类型
    #[prost(enumeration = "ConfType", tag = "1")]
    pub conf_type: i32,
    /// 是否隐藏
    #[prost(bool, tag = "2")]
    pub show: bool,
    /// 配置字段值
    #[prost(message, optional, tag = "3")]
    pub field_value: ::core::option::Option<FieldValue>,
    ///
    #[prost(message, optional, tag = "4")]
    pub conf_value: ::core::option::Option<ConfValue>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayLimit {
    ///
    #[prost(enumeration = "PlayLimitCode", tag = "1")]
    pub code: i32,
    ///
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub sub_message: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "4")]
    pub button: ::core::option::Option<ButtonStyle>,
}
/// 视频地址-回复
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayUrlReply {
    /// 清晰的
    #[prost(uint32, tag = "1")]
    pub quality: u32,
    /// 格式
    #[prost(string, tag = "2")]
    pub format: ::prost::alloc::string::String,
    /// 总时长(单位为ms)
    #[prost(uint64, tag = "3")]
    pub timelength: u64,
    /// 编码id
    #[prost(uint32, tag = "4")]
    pub video_codecid: u32,
    /// 视频流版本
    #[prost(uint32, tag = "5")]
    pub fnver: u32,
    /// 视频流格式
    #[prost(uint32, tag = "6")]
    pub fnval: u32,
    /// 是否支持投影
    #[prost(bool, tag = "7")]
    pub video_project: bool,
    /// 分段视频流列表
    #[prost(message, repeated, tag = "8")]
    pub durl: ::prost::alloc::vec::Vec<ResponseUrl>,
    /// dash数据
    #[prost(message, optional, tag = "9")]
    pub dash: ::core::option::Option<ResponseDash>,
    /// 是否非全二压
    #[prost(int32, tag = "10")]
    pub no_rexcode: i32,
    /// 互动视频升级提示
    #[prost(message, optional, tag = "11")]
    pub upgrade_limit: ::core::option::Option<UpgradeLimit>,
    /// 清晰度描述列表
    #[prost(message, repeated, tag = "12")]
    pub support_formats: ::prost::alloc::vec::Vec<FormatDescription>,
    /// 视频格式
    #[prost(enumeration = "VideoType", tag = "13")]
    pub r#type: i32,
}
/// 视频地址-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayUrlReq {
    /// 稿件avid
    #[prost(int64, tag = "1")]
    pub aid: i64,
    /// 视频cid
    #[prost(int64, tag = "2")]
    pub cid: i64,
    /// 清晰度
    #[prost(int64, tag = "3")]
    pub qn: i64,
    /// 视频流版本
    #[prost(int32, tag = "4")]
    pub fnver: i32,
    /// 视频流格式
    #[prost(int32, tag = "5")]
    pub fnval: i32,
    /// 下载模式
    /// 0:播放 1:flv下载 2:dash下载
    #[prost(uint32, tag = "6")]
    pub download: u32,
    /// 流url强制是用域名
    /// 0:允许使用ip 1:使用http 2:使用https
    #[prost(int32, tag = "7")]
    pub force_host: i32,
    /// 是否4K
    #[prost(bool, tag = "8")]
    pub fourk: bool,
    /// 当前页spm
    #[prost(string, tag = "9")]
    pub spmid: ::prost::alloc::string::String,
    /// 上一页spm
    #[prost(string, tag = "10")]
    pub from_spmid: ::prost::alloc::string::String,
}
/// 播放页信息-回复
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayViewReply {
    /// 视频流信息
    #[prost(message, optional, tag = "1")]
    pub video_info: ::core::option::Option<VideoInfo>,
    /// 播放控件用户自定义配置
    #[prost(message, optional, tag = "2")]
    pub play_conf: ::core::option::Option<PlayAbilityConf>,
    /// 互动视频升级提示
    #[prost(message, optional, tag = "3")]
    pub upgrade_limit: ::core::option::Option<UpgradeLimit>,
    /// Chronos灰度管理
    #[prost(message, optional, tag = "4")]
    pub chronos: ::core::option::Option<Chronos>,
    /// 播放控件稿件配置
    #[prost(message, optional, tag = "5")]
    pub play_arc: ::core::option::Option<PlayArcConf>,
    /// 事件
    #[prost(message, optional, tag = "6")]
    pub event: ::core::option::Option<Event>,
    ///
    #[prost(message, optional, tag = "7")]
    pub ab: ::core::option::Option<Ab>,
    ///
    #[prost(message, optional, tag = "8")]
    pub play_limit: ::core::option::Option<PlayLimit>,
}
/// 播放页信息-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayViewReq {
    /// 稿件avid
    #[prost(int64, tag = "1")]
    pub aid: i64,
    /// 视频cid
    #[prost(int64, tag = "2")]
    pub cid: i64,
    /// 清晰度
    #[prost(int64, tag = "3")]
    pub qn: i64,
    /// 视频流版本
    #[prost(int32, tag = "4")]
    pub fnver: i32,
    /// 视频流格式
    #[prost(int32, tag = "5")]
    pub fnval: i32,
    /// 下载模式
    /// 0:播放 1:flv下载 2:dash下载
    #[prost(uint32, tag = "6")]
    pub download: u32,
    /// 流url强制是用域名
    /// 0:允许使用ip 1:使用http 2:使用https
    #[prost(int32, tag = "7")]
    pub force_host: i32,
    /// 是否4K
    #[prost(bool, tag = "8")]
    pub fourk: bool,
    /// 当前页spm
    #[prost(string, tag = "9")]
    pub spmid: ::prost::alloc::string::String,
    /// 上一页spm
    #[prost(string, tag = "10")]
    pub from_spmid: ::prost::alloc::string::String,
    /// 青少年模式
    #[prost(int32, tag = "11")]
    pub teenagers_mode: i32,
    /// 编码
    #[prost(enumeration = "CodeType", tag = "12")]
    pub prefer_codec_type: i32,
    /// 业务类型
    #[prost(enumeration = "Business", tag = "13")]
    pub business: i32,
    ///
    #[prost(int64, tag = "14")]
    pub voice_balance: i64,
}
/// 投屏地址-响应
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectReply {
    #[prost(message, optional, tag = "1")]
    pub project: ::core::option::Option<PlayUrlReply>,
}
/// 投屏地址-请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectReq {
    /// 稿件avid
    #[prost(int64, tag = "1")]
    pub aid: i64,
    /// 视频cid
    #[prost(int64, tag = "2")]
    pub cid: i64,
    /// 清晰度
    #[prost(int64, tag = "3")]
    pub qn: i64,
    /// 视频流版本
    #[prost(int32, tag = "4")]
    pub fnver: i32,
    /// 视频流格式
    #[prost(int32, tag = "5")]
    pub fnval: i32,
    /// 下载模式
    /// 0:播放 1:flv下载 2:dash下载
    #[prost(uint32, tag = "6")]
    pub download: u32,
    /// 流url强制是用域名
    /// 0:允许使用ip 1:使用http 2:使用https
    #[prost(int32, tag = "7")]
    pub force_host: i32,
    /// 是否4K
    #[prost(bool, tag = "8")]
    pub fourk: bool,
    /// 当前页spm
    #[prost(string, tag = "9")]
    pub spmid: ::prost::alloc::string::String,
    /// 上一页spm
    #[prost(string, tag = "10")]
    pub from_spmid: ::prost::alloc::string::String,
    /// 使用协议
    /// 0:默认乐播 1:自建协议 2:云投屏 3:airplay
    #[prost(int32, tag = "11")]
    pub protocol: i32,
    /// 投屏设备
    /// 0:默认其他 1:OTT设备
    #[prost(int32, tag = "12")]
    pub device_type: i32,
}
/// dash数据
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseDash {
    /// dash视频流
    #[prost(message, repeated, tag = "1")]
    pub video: ::prost::alloc::vec::Vec<DashItem>,
    /// dash伴音流
    #[prost(message, repeated, tag = "2")]
    pub audio: ::prost::alloc::vec::Vec<DashItem>,
}
/// 分段流条目
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseUrl {
    /// 分段序号
    #[prost(uint32, tag = "1")]
    pub order: u32,
    /// 分段时长
    #[prost(uint64, tag = "2")]
    pub length: u64,
    /// 分段大小
    #[prost(uint64, tag = "3")]
    pub size: u64,
    /// 主线流
    #[prost(string, tag = "4")]
    pub url: ::prost::alloc::string::String,
    /// 备用流
    #[prost(string, repeated, tag = "5")]
    pub backup_url: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// md5
    #[prost(string, tag = "6")]
    pub md5: ::prost::alloc::string::String,
}
/// 分段视频流
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SegmentVideo {
    /// 分段视频流列表
    #[prost(message, repeated, tag = "1")]
    pub segment: ::prost::alloc::vec::Vec<ResponseUrl>,
}
/// 震动
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Shake {
    /// 文件地址
    #[prost(string, tag = "1")]
    pub file: ::prost::alloc::string::String,
}
/// 视频流信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Stream {
    /// 元数据
    #[prost(message, optional, tag = "1")]
    pub stream_info: ::core::option::Option<StreamInfo>,
    /// 流数据
    #[prost(oneof = "stream::Content", tags = "2, 3")]
    pub content: ::core::option::Option<stream::Content>,
}
/// Nested message and enum types in `Stream`.
pub mod stream {
    /// 流数据
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        /// dash流
        #[prost(message, tag = "2")]
        DashVideo(super::DashVideo),
        /// 分段流
        #[prost(message, tag = "3")]
        SegmentVideo(super::SegmentVideo),
    }
}
/// 流媒体元数据
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamInfo {
    /// 清晰度
    #[prost(uint32, tag = "1")]
    pub quality: u32,
    /// 格式
    #[prost(string, tag = "2")]
    pub format: ::prost::alloc::string::String,
    /// 格式描述
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// 错误码
    #[prost(enumeration = "PlayErr", tag = "4")]
    pub err_code: i32,
    /// 不满足条件信息
    #[prost(message, optional, tag = "5")]
    pub limit: ::core::option::Option<StreamLimit>,
    /// 是否需要vip
    #[prost(bool, tag = "6")]
    pub need_vip: bool,
    /// 是否需要登录
    #[prost(bool, tag = "7")]
    pub need_login: bool,
    /// 是否完整
    #[prost(bool, tag = "8")]
    pub intact: bool,
    /// 是否非全二压
    #[prost(bool, tag = "9")]
    pub no_rexcode: bool,
    /// 清晰度属性位
    #[prost(int64, tag = "10")]
    pub attribute: i64,
    /// 新版格式描述
    #[prost(string, tag = "11")]
    pub new_description: ::prost::alloc::string::String,
    /// 格式文字
    #[prost(string, tag = "12")]
    pub display_desc: ::prost::alloc::string::String,
    /// 新版格式描述备注
    #[prost(string, tag = "13")]
    pub superscript: ::prost::alloc::string::String,
}
/// 清晰度不满足条件信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamLimit {
    /// 标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 跳转地址
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// 提示信息
    #[prost(string, tag = "3")]
    pub msg: ::prost::alloc::string::String,
}
/// 互动视频升级按钮信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradeButton {
    /// 标题
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// 链接
    #[prost(string, tag = "2")]
    pub link: ::prost::alloc::string::String,
}
/// 互动视频升级提示
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradeLimit {
    /// 错误码
    #[prost(int32, tag = "1")]
    pub code: i32,
    /// 错误信息
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    /// 图片url
    #[prost(string, tag = "3")]
    pub image: ::prost::alloc::string::String,
    /// 按钮信息
    #[prost(message, optional, tag = "4")]
    pub button: ::core::option::Option<UpgradeButton>,
}
/// 视频url信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoInfo {
    /// 视频清晰度
    #[prost(uint32, tag = "1")]
    pub quality: u32,
    /// 视频格式
    #[prost(string, tag = "2")]
    pub format: ::prost::alloc::string::String,
    /// 视频时长
    #[prost(uint64, tag = "3")]
    pub timelength: u64,
    /// 视频编码id
    #[prost(uint32, tag = "4")]
    pub video_codecid: u32,
    /// 视频流
    #[prost(message, repeated, tag = "5")]
    pub stream_list: ::prost::alloc::vec::Vec<Stream>,
    /// 伴音流
    #[prost(message, repeated, tag = "6")]
    pub dash_audio: ::prost::alloc::vec::Vec<DashItem>,
    /// 杜比伴音流
    #[prost(message, optional, tag = "7")]
    pub dolby: ::core::option::Option<DolbyItem>,
    ///
    #[prost(message, optional, tag = "8")]
    pub volume: ::core::option::Option<VolumeInfo>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeInfo {
    ///
    #[prost(double, tag = "1")]
    pub measured_i: f64,
    ///
    #[prost(double, tag = "2")]
    pub measured_lra: f64,
    ///
    #[prost(double, tag = "3")]
    pub measured_tp: f64,
    ///
    #[prost(double, tag = "4")]
    pub measured_threshold: f64,
    ///
    #[prost(double, tag = "5")]
    pub target_offset: f64,
    ///
    #[prost(double, tag = "6")]
    pub target_i: f64,
    ///
    #[prost(double, tag = "7")]
    pub target_tp: f64,
}
/// 业务类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Business {
    /// 未知类型
    Unknown = 0,
    /// story业务
    Story = 1,
}
impl Business {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Business::Unknown => "UNKNOWN",
            Business::Story => "STORY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "STORY" => Some(Self::Story),
            _ => None,
        }
    }
}
/// 编码类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CodeType {
    /// 默认
    Nocode = 0,
    /// H.264
    Code264 = 1,
    /// H.265
    Code265 = 2,
    /// av1
    Codeav1 = 3,
}
impl CodeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CodeType::Nocode => "NOCODE",
            CodeType::Code264 => "CODE264",
            CodeType::Code265 => "CODE265",
            CodeType::Codeav1 => "CODEAV1",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NOCODE" => Some(Self::Nocode),
            "CODE264" => Some(Self::Code264),
            "CODE265" => Some(Self::Code265),
            "CODEAV1" => Some(Self::Codeav1),
            _ => None,
        }
    }
}
/// 设置类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ConfType {
    ///
    NoType = 0,
    /// 镜像反转
    Flipconf = 1,
    /// 视频投屏
    Castconf = 2,
    /// 反馈
    Feedback = 3,
    /// 字幕
    Subtitle = 4,
    /// 播放速度
    Playbackrate = 5,
    /// 定时停止播放
    Timeup = 6,
    /// 播放方式
    Playbackmode = 7,
    /// 画面尺寸
    Scalemode = 8,
    /// 后台播放
    Backgroundplay = 9,
    /// 顶
    Like = 10,
    /// 踩
    Dislike = 11,
    /// 投币
    Coin = 12,
    /// 充电
    Elec = 13,
    /// 分享
    Share = 14,
    /// 截图
    Screenshot = 15,
    /// 锁屏
    Lockscreen = 16,
    /// 推荐
    Recommend = 17,
    /// 倍速
    Playbackspeed = 18,
    /// 清晰度
    Definition = 19,
    /// 选集
    Selections = 20,
    /// 下一集
    Next = 21,
    /// 编辑弹幕
    Editdm = 22,
    /// 小窗
    Smallwindow = 23,
    /// 播放震动
    Shake = 24,
    /// 外层面板弹幕设置
    Outerdm = 25,
    /// 三点内弹幕设置
    Innerdm = 26,
    /// 全景
    Panorama = 27,
    /// 杜比
    Dolby = 28,
    /// 颜色滤镜
    Colorfilter = 29,
}
impl ConfType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ConfType::NoType => "NoType",
            ConfType::Flipconf => "FLIPCONF",
            ConfType::Castconf => "CASTCONF",
            ConfType::Feedback => "FEEDBACK",
            ConfType::Subtitle => "SUBTITLE",
            ConfType::Playbackrate => "PLAYBACKRATE",
            ConfType::Timeup => "TIMEUP",
            ConfType::Playbackmode => "PLAYBACKMODE",
            ConfType::Scalemode => "SCALEMODE",
            ConfType::Backgroundplay => "BACKGROUNDPLAY",
            ConfType::Like => "LIKE",
            ConfType::Dislike => "DISLIKE",
            ConfType::Coin => "COIN",
            ConfType::Elec => "ELEC",
            ConfType::Share => "SHARE",
            ConfType::Screenshot => "SCREENSHOT",
            ConfType::Lockscreen => "LOCKSCREEN",
            ConfType::Recommend => "RECOMMEND",
            ConfType::Playbackspeed => "PLAYBACKSPEED",
            ConfType::Definition => "DEFINITION",
            ConfType::Selections => "SELECTIONS",
            ConfType::Next => "NEXT",
            ConfType::Editdm => "EDITDM",
            ConfType::Smallwindow => "SMALLWINDOW",
            ConfType::Shake => "SHAKE",
            ConfType::Outerdm => "OUTERDM",
            ConfType::Innerdm => "INNERDM",
            ConfType::Panorama => "PANORAMA",
            ConfType::Dolby => "DOLBY",
            ConfType::Colorfilter => "COLORFILTER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NoType" => Some(Self::NoType),
            "FLIPCONF" => Some(Self::Flipconf),
            "CASTCONF" => Some(Self::Castconf),
            "FEEDBACK" => Some(Self::Feedback),
            "SUBTITLE" => Some(Self::Subtitle),
            "PLAYBACKRATE" => Some(Self::Playbackrate),
            "TIMEUP" => Some(Self::Timeup),
            "PLAYBACKMODE" => Some(Self::Playbackmode),
            "SCALEMODE" => Some(Self::Scalemode),
            "BACKGROUNDPLAY" => Some(Self::Backgroundplay),
            "LIKE" => Some(Self::Like),
            "DISLIKE" => Some(Self::Dislike),
            "COIN" => Some(Self::Coin),
            "ELEC" => Some(Self::Elec),
            "SHARE" => Some(Self::Share),
            "SCREENSHOT" => Some(Self::Screenshot),
            "LOCKSCREEN" => Some(Self::Lockscreen),
            "RECOMMEND" => Some(Self::Recommend),
            "PLAYBACKSPEED" => Some(Self::Playbackspeed),
            "DEFINITION" => Some(Self::Definition),
            "SELECTIONS" => Some(Self::Selections),
            "NEXT" => Some(Self::Next),
            "EDITDM" => Some(Self::Editdm),
            "SMALLWINDOW" => Some(Self::Smallwindow),
            "SHAKE" => Some(Self::Shake),
            "OUTERDM" => Some(Self::Outerdm),
            "INNERDM" => Some(Self::Innerdm),
            "PANORAMA" => Some(Self::Panorama),
            "DOLBY" => Some(Self::Dolby),
            "COLORFILTER" => Some(Self::Colorfilter),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Group {
    ///
    UnknownGroup = 0,
    ///
    A = 1,
    ///
    B = 2,
    ///
    C = 3,
}
impl Group {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Group::UnknownGroup => "UnknownGroup",
            Group::A => "A",
            Group::B => "B",
            Group::C => "C",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UnknownGroup" => Some(Self::UnknownGroup),
            "A" => Some(Self::A),
            "B" => Some(Self::B),
            "C" => Some(Self::C),
            _ => None,
        }
    }
}
/// 错误码
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PlayErr {
    ///
    NoErr = 0,
    /// 管控类型的错误码
    WithMultiDeviceLoginErr = 1,
}
impl PlayErr {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PlayErr::NoErr => "NoErr",
            PlayErr::WithMultiDeviceLoginErr => "WithMultiDeviceLoginErr",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NoErr" => Some(Self::NoErr),
            "WithMultiDeviceLoginErr" => Some(Self::WithMultiDeviceLoginErr),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PlayLimitCode {
    ///
    PlcUnkown = 0,
    ///
    PlcUgcNotPayed = 1,
}
impl PlayLimitCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PlayLimitCode::PlcUnkown => "PLCUnkown",
            PlayLimitCode::PlcUgcNotPayed => "PLCUgcNotPayed",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PLCUnkown" => Some(Self::PlcUnkown),
            "PLCUgcNotPayed" => Some(Self::PlcUgcNotPayed),
            _ => None,
        }
    }
}
/// 视频类型
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VideoType {
    ///
    UnknownValue = 0,
    /// flv格式
    FlvValue = 1,
    /// dash格式
    DashValue = 2,
    /// mp4格式
    Mp4Value = 3,
}
impl VideoType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VideoType::UnknownValue => "Unknown_VALUE",
            VideoType::FlvValue => "FLV_VALUE",
            VideoType::DashValue => "DASH_VALUE",
            VideoType::Mp4Value => "MP4_VALUE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Unknown_VALUE" => Some(Self::UnknownValue),
            "FLV_VALUE" => Some(Self::FlvValue),
            "DASH_VALUE" => Some(Self::DashValue),
            "MP4_VALUE" => Some(Self::Mp4Value),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod play_url_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// 视频url
    #[derive(Debug, Clone)]
    pub struct PlayUrlClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PlayUrlClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PlayUrlClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> PlayUrlClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            PlayUrlClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// 视频地址
        pub async fn play_url(
            &mut self,
            request: impl tonic::IntoRequest<super::PlayUrlReq>,
        ) -> Result<tonic::Response<super::PlayUrlReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.app.playurl.v1.PlayURL/PlayURL",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// 投屏地址
        pub async fn project(
            &mut self,
            request: impl tonic::IntoRequest<super::ProjectReq>,
        ) -> Result<tonic::Response<super::ProjectReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.app.playurl.v1.PlayURL/Project",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// 播放页信息
        pub async fn play_view(
            &mut self,
            request: impl tonic::IntoRequest<super::PlayViewReq>,
        ) -> Result<tonic::Response<super::PlayViewReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.app.playurl.v1.PlayURL/PlayView",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// 编辑播放界面配置
        pub async fn play_conf_edit(
            &mut self,
            request: impl tonic::IntoRequest<super::PlayConfEditReq>,
        ) -> Result<tonic::Response<super::PlayConfEditReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.app.playurl.v1.PlayURL/PlayConfEdit",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// 获取播放界面配置
        pub async fn play_conf(
            &mut self,
            request: impl tonic::IntoRequest<super::PlayConfReq>,
        ) -> Result<tonic::Response<super::PlayConfReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/bilibili.app.playurl.v1.PlayURL/PlayConf",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod play_url_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with PlayUrlServer.
    #[async_trait]
    pub trait PlayUrl: Send + Sync + 'static {
        /// 视频地址
        async fn play_url(
            &self,
            request: tonic::Request<super::PlayUrlReq>,
        ) -> Result<tonic::Response<super::PlayUrlReply>, tonic::Status>;
        /// 投屏地址
        async fn project(
            &self,
            request: tonic::Request<super::ProjectReq>,
        ) -> Result<tonic::Response<super::ProjectReply>, tonic::Status>;
        /// 播放页信息
        async fn play_view(
            &self,
            request: tonic::Request<super::PlayViewReq>,
        ) -> Result<tonic::Response<super::PlayViewReply>, tonic::Status>;
        /// 编辑播放界面配置
        async fn play_conf_edit(
            &self,
            request: tonic::Request<super::PlayConfEditReq>,
        ) -> Result<tonic::Response<super::PlayConfEditReply>, tonic::Status>;
        /// 获取播放界面配置
        async fn play_conf(
            &self,
            request: tonic::Request<super::PlayConfReq>,
        ) -> Result<tonic::Response<super::PlayConfReply>, tonic::Status>;
    }
    /// 视频url
    #[derive(Debug)]
    pub struct PlayUrlServer<T: PlayUrl> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: PlayUrl> PlayUrlServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PlayUrlServer<T>
    where
        T: PlayUrl,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/bilibili.app.playurl.v1.PlayURL/PlayURL" => {
                    #[allow(non_camel_case_types)]
                    struct PlayURLSvc<T: PlayUrl>(pub Arc<T>);
                    impl<T: PlayUrl> tonic::server::UnaryService<super::PlayUrlReq>
                    for PlayURLSvc<T> {
                        type Response = super::PlayUrlReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PlayUrlReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).play_url(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PlayURLSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/bilibili.app.playurl.v1.PlayURL/Project" => {
                    #[allow(non_camel_case_types)]
                    struct ProjectSvc<T: PlayUrl>(pub Arc<T>);
                    impl<T: PlayUrl> tonic::server::UnaryService<super::ProjectReq>
                    for ProjectSvc<T> {
                        type Response = super::ProjectReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ProjectReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).project(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ProjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/bilibili.app.playurl.v1.PlayURL/PlayView" => {
                    #[allow(non_camel_case_types)]
                    struct PlayViewSvc<T: PlayUrl>(pub Arc<T>);
                    impl<T: PlayUrl> tonic::server::UnaryService<super::PlayViewReq>
                    for PlayViewSvc<T> {
                        type Response = super::PlayViewReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PlayViewReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).play_view(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PlayViewSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/bilibili.app.playurl.v1.PlayURL/PlayConfEdit" => {
                    #[allow(non_camel_case_types)]
                    struct PlayConfEditSvc<T: PlayUrl>(pub Arc<T>);
                    impl<T: PlayUrl> tonic::server::UnaryService<super::PlayConfEditReq>
                    for PlayConfEditSvc<T> {
                        type Response = super::PlayConfEditReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PlayConfEditReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).play_conf_edit(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PlayConfEditSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/bilibili.app.playurl.v1.PlayURL/PlayConf" => {
                    #[allow(non_camel_case_types)]
                    struct PlayConfSvc<T: PlayUrl>(pub Arc<T>);
                    impl<T: PlayUrl> tonic::server::UnaryService<super::PlayConfReq>
                    for PlayConfSvc<T> {
                        type Response = super::PlayConfReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PlayConfReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).play_conf(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PlayConfSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: PlayUrl> Clone for PlayUrlServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: PlayUrl> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: PlayUrl> tonic::server::NamedService for PlayUrlServer<T> {
        const NAME: &'static str = "bilibili.app.playurl.v1.PlayURL";
    }
}
