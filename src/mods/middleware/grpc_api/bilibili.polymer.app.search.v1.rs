///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Args {
    ///
    #[prost(int32, tag = "1")]
    pub online: i32,
    ///
    #[prost(string, tag = "2")]
    pub rname: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub room_id: i64,
    ///
    #[prost(string, tag = "4")]
    pub tname: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "5")]
    pub up_id: i64,
    ///
    #[prost(string, tag = "6")]
    pub up_name: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "7")]
    pub rid: i64,
    ///
    #[prost(int64, tag = "8")]
    pub tid: i64,
    ///
    #[prost(int64, tag = "9")]
    pub aid: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Avatar {
    ///
    #[prost(string, tag = "1")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub event: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub event_v2: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "5")]
    pub up_id: i64,
    ///
    #[prost(string, tag = "6")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "7")]
    pub face_nft_new: i32,
    ///
    #[prost(message, optional, tag = "8")]
    pub nft_face_icon: ::core::option::Option<NftFaceIcon>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AvItem {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub ctime_label: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub duration: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "6")]
    pub play: i32,
    ///
    #[prost(int64, tag = "7")]
    pub danmaku: i64,
    ///
    #[prost(int32, tag = "8")]
    pub ctime: i32,
    ///
    #[prost(string, tag = "9")]
    pub goto: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "10")]
    pub param: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "11")]
    pub position: i32,
    ///
    #[prost(string, tag = "12")]
    pub ctime_label_v2: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Background {
    ///
    #[prost(int32, tag = "1")]
    pub show: i32,
    ///
    #[prost(string, tag = "2")]
    pub bg_pic_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub fg_pic_url: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Badge {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub bg_cover: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Badge2 {
    ///
    #[prost(string, tag = "1")]
    pub bg_cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BottomButton {
    ///
    #[prost(string, tag = "1")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub link: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BrandAdAccount {
    ///
    #[prost(string, tag = "1")]
    pub param: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub goto: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub mid: i64,
    ///
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub face: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub sign: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "7")]
    pub relation: ::core::option::Option<Relation>,
    ///
    #[prost(int64, tag = "8")]
    pub roomid: i64,
    ///
    #[prost(int64, tag = "9")]
    pub live_status: i64,
    ///
    #[prost(string, tag = "10")]
    pub live_link: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "11")]
    pub official_verify: ::core::option::Option<OfficialVerify>,
    ///
    #[prost(message, optional, tag = "12")]
    pub vip: ::core::option::Option<VipInfo>,
    ///
    #[prost(string, tag = "13")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "14")]
    pub face_nft_new: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BrandAdArc {
    ///
    #[prost(string, tag = "1")]
    pub param: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub goto: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub aid: i64,
    ///
    #[prost(int64, tag = "4")]
    pub play: i64,
    ///
    #[prost(int64, tag = "5")]
    pub reply: i64,
    ///
    #[prost(string, tag = "6")]
    pub duration: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub author: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "10")]
    pub cover: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Button {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub param: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub event: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "5")]
    pub selected: i32,
    ///
    #[prost(int32, tag = "6")]
    pub r#type: i32,
    ///
    #[prost(string, tag = "7")]
    pub event_v2: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "8")]
    pub relation: ::core::option::Option<Relation>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ButtonMeta {
    ///
    #[prost(string, tag = "1")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub button_status: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub toast: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardBusinessBadge {
    ///
    #[prost(message, optional, tag = "1")]
    pub goto_icon: ::core::option::Option<GotoIcon>,
    ///
    #[prost(message, optional, tag = "2")]
    pub badge_style: ::core::option::Option<ReasonStyle>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelLabel {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelMixedItem {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
    ///
    #[prost(int32, tag = "2")]
    pub cover_left_icon1: i32,
    ///
    #[prost(string, tag = "3")]
    pub cover_left_text1: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub goto: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub param: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "9")]
    pub badge: ::core::option::Option<Badge2>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckMore {
    ///
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudGameParams {
    ///
    #[prost(int64, tag = "1")]
    pub source_from: i64,
    ///
    #[prost(string, tag = "2")]
    pub scene: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetailsRelationItem {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub cover_left_text: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "4")]
    pub cover_badge_style: ::core::option::Option<ReasonStyle>,
    ///
    #[prost(string, tag = "5")]
    pub module_pos: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub goto: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub param: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "9")]
    pub position: i32,
    ///
    #[prost(string, tag = "10")]
    pub cover_left_text_v2: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "11")]
    pub cover_badge_style_v2: ::core::option::Option<ReasonStyle>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DislikeReason {
    ///
    #[prost(int32, tag = "1")]
    pub id: i32,
    ///
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisplayOption {
    ///
    #[prost(int32, tag = "1")]
    pub video_title_row: i32,
    ///
    #[prost(int32, tag = "2")]
    pub search_page_visual_opti: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DyTopic {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EasterEgg {
    ///
    #[prost(int32, tag = "1")]
    pub id: i32,
    ///
    #[prost(int32, tag = "2")]
    pub show_count: i32,
    ///
    #[prost(int32, tag = "3")]
    pub r#type: i32,
    ///
    #[prost(string, tag = "4")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "5")]
    pub close_count: i32,
    ///
    #[prost(int32, tag = "6")]
    pub mask_transparency: i32,
    ///
    #[prost(string, tag = "7")]
    pub mask_color: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "8")]
    pub pic_type: i32,
    ///
    #[prost(int32, tag = "9")]
    pub show_time: i32,
    ///
    #[prost(string, tag = "10")]
    pub source_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "11")]
    pub source_md5: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "12")]
    pub source_size: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Episode {
    ///
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub param: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub index: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "4")]
    pub badges: ::prost::alloc::vec::Vec<ReasonStyle>,
    ///
    #[prost(int32, tag = "5")]
    pub position: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EpisodeNew {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub param: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "4")]
    pub is_new: i32,
    ///
    #[prost(message, repeated, tag = "5")]
    pub badges: ::prost::alloc::vec::Vec<ReasonStyle>,
    ///
    #[prost(int32, tag = "6")]
    pub r#type: i32,
    ///
    #[prost(int32, tag = "7")]
    pub position: i32,
    ///
    #[prost(string, tag = "8")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub label: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtraLink {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FollowButton {
    ///
    #[prost(string, tag = "1")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(map = "string, string", tag = "2")]
    pub texts: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    ///
    #[prost(string, tag = "3")]
    pub status_report: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FullTextResult {
    ///
    #[prost(int32, tag = "1")]
    pub r#type: i32,
    ///
    #[prost(string, tag = "2")]
    pub show_text: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub jump_start_progress: i64,
    ///
    #[prost(string, tag = "4")]
    pub jump_uri: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GotoIcon {
    ///
    #[prost(string, tag = "1")]
    pub icon_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub icon_night_url: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "3")]
    pub icon_width: i32,
    ///
    #[prost(int32, tag = "4")]
    pub icon_height: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InlineProgressBar {
    ///
    #[prost(string, tag = "1")]
    pub icon_drag: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub icon_drag_hash: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub icon_stop: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub icon_stop_hash: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InlineThreePointPanel {
    ///
    #[prost(int32, tag = "1")]
    pub panel_type: i32,
    ///
    #[prost(string, tag = "2")]
    pub share_id: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub share_origin: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "4")]
    pub functional_buttons: ::prost::alloc::vec::Vec<ShareButtonItem>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Item {
    ///
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub param: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub goto: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub linktype: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "5")]
    pub position: i32,
    ///
    #[prost(string, tag = "6")]
    pub trackid: ::prost::alloc::string::String,
    ///
    #[prost(
        oneof = "item::CardItem",
        tags = "7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39"
    )]
    pub card_item: ::core::option::Option<item::CardItem>,
}
/// Nested message and enum types in `Item`.
pub mod item {
    ///
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum CardItem {
        ///
        #[prost(message, tag = "7")]
        Special(super::SearchSpecialCard),
        ///
        #[prost(message, tag = "8")]
        Article(super::SearchArticleCard),
        ///
        #[prost(message, tag = "9")]
        Banner(super::SearchBannerCard),
        ///
        #[prost(message, tag = "10")]
        Live(super::SearchLiveCard),
        ///
        #[prost(message, tag = "11")]
        Game(super::SearchGameCard),
        ///
        #[prost(message, tag = "12")]
        Purchase(super::SearchPurchaseCard),
        ///
        #[prost(message, tag = "13")]
        RecommendWord(super::SearchRecommendWordCard),
        ///
        #[prost(message, tag = "14")]
        Dynamic(super::SearchDynamicCard),
        ///
        #[prost(message, tag = "15")]
        SuggestKeyword(super::SearchNoResultSuggestWordCard),
        ///
        #[prost(message, tag = "16")]
        SpecialGuide(super::SearchSpecialGuideCard),
        ///
        #[prost(message, tag = "17")]
        Comic(super::SearchComicCard),
        ///
        #[prost(message, tag = "18")]
        ChannelNew(super::SearchNewChannelCard),
        ///
        #[prost(message, tag = "19")]
        OgvCard(super::SearchOgvCard),
        ///
        #[prost(message, tag = "20")]
        BangumiRelates(super::SearchOgvRelationCard),
        ///
        #[prost(message, tag = "21")]
        FindMore(super::SearchOgvRecommendCard),
        ///
        #[prost(message, tag = "22")]
        Esport(super::SearchSportCard),
        ///
        #[prost(message, tag = "23")]
        AuthorNew(super::SearchAuthorNewCard),
        ///
        #[prost(message, tag = "24")]
        Tips(super::SearchTipsCard),
        ///
        #[prost(message, tag = "25")]
        Cm(super::SearchAdCard),
        ///
        #[prost(message, tag = "26")]
        PediaCard(super::SearchPediaCard),
        ///
        #[prost(message, tag = "27")]
        UgcInline(super::SearchUgcInlineCard),
        ///
        #[prost(message, tag = "28")]
        LiveInline(super::SearchLiveInlineCard),
        ///
        #[prost(message, tag = "29")]
        TopGame(super::SearchTopGameCard),
        ///
        #[prost(message, tag = "30")]
        Sports(super::SearchOlympicGameCard),
        ///
        #[prost(message, tag = "31")]
        PediaCardInline(super::SearchOlympicWikiCard),
        ///
        #[prost(message, tag = "32")]
        RecommendTips(super::SearchRecommendTipCard),
        ///
        #[prost(message, tag = "33")]
        CollectionCard(super::SearchCollectionCard),
        ///
        #[prost(message, tag = "34")]
        OgvChannel(super::SearchOgvChannelCard),
        ///
        #[prost(message, tag = "35")]
        OgvInline(super::SearchOgvInlineCard),
        ///
        #[prost(message, tag = "36")]
        Author(super::SearchUpperCard),
        ///
        #[prost(message, tag = "37")]
        Av(super::SearchVideoCard),
        ///
        #[prost(message, tag = "38")]
        Bangumi(super::SearchBangumiCard),
        ///
        #[prost(message, tag = "39")]
        EsportsInline(super::SearchSportInlineCard),
    }
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LikeResource {
    ///
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub content_hash: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiveBadgeResource {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub animation_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub animation_url_hash: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub background_color_light: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub background_color_night: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "6")]
    pub alpha_light: i64,
    ///
    #[prost(int64, tag = "7")]
    pub alpha_night: i64,
    ///
    #[prost(string, tag = "8")]
    pub font_color: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mask {
    ///
    #[prost(message, optional, tag = "1")]
    pub avatar: ::core::option::Option<Avatar>,
    ///
    #[prost(message, optional, tag = "2")]
    pub button: ::core::option::Option<Button>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchInfoObj {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
    ///
    #[prost(int32, tag = "2")]
    pub status: i32,
    ///
    #[prost(string, tag = "3")]
    pub match_stage: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "4")]
    pub team1: ::core::option::Option<MatchTeam>,
    ///
    #[prost(message, optional, tag = "5")]
    pub team2: ::core::option::Option<MatchTeam>,
    ///
    #[prost(message, optional, tag = "6")]
    pub match_label: ::core::option::Option<MatchItem>,
    ///
    #[prost(message, optional, tag = "7")]
    pub match_time: ::core::option::Option<MatchItem>,
    ///
    #[prost(message, optional, tag = "8")]
    pub match_button: ::core::option::Option<MatchItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchItem {
    ///
    #[prost(int32, tag = "1")]
    pub state: i32,
    ///
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub text_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub text_color_night: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub live_link: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "7")]
    pub texts: ::core::option::Option<Texts>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchTeam {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "4")]
    pub score: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Nav {
    ///
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "2")]
    pub total: i32,
    ///
    #[prost(int32, tag = "3")]
    pub pages: i32,
    ///
    #[prost(int32, tag = "4")]
    pub r#type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Navigation {
    ///
    #[prost(int64, tag = "1")]
    pub id: i64,
    ///
    #[prost(message, repeated, tag = "2")]
    pub children: ::prost::alloc::vec::Vec<Navigation>,
    ///
    #[prost(message, repeated, tag = "3")]
    pub inline_children: ::prost::alloc::vec::Vec<Navigation>,
    ///
    #[prost(string, tag = "4")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "6")]
    pub button: ::core::option::Option<NavigationButton>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NavigationButton {
    ///
    #[prost(int64, tag = "1")]
    pub r#type: i64,
    ///
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NftFaceIcon {
    ///
    #[prost(int32, tag = "1")]
    pub region_type: i32,
    ///
    #[prost(string, tag = "2")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "3")]
    pub show_status: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Notice {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
    ///
    #[prost(int64, tag = "2")]
    pub notice_id: i64,
    ///
    #[prost(string, tag = "3")]
    pub content: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "5")]
    pub notice_type: i64,
    ///
    #[prost(string, tag = "6")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub icon_night: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub text_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub text_color_night: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "10")]
    pub bg_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "11")]
    pub bg_color_night: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfficialVerify {
    ///
    #[prost(int32, tag = "1")]
    pub r#type: i32,
    ///
    #[prost(string, tag = "2")]
    pub desc: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OgvCardUi {
    ///
    #[prost(string, tag = "1")]
    pub background_image: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub gaussian_blur_value: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub module_color: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OgvClipInfo {
    ///
    #[prost(int64, tag = "1")]
    pub play_start_time: i64,
    ///
    #[prost(int64, tag = "2")]
    pub play_end_time: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OgvRecommendWord {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub goto: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub param: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub uri: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PediaCover {
    ///
    #[prost(int64, tag = "1")]
    pub cover_type: i64,
    ///
    #[prost(string, tag = "2")]
    pub cover_sun_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub cover_night_url: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "4")]
    pub cover_width: i32,
    ///
    #[prost(int32, tag = "5")]
    pub cover_height: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerArgs {
    ///
    #[prost(int32, tag = "1")]
    pub is_live: i32,
    ///
    #[prost(int64, tag = "2")]
    pub aid: i64,
    ///
    #[prost(int64, tag = "3")]
    pub cid: i64,
    ///
    #[prost(int32, tag = "4")]
    pub sub_type: i32,
    ///
    #[prost(int64, tag = "5")]
    pub room_id: i64,
    ///
    #[prost(int64, tag = "7")]
    pub ep_id: i64,
    ///
    #[prost(int32, tag = "8")]
    pub is_preview: i32,
    ///
    #[prost(string, tag = "9")]
    pub r#type: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "10")]
    pub duration: i32,
    ///
    #[prost(int64, tag = "11")]
    pub season_id: i64,
    ///
    #[prost(int32, tag = "12")]
    pub report_required_play_duration: i32,
    ///
    #[prost(int32, tag = "13")]
    pub report_required_time: i32,
    ///
    #[prost(int32, tag = "14")]
    pub manual_play: i32,
    ///
    #[prost(bool, tag = "15")]
    pub hide_play_button: bool,
    ///
    #[prost(int32, tag = "16")]
    pub content_mode: i32,
    ///
    #[prost(int32, tag = "17")]
    pub report_history: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerWidget {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub desc: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RankInfo {
    ///
    #[prost(string, tag = "1")]
    pub search_night_icon_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub search_day_icon_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub search_bkg_night_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub search_bkg_day_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub search_font_night_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub search_font_day_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub rank_content: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub rank_link: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RcmdReason {
    ///
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReasonStyle {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub text_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub text_color_night: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub bg_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub bg_color_night: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub border_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub border_color_night: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "8")]
    pub bg_style: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecommendWord {
    ///
    #[prost(string, tag = "1")]
    pub param: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub from_source: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Relation {
    ///
    #[prost(int32, tag = "1")]
    pub status: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RightTopLiveBadge {
    ///
    #[prost(int32, tag = "1")]
    pub live_status: i32,
    ///
    #[prost(message, optional, tag = "2")]
    pub in_live: ::core::option::Option<LiveBadgeResource>,
    ///
    #[prost(string, tag = "3")]
    pub live_stats_desc: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAdCard {
    ///
    #[prost(string, tag = "1")]
    pub json_str: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAllRequest {
    ///
    #[prost(string, tag = "1")]
    pub keyword: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "2")]
    pub order: i32,
    ///
    #[prost(string, tag = "3")]
    pub tid_list: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub duration_list: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub extra_word: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub from_source: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "7")]
    pub is_org_query: i32,
    ///
    #[prost(int32, tag = "8")]
    pub local_time: i32,
    ///
    #[prost(string, tag = "9")]
    pub ad_extra: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "10")]
    pub pagination: ::core::option::Option<
        crate::mods::middleware::grpc_api::bilibili_pagination::Pagination,
    >,
    ///
    #[prost(message, optional, tag = "11")]
    pub player_args: ::core::option::Option<
        crate::mods::middleware::grpc_api::bilibili_app_archive_middleware_v1::PlayerArgs,
    >,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAllResponse {
    ///
    #[prost(string, tag = "1")]
    pub keyword: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub trackid: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "3")]
    pub nav: ::prost::alloc::vec::Vec<Nav>,
    ///
    #[prost(message, repeated, tag = "4")]
    pub item: ::prost::alloc::vec::Vec<Item>,
    ///
    #[prost(message, optional, tag = "5")]
    pub easter_egg: ::core::option::Option<EasterEgg>,
    ///
    #[prost(string, tag = "6")]
    pub exp_str: ::prost::alloc::string::String,
    ///
    #[prost(string, repeated, tag = "7")]
    pub extra_word_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(string, tag = "8")]
    pub org_extra_word: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "9")]
    pub select_bar_type: i64,
    ///
    #[prost(int64, tag = "10")]
    pub new_search_exp_num: i64,
    ///
    #[prost(message, optional, tag = "11")]
    pub pagination: ::core::option::Option<
        crate::mods::middleware::grpc_api::bilibili_pagination::PaginationReply,
    >,
    ///
    #[prost(message, optional, tag = "12")]
    pub app_display_option: ::core::option::Option<DisplayOption>,
    ///
    #[prost(map = "string, string", tag = "13")]
    pub annotation: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchArticleCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "3")]
    pub play: i32,
    ///
    #[prost(int32, tag = "4")]
    pub like: i32,
    ///
    #[prost(int32, tag = "5")]
    pub reply: i32,
    ///
    #[prost(string, repeated, tag = "6")]
    pub image_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(string, tag = "7")]
    pub author: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "8")]
    pub template_id: i32,
    ///
    #[prost(int64, tag = "9")]
    pub id: i64,
    ///
    #[prost(int64, tag = "10")]
    pub mid: i64,
    ///
    #[prost(string, tag = "11")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "12")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "13")]
    pub view: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAuthorNewCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "3")]
    pub live_face: i32,
    ///
    #[prost(string, tag = "4")]
    pub live_uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub live_link: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "6")]
    pub fans: i32,
    ///
    #[prost(int32, tag = "7")]
    pub level: i32,
    ///
    #[prost(string, tag = "8")]
    pub sign: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "9")]
    pub is_up: bool,
    ///
    #[prost(int32, tag = "10")]
    pub archives: i32,
    ///
    #[prost(int64, tag = "11")]
    pub mid: i64,
    ///
    #[prost(int64, tag = "12")]
    pub roomid: i64,
    ///
    #[prost(message, optional, tag = "13")]
    pub relation: ::core::option::Option<Relation>,
    ///
    #[prost(message, optional, tag = "14")]
    pub official_verify: ::core::option::Option<OfficialVerify>,
    ///
    #[prost(int32, tag = "15")]
    pub face_nft_new: i32,
    ///
    #[prost(message, optional, tag = "16")]
    pub nft_face_icon: ::core::option::Option<NftFaceIcon>,
    ///
    #[prost(int32, tag = "17")]
    pub is_senior_member: i32,
    ///
    #[prost(message, optional, tag = "18")]
    pub background: ::core::option::Option<Background>,
    ///
    #[prost(int32, tag = "19")]
    pub av_style: i32,
    ///
    #[prost(message, optional, tag = "20")]
    pub space: ::core::option::Option<Space>,
    ///
    #[prost(message, repeated, tag = "21")]
    pub av_items: ::prost::alloc::vec::Vec<AvItem>,
    ///
    #[prost(message, optional, tag = "22")]
    pub notice: ::core::option::Option<Notice>,
    ///
    #[prost(message, optional, tag = "23")]
    pub share_plane: ::core::option::Option<SharePlane>,
    ///
    #[prost(string, tag = "24")]
    pub inline_type: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "25")]
    pub inline_live: ::core::option::Option<SearchInlineData>,
    ///
    #[prost(int32, tag = "26")]
    pub is_inline_live: i32,
    ///
    #[prost(message, repeated, tag = "27")]
    pub three_point: ::prost::alloc::vec::Vec<ThreePoint>,
    ///
    #[prost(int32, tag = "28")]
    pub live_status: i32,
    ///
    #[prost(message, optional, tag = "29")]
    pub vip: ::core::option::Option<VipInfo>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchBangumiCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "3")]
    pub media_type: i32,
    ///
    #[prost(int32, tag = "4")]
    pub play_state: i32,
    ///
    #[prost(string, tag = "5")]
    pub area: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub style: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub styles: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub cv: ::prost::alloc::string::String,
    ///
    #[prost(double, tag = "9")]
    pub rating: f64,
    ///
    #[prost(int32, tag = "10")]
    pub vote: i32,
    ///
    #[prost(string, tag = "11")]
    pub target: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "12")]
    pub staff: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "13")]
    pub prompt: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "14")]
    pub ptime: i64,
    ///
    #[prost(string, tag = "15")]
    pub season_type_name: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "16")]
    pub episodes: ::prost::alloc::vec::Vec<Episode>,
    ///
    #[prost(int32, tag = "17")]
    pub is_selection: i32,
    ///
    #[prost(int32, tag = "18")]
    pub is_atten: i32,
    ///
    #[prost(string, tag = "19")]
    pub label: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "20")]
    pub season_id: i64,
    ///
    #[prost(string, tag = "21")]
    pub out_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "22")]
    pub out_icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "23")]
    pub out_url: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "24")]
    pub badges: ::prost::alloc::vec::Vec<ReasonStyle>,
    ///
    #[prost(int32, tag = "25")]
    pub is_out: i32,
    ///
    #[prost(message, repeated, tag = "26")]
    pub episodes_new: ::prost::alloc::vec::Vec<EpisodeNew>,
    ///
    #[prost(message, optional, tag = "27")]
    pub watch_button: ::core::option::Option<WatchButton>,
    ///
    #[prost(string, tag = "28")]
    pub selection_style: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "29")]
    pub check_more: ::core::option::Option<CheckMore>,
    ///
    #[prost(message, optional, tag = "30")]
    pub follow_button: ::core::option::Option<FollowButton>,
    ///
    #[prost(message, optional, tag = "31")]
    pub style_label: ::core::option::Option<ReasonStyle>,
    ///
    #[prost(message, repeated, tag = "32")]
    pub badges_v2: ::prost::alloc::vec::Vec<ReasonStyle>,
    ///
    #[prost(string, tag = "33")]
    pub styles_v2: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchBannerCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchByTypeRequest {
    /// 搜索目标类型, 番剧为7
    #[prost(int32, tag = "1")]
    pub r#type: i32,
    /// 关键词
    #[prost(string, tag = "2")]
    pub keyword: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "3")]
    pub category_sort: i32,
    ///
    #[prost(int64, tag = "4")]
    pub category_id: i64,
    ///
    #[prost(int32, tag = "5")]
    pub user_type: i32,
    ///
    #[prost(int32, tag = "6")]
    pub user_sort: i32,
    ///
    #[prost(message, optional, tag = "7")]
    pub pagination: ::core::option::Option<
        crate::mods::middleware::grpc_api::bilibili_pagination::Pagination,
    >,
    ///
    #[prost(message, optional, tag = "8")]
    pub player_args: ::core::option::Option<
        crate::mods::middleware::grpc_api::bilibili_app_archive_middleware_v1::PlayerArgs,
    >,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchByTypeResponse {
    /// 追踪id
    #[prost(string, tag = "1")]
    pub trackid: ::prost::alloc::string::String,
    /// 当前页码
    #[prost(int32, tag = "2")]
    pub pages: i32,
    ///
    #[prost(string, tag = "3")]
    pub exp_str: ::prost::alloc::string::String,
    /// 搜索关键词
    #[prost(string, tag = "4")]
    pub keyword: ::prost::alloc::string::String,
    /// 是否为推荐结果
    #[prost(int32, tag = "5")]
    pub result_is_recommend: i32,
    /// 搜索结果条目
    #[prost(message, repeated, tag = "6")]
    pub items: ::prost::alloc::vec::Vec<Item>,
    /// 分页信息
    #[prost(message, optional, tag = "7")]
    pub pagination: ::core::option::Option<
        crate::mods::middleware::grpc_api::bilibili_pagination::PaginationReply,
    >,
    ///
    #[prost(map = "string, string", tag = "8")]
    pub annotation: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchCollectionCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub author: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "4")]
    pub av_items: ::prost::alloc::vec::Vec<AvItem>,
    ///
    #[prost(message, optional, tag = "5")]
    pub bottom_button: ::core::option::Option<BottomButton>,
    ///
    #[prost(string, tag = "6")]
    pub collection_icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub show_card_desc1: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub show_card_desc2: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchComicCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub style: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub comic_url: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub badge: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchComicInfo {
    ///
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub param: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "3")]
    pub comic: ::core::option::Option<SearchComicCard>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchComicRequest {
    ///
    #[prost(string, tag = "1")]
    pub id_list: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchComicResponse {
    ///
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<SearchComicInfo>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchDynamicCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "3")]
    pub cover_count: i32,
    ///
    #[prost(string, repeated, tag = "4")]
    pub covers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///
    #[prost(message, optional, tag = "5")]
    pub upper: ::core::option::Option<Upper>,
    ///
    #[prost(message, optional, tag = "6")]
    pub stat: ::core::option::Option<Stat>,
    ///
    #[prost(message, repeated, tag = "7")]
    pub dy_topic: ::prost::alloc::vec::Vec<DyTopic>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchGameCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub reserve: ::prost::alloc::string::String,
    ///
    #[prost(float, tag = "4")]
    pub rating: f32,
    ///
    #[prost(string, tag = "5")]
    pub tags: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub notice_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub notice_content: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub gift_content: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub gift_url: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "10")]
    pub reserve_status: i32,
    ///
    #[prost(message, optional, tag = "11")]
    pub rank_info: ::core::option::Option<RankInfo>,
    ///
    #[prost(string, tag = "12")]
    pub special_bg_color: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "13")]
    pub cloud_game_params: ::core::option::Option<CloudGameParams>,
    ///
    #[prost(bool, tag = "14")]
    pub show_cloud_game_entry: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchInlineData {
    ///
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "3")]
    pub player_args: ::core::option::Option<PlayerArgs>,
    ///
    #[prost(int32, tag = "4")]
    pub can_play: i32,
    ///
    #[prost(message, optional, tag = "5")]
    pub args: ::core::option::Option<Args>,
    ///
    #[prost(string, tag = "6")]
    pub card_goto: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub card_type: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "9")]
    pub cover_left_icon1: i32,
    ///
    #[prost(int32, tag = "10")]
    pub cover_left_icon2: i32,
    ///
    #[prost(string, tag = "11")]
    pub cover_left_text1: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "12")]
    pub cover_left_text2: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "13")]
    pub up_args: ::core::option::Option<UpArgs>,
    ///
    #[prost(string, tag = "14")]
    pub extra_uri: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "15")]
    pub is_fav: bool,
    ///
    #[prost(bool, tag = "16")]
    pub is_coin: bool,
    ///
    #[prost(string, tag = "17")]
    pub goto: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "18")]
    pub share: ::core::option::Option<Share>,
    ///
    #[prost(message, optional, tag = "19")]
    pub three_point: ::core::option::Option<ThreePoint2>,
    ///
    #[prost(message, repeated, tag = "20")]
    pub three_point_v2: ::prost::alloc::vec::Vec<ThreePointV2>,
    ///
    #[prost(message, optional, tag = "21")]
    pub share_plane: ::core::option::Option<SharePlane>,
    ///
    #[prost(message, optional, tag = "22")]
    pub three_point_meta: ::core::option::Option<InlineThreePointPanel>,
    ///
    #[prost(message, optional, tag = "23")]
    pub avatar: ::core::option::Option<Avatar>,
    ///
    #[prost(string, tag = "24")]
    pub cover_right_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "25")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "26")]
    pub inline_progress_bar: ::core::option::Option<InlineProgressBar>,
    ///
    #[prost(message, optional, tag = "27")]
    pub like_button: ::core::option::Option<SearchLikeButtonItem>,
    ///
    #[prost(int32, tag = "28")]
    pub official_icon: i32,
    ///
    #[prost(int32, tag = "29")]
    pub official_icon_v2: i32,
    ///
    #[prost(string, tag = "30")]
    pub param: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "31")]
    pub traffic_config: ::core::option::Option<TrafficConfig>,
    ///
    #[prost(bool, tag = "32")]
    pub is_atten: bool,
    ///
    #[prost(message, optional, tag = "33")]
    pub goto_icon: ::core::option::Option<GotoIcon>,
    ///
    #[prost(bool, tag = "34")]
    pub disable_danmaku: bool,
    ///
    #[prost(bool, tag = "35")]
    pub hide_danmaku_switch: bool,
    ///
    #[prost(message, optional, tag = "36")]
    pub badge_style: ::core::option::Option<ReasonStyle>,
    ///
    #[prost(message, optional, tag = "37")]
    pub player_widget: ::core::option::Option<PlayerWidget>,
    ///
    #[prost(message, optional, tag = "38")]
    pub cover_badge_style: ::core::option::Option<ReasonStyle>,
    ///
    #[prost(message, optional, tag = "39")]
    pub right_top_live_badge: ::core::option::Option<RightTopLiveBadge>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchLikeButtonItem {
    ///
    #[prost(int64, tag = "1")]
    pub aid: i64,
    ///
    #[prost(int64, tag = "2")]
    pub count: i64,
    ///
    #[prost(int32, tag = "3")]
    pub selected: i32,
    ///
    #[prost(bool, tag = "4")]
    pub show_count: bool,
    ///
    #[prost(message, optional, tag = "5")]
    pub like_resource: ::core::option::Option<LikeResource>,
    ///
    #[prost(message, optional, tag = "6")]
    pub like_night_resource: ::core::option::Option<LikeResource>,
    ///
    #[prost(message, optional, tag = "7")]
    pub dislike_resource: ::core::option::Option<LikeResource>,
    ///
    #[prost(message, optional, tag = "8")]
    pub dislike_night_resource: ::core::option::Option<LikeResource>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchLiveCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "3")]
    pub rcmd_reason: ::core::option::Option<RcmdReason>,
    ///
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "5")]
    pub online: i32,
    ///
    #[prost(string, tag = "6")]
    pub badge: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub live_link: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub card_left_text: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "9")]
    pub card_left_icon: i32,
    ///
    #[prost(string, tag = "10")]
    pub show_card_desc2: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "11")]
    pub right_top_live_badge: ::core::option::Option<RightTopLiveBadge>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchLiveInlineCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub mid: i64,
    ///
    #[prost(message, optional, tag = "4")]
    pub rcmd_reason_style: ::core::option::Option<ReasonStyle>,
    ///
    #[prost(int64, tag = "5")]
    pub roomid: i64,
    ///
    #[prost(string, tag = "6")]
    pub live_link: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "7")]
    pub live_room_inline: ::core::option::Option<SearchInlineData>,
    ///
    #[prost(string, tag = "8")]
    pub inline_type: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchNewChannelCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub id: i64,
    ///
    #[prost(string, tag = "4")]
    pub type_icon: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "5")]
    pub channel_label1: ::core::option::Option<ChannelLabel>,
    ///
    #[prost(message, optional, tag = "6")]
    pub channel_label2: ::core::option::Option<ChannelLabel>,
    ///
    #[prost(message, optional, tag = "7")]
    pub channel_button: ::core::option::Option<ChannelLabel>,
    ///
    #[prost(string, tag = "8")]
    pub design_type: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "9")]
    pub items: ::prost::alloc::vec::Vec<ChannelMixedItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchNoResultSuggestWordCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "3")]
    pub sug_key_word_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchOgvCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub sub_title1: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub sub_title2: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub bg_cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub special_bg_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub cover_uri: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchOgvChannelCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "3")]
    pub media_id: i64,
    ///
    #[prost(string, tag = "4")]
    pub styles: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub area: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub staff: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub badge: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "8")]
    pub watch_button: ::core::option::Option<WatchButton>,
    ///
    #[prost(double, tag = "9")]
    pub rating: f64,
    ///
    #[prost(string, tag = "10")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "11")]
    pub badges_v2: ::prost::alloc::vec::Vec<ReasonStyle>,
    ///
    #[prost(string, tag = "12")]
    pub styles_v2: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchOgvInlineCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub author: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "4")]
    pub danmaku: i32,
    ///
    #[prost(string, tag = "5")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub face: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub inline_type: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "8")]
    pub mid: i64,
    ///
    #[prost(int64, tag = "9")]
    pub play: i64,
    ///
    #[prost(message, optional, tag = "10")]
    pub ogv_inline: ::core::option::Option<SearchInlineData>,
    ///
    #[prost(message, optional, tag = "11")]
    pub ogv_clip_info: ::core::option::Option<OgvClipInfo>,
    ///
    #[prost(message, optional, tag = "12")]
    pub watch_button: ::core::option::Option<WatchButton>,
    ///
    #[prost(string, tag = "13")]
    pub score: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "14")]
    pub ogv_inline_exp: i32,
    ///
    #[prost(message, repeated, tag = "15")]
    pub badges_v2: ::prost::alloc::vec::Vec<ReasonStyle>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchOgvRecommendCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "3")]
    pub items: ::prost::alloc::vec::Vec<OgvRecommendWord>,
    ///
    #[prost(string, tag = "4")]
    pub special_bg_color: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchOgvRelationCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub special_bg_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub more_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub more_url: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "6")]
    pub items: ::prost::alloc::vec::Vec<DetailsRelationItem>,
    ///
    #[prost(int32, tag = "7")]
    pub is_new_style: i32,
    ///
    #[prost(message, optional, tag = "8")]
    pub ogv_card_ui: ::core::option::Option<OgvCardUi>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchOlympicGameCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "3")]
    pub sports_match_item: ::core::option::Option<SportsMatchItem>,
    ///
    #[prost(message, optional, tag = "4")]
    pub match_top: ::core::option::Option<MatchItem>,
    ///
    #[prost(string, tag = "5")]
    pub bg_cover: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "6")]
    pub extra_link: ::prost::alloc::vec::Vec<ExtraLink>,
    ///
    #[prost(string, tag = "7")]
    pub inline_type: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "8")]
    pub ugc_inline: ::core::option::Option<SearchInlineData>,
    ///
    #[prost(message, optional, tag = "9")]
    pub live_room_inline: ::core::option::Option<SearchInlineData>,
    ///
    #[prost(message, optional, tag = "10")]
    pub match_bottom: ::core::option::Option<MatchItem>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchOlympicWikiCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "3")]
    pub card_business_badge: ::core::option::Option<CardBusinessBadge>,
    ///
    #[prost(message, optional, tag = "4")]
    pub read_more: ::core::option::Option<NavigationButton>,
    ///
    #[prost(string, tag = "5")]
    pub inline_type: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "6")]
    pub ugc_inline: ::core::option::Option<SearchInlineData>,
    ///
    #[prost(message, optional, tag = "7")]
    pub live_room_inline: ::core::option::Option<SearchInlineData>,
    ///
    #[prost(message, optional, tag = "8")]
    pub pedia_cover: ::core::option::Option<PediaCover>,
    ///
    #[prost(message, repeated, tag = "9")]
    pub navigation: ::prost::alloc::vec::Vec<Navigation>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchPediaCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "3")]
    pub navigation: ::prost::alloc::vec::Vec<Navigation>,
    ///
    #[prost(message, optional, tag = "4")]
    pub read_more: ::core::option::Option<NavigationButton>,
    ///
    #[prost(int32, tag = "5")]
    pub navigation_module_count: i32,
    ///
    #[prost(message, optional, tag = "6")]
    pub pedia_cover: ::core::option::Option<PediaCover>,
    ///
    #[prost(message, optional, tag = "7")]
    pub card_business_badge: ::core::option::Option<CardBusinessBadge>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchPurchaseCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub badge: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub venue: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "5")]
    pub price: i32,
    ///
    #[prost(string, tag = "6")]
    pub price_complete: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "7")]
    pub price_type: i32,
    ///
    #[prost(int32, tag = "8")]
    pub required_number: i32,
    ///
    #[prost(string, tag = "9")]
    pub city: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "10")]
    pub show_time: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "11")]
    pub id: i64,
    ///
    #[prost(string, tag = "12")]
    pub shop_name: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchRecommendTipCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchRecommendWordCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "3")]
    pub list: ::prost::alloc::vec::Vec<RecommendWord>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchSpecialCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "3")]
    pub new_rec_tags: ::prost::alloc::vec::Vec<ReasonStyle>,
    ///
    #[prost(message, optional, tag = "4")]
    pub card_business_badge: ::core::option::Option<CardBusinessBadge>,
    ///
    #[prost(string, tag = "5")]
    pub badge: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "7")]
    pub new_rec_tags_v2: ::prost::alloc::vec::Vec<ReasonStyle>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchSpecialGuideCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub phone: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub desc: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchSportCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub bg_cover: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "4")]
    pub match_top: ::core::option::Option<MatchItem>,
    ///
    #[prost(message, optional, tag = "5")]
    pub match_bottom: ::core::option::Option<MatchItem>,
    ///
    #[prost(message, repeated, tag = "6")]
    pub extra_link: ::prost::alloc::vec::Vec<ExtraLink>,
    ///
    #[prost(message, repeated, tag = "7")]
    pub items: ::prost::alloc::vec::Vec<MatchInfoObj>,
    ///
    #[prost(int64, tag = "8")]
    pub id: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchSportInlineCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub bg_cover: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "4")]
    pub match_top: ::core::option::Option<MatchItem>,
    ///
    #[prost(message, optional, tag = "5")]
    pub match_bottom: ::core::option::Option<MatchItem>,
    ///
    #[prost(message, repeated, tag = "6")]
    pub extra_link: ::prost::alloc::vec::Vec<ExtraLink>,
    ///
    #[prost(message, repeated, tag = "7")]
    pub items: ::prost::alloc::vec::Vec<MatchInfoObj>,
    ///
    #[prost(int64, tag = "8")]
    pub id: i64,
    ///
    #[prost(message, optional, tag = "9")]
    pub esports_inline: ::core::option::Option<SearchInlineData>,
    ///
    #[prost(string, tag = "10")]
    pub inline_type: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchTipsCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub sub_title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "134")]
    pub cover_night: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchTopGameCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "3")]
    pub array: i32,
    ///
    #[prost(string, tag = "4")]
    pub background_image: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "5")]
    pub button_type: i32,
    ///
    #[prost(string, tag = "6")]
    pub game_icon: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "7")]
    pub game_base_id: i64,
    ///
    #[prost(int32, tag = "8")]
    pub game_status: i32,
    ///
    #[prost(string, tag = "9")]
    pub inline_type: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "10")]
    pub top_game_ui: ::core::option::Option<TopGameUi>,
    ///
    #[prost(string, tag = "11")]
    pub notice_content: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "12")]
    pub notice_name: ::prost::alloc::string::String,
    ///
    #[prost(float, tag = "13")]
    pub rating: f32,
    ///
    #[prost(string, tag = "14")]
    pub score: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "15")]
    pub tab_info: ::prost::alloc::vec::Vec<TabInfo>,
    ///
    #[prost(string, tag = "16")]
    pub tags: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "17")]
    pub ugc_inline: ::core::option::Option<SearchInlineData>,
    ///
    #[prost(string, tag = "18")]
    pub video_cover_image: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "19")]
    pub inline_live: ::core::option::Option<SearchInlineData>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchUgcInlineCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub author: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "4")]
    pub danmaku: i32,
    ///
    #[prost(string, tag = "5")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub inline_type: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "7")]
    pub mid: i64,
    ///
    #[prost(int64, tag = "8")]
    pub play: i64,
    ///
    #[prost(message, optional, tag = "9")]
    pub ugc_inline: ::core::option::Option<SearchInlineData>,
    ///
    #[prost(message, optional, tag = "10")]
    pub full_text: ::core::option::Option<FullTextResult>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchUpperCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub sign: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "4")]
    pub fans: i32,
    ///
    #[prost(int32, tag = "5")]
    pub archives: i32,
    ///
    #[prost(int32, tag = "6")]
    pub live_status: i32,
    ///
    #[prost(int32, tag = "7")]
    pub roomid: i32,
    ///
    #[prost(message, optional, tag = "8")]
    pub official_verify: ::core::option::Option<OfficialVerify>,
    ///
    #[prost(int32, tag = "9")]
    pub face_nft_new: i32,
    ///
    #[prost(message, optional, tag = "10")]
    pub nft_face_icon: ::core::option::Option<NftFaceIcon>,
    ///
    #[prost(message, repeated, tag = "11")]
    pub av_items: ::prost::alloc::vec::Vec<AvItem>,
    ///
    #[prost(bool, tag = "12")]
    pub is_up: bool,
    ///
    #[prost(int32, tag = "13")]
    pub attentions: i32,
    ///
    #[prost(int32, tag = "14")]
    pub level: i32,
    ///
    #[prost(int32, tag = "15")]
    pub is_senior_member: i32,
    ///
    #[prost(message, optional, tag = "16")]
    pub vip: ::core::option::Option<VipInfo>,
    ///
    #[prost(message, optional, tag = "17")]
    pub relation: ::core::option::Option<Relation>,
    ///
    #[prost(string, tag = "18")]
    pub live_link: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "19")]
    pub notice: ::core::option::Option<Notice>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchVideoCard {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "3")]
    pub rcmd_reason: ::core::option::Option<RcmdReason>,
    ///
    #[prost(message, repeated, tag = "4")]
    pub new_rec_tags: ::prost::alloc::vec::Vec<ReasonStyle>,
    ///
    #[prost(message, repeated, tag = "5")]
    pub three_point: ::prost::alloc::vec::Vec<ThreePoint>,
    ///
    #[prost(message, optional, tag = "6")]
    pub share: ::core::option::Option<Share>,
    ///
    #[prost(message, optional, tag = "7")]
    pub card_business_badge: ::core::option::Option<CardBusinessBadge>,
    ///
    #[prost(int32, tag = "8")]
    pub play: i32,
    ///
    #[prost(int32, tag = "9")]
    pub danmaku: i32,
    ///
    #[prost(string, tag = "10")]
    pub author: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "11")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "12")]
    pub duration: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "13")]
    pub badges: ::prost::alloc::vec::Vec<ReasonStyle>,
    ///
    #[prost(int64, tag = "14")]
    pub mid: i64,
    ///
    #[prost(string, tag = "15")]
    pub show_card_desc1: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "16")]
    pub show_card_desc2: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "17")]
    pub full_text: ::core::option::Option<FullTextResult>,
    ///
    #[prost(message, repeated, tag = "18")]
    pub new_rec_tags_v2: ::prost::alloc::vec::Vec<ReasonStyle>,
    ///
    #[prost(message, repeated, tag = "19")]
    pub badges_v2: ::prost::alloc::vec::Vec<ReasonStyle>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Share {
    ///
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "2")]
    pub video: ::core::option::Option<Video>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShareButtonItem {
    ///
    #[prost(int32, tag = "1")]
    pub r#type: i32,
    ///
    #[prost(message, repeated, tag = "2")]
    pub button_metas: ::prost::alloc::vec::Vec<ButtonMeta>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SharePlane {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub share_subtitle: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub desc: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "5")]
    pub aid: i64,
    ///
    #[prost(string, tag = "6")]
    pub bvid: ::prost::alloc::string::String,
    ///
    #[prost(message, optional, tag = "7")]
    pub share_to: ::core::option::Option<ShareTo>,
    ///
    #[prost(string, tag = "8")]
    pub author: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "9")]
    pub author_id: i64,
    ///
    #[prost(string, tag = "10")]
    pub short_link: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "11")]
    pub play_number: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "12")]
    pub room_id: i64,
    ///
    #[prost(int32, tag = "13")]
    pub ep_id: i32,
    ///
    #[prost(string, tag = "14")]
    pub area_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "15")]
    pub author_face: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "16")]
    pub season_id: i32,
    ///
    #[prost(string, tag = "17")]
    pub share_from: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "18")]
    pub season_title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "19")]
    pub from: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShareTo {
    ///
    #[prost(bool, tag = "1")]
    pub dynamic: bool,
    ///
    #[prost(bool, tag = "2")]
    pub im: bool,
    ///
    #[prost(bool, tag = "3")]
    pub copy: bool,
    ///
    #[prost(bool, tag = "4")]
    pub more: bool,
    ///
    #[prost(bool, tag = "5")]
    pub wechat: bool,
    ///
    #[prost(bool, tag = "6")]
    pub weibo: bool,
    ///
    #[prost(bool, tag = "7")]
    pub wechat_monment: bool,
    ///
    #[prost(bool, tag = "8")]
    pub qq: bool,
    ///
    #[prost(bool, tag = "9")]
    pub qzone: bool,
    ///
    #[prost(bool, tag = "10")]
    pub facebook: bool,
    ///
    #[prost(bool, tag = "11")]
    pub line: bool,
    ///
    #[prost(bool, tag = "12")]
    pub messenger: bool,
    ///
    #[prost(bool, tag = "13")]
    pub whats_app: bool,
    ///
    #[prost(bool, tag = "14")]
    pub twitter: bool,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Space {
    ///
    #[prost(int32, tag = "1")]
    pub show: i32,
    ///
    #[prost(string, tag = "2")]
    pub text_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub text_color_night: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub space_url: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SportsMatchItem {
    ///
    #[prost(int64, tag = "1")]
    pub match_id: i64,
    ///
    #[prost(int64, tag = "2")]
    pub season_id: i64,
    ///
    #[prost(string, tag = "3")]
    pub match_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub img: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub begin_time_desc: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub match_status_desc: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub sub_content: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "8")]
    pub sub_extra_icon: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Stat {
    ///
    #[prost(int32, tag = "1")]
    pub play: i32,
    ///
    #[prost(int32, tag = "2")]
    pub like: i32,
    ///
    #[prost(int32, tag = "3")]
    pub reply: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TabInfo {
    ///
    #[prost(string, tag = "1")]
    pub tab_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub tab_url: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "3")]
    pub sort: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextButton {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextLabel {
    ///
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Texts {
    ///
    #[prost(string, tag = "1")]
    pub booking_text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub unbooking_text: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreePoint {
    ///
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreePoint2 {
    ///
    #[prost(message, repeated, tag = "1")]
    pub dislike_reasons: ::prost::alloc::vec::Vec<DislikeReason>,
    ///
    #[prost(message, repeated, tag = "2")]
    pub feedbacks: ::prost::alloc::vec::Vec<DislikeReason>,
    ///
    #[prost(int32, tag = "3")]
    pub watch_later: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreePointV2 {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub subtitle: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "3")]
    pub reasons: ::prost::alloc::vec::Vec<DislikeReason>,
    ///
    #[prost(string, tag = "4")]
    pub r#type: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "5")]
    pub id: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreePointV3 {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub selected_title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub subtitle: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "4")]
    pub reasons: ::prost::alloc::vec::Vec<DislikeReason>,
    ///
    #[prost(string, tag = "5")]
    pub r#type: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "6")]
    pub id: i64,
    ///
    #[prost(int32, tag = "7")]
    pub selected: i32,
    ///
    #[prost(string, tag = "8")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "9")]
    pub selected_icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "10")]
    pub url: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "11")]
    pub default_id: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreePointV4 {
    ///
    #[prost(message, optional, tag = "1")]
    pub share_plane: ::core::option::Option<SharePlane>,
    ///
    #[prost(message, optional, tag = "2")]
    pub watch_later: ::core::option::Option<WatchLater>,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopGameUi {
    ///
    #[prost(string, tag = "1")]
    pub background_image: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub cover_default_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub gaussian_blur_value: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub mask_color_value: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub mask_opacity: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub module_color: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrafficConfig {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(message, repeated, tag = "2")]
    pub options: ::prost::alloc::vec::Vec<TrafficConfigOption>,
    ///
    #[prost(int64, tag = "3")]
    pub default_option_id: i64,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrafficConfigOption {
    ///
    #[prost(int32, tag = "1")]
    pub id: i32,
    ///
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpArgs {
    ///
    #[prost(int64, tag = "1")]
    pub up_id: i64,
    ///
    #[prost(string, tag = "2")]
    pub up_name: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub up_face: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "4")]
    pub selected: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Upper {
    ///
    #[prost(int64, tag = "1")]
    pub mid: i64,
    ///
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub cover: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub ptime_text: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Video {
    ///
    #[prost(string, tag = "1")]
    pub bvid: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "2")]
    pub cid: i64,
    ///
    #[prost(string, tag = "3")]
    pub share_subtitle: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "4")]
    pub is_hot_label: bool,
    ///
    #[prost(int32, tag = "5")]
    pub page: i32,
    ///
    #[prost(int32, tag = "6")]
    pub page_count: i32,
    ///
    #[prost(string, tag = "7")]
    pub short_link: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VipInfo {
    ///
    #[prost(int32, tag = "1")]
    pub r#type: i32,
    ///
    #[prost(int32, tag = "2")]
    pub status: i32,
    ///
    #[prost(int64, tag = "3")]
    pub due_date: i64,
    ///
    #[prost(int32, tag = "4")]
    pub vip_pay_type: i32,
    ///
    #[prost(int32, tag = "5")]
    pub theme_type: i32,
    ///
    #[prost(message, optional, tag = "6")]
    pub label: ::core::option::Option<VipLabel>,
    ///
    #[prost(int32, tag = "7")]
    pub avatar_subscript: i32,
    ///
    #[prost(string, tag = "8")]
    pub nickname_color: ::prost::alloc::string::String,
    ///
    #[prost(int64, tag = "9")]
    pub role: i64,
    ///
    #[prost(string, tag = "10")]
    pub avatar_subscript_url: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "11")]
    pub tv_vip_status: i32,
    ///
    #[prost(int32, tag = "12")]
    pub tv_vip_pay_type: i32,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VipLabel {
    ///
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "3")]
    pub label_theme: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub text_color: ::prost::alloc::string::String,
    ///
    #[prost(int32, tag = "5")]
    pub bg_style: i32,
    ///
    #[prost(string, tag = "6")]
    pub bg_color: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub border_color: ::prost::alloc::string::String,
    ///
    #[prost(bool, tag = "8")]
    pub use_img_label: bool,
    ///
    #[prost(string, tag = "9")]
    pub img_label_uri_hans: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "10")]
    pub img_label_uri_hant: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "11")]
    pub img_label_uri_hans_static: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "12")]
    pub img_label_uri_hant_static: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WatchButton {
    ///
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub link: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WatchedShow {
    ///
    #[prost(bool, tag = "1")]
    pub switch: bool,
    ///
    #[prost(int64, tag = "2")]
    pub num: i64,
    ///
    #[prost(string, tag = "3")]
    pub text_small: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "4")]
    pub text_large: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "5")]
    pub icon: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "6")]
    pub icon_location: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "7")]
    pub icon_web: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WatchLater {
    ///
    #[prost(int64, tag = "1")]
    pub aid: i64,
    ///
    #[prost(string, tag = "2")]
    pub bvid: ::prost::alloc::string::String,
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CategorySort {
    ///
    Default = 0,
    ///
    PublishTime = 1,
    ///
    ClickCount = 2,
    ///
    CommentCount = 3,
    ///
    LikeCount = 4,
}
impl CategorySort {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CategorySort::Default => "CATEGORY_SORT_DEFAULT",
            CategorySort::PublishTime => "CATEGORY_SORT_PUBLISH_TIME",
            CategorySort::ClickCount => "CATEGORY_SORT_CLICK_COUNT",
            CategorySort::CommentCount => "CATEGORY_SORT_COMMENT_COUNT",
            CategorySort::LikeCount => "CATEGORY_SORT_LIKE_COUNT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CATEGORY_SORT_DEFAULT" => Some(Self::Default),
            "CATEGORY_SORT_PUBLISH_TIME" => Some(Self::PublishTime),
            "CATEGORY_SORT_CLICK_COUNT" => Some(Self::ClickCount),
            "CATEGORY_SORT_COMMENT_COUNT" => Some(Self::CommentCount),
            "CATEGORY_SORT_LIKE_COUNT" => Some(Self::LikeCount),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Sort {
    ///
    Default = 0,
    ///
    ViewCount = 1,
    ///
    PublishTime = 2,
    ///
    DanmakuCount = 3,
}
impl Sort {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Sort::Default => "SORT_DEFAULT",
            Sort::ViewCount => "SORT_VIEW_COUNT",
            Sort::PublishTime => "SORT_PUBLISH_TIME",
            Sort::DanmakuCount => "SORT_DANMAKU_COUNT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SORT_DEFAULT" => Some(Self::Default),
            "SORT_VIEW_COUNT" => Some(Self::ViewCount),
            "SORT_PUBLISH_TIME" => Some(Self::PublishTime),
            "SORT_DANMAKU_COUNT" => Some(Self::DanmakuCount),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UserSort {
    ///
    Default = 0,
    ///
    FansDescend = 1,
    ///
    FansAscend = 2,
    ///
    LevelDescend = 3,
    ///
    LevelAscend = 4,
}
impl UserSort {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UserSort::Default => "USER_SORT_DEFAULT",
            UserSort::FansDescend => "USER_SORT_FANS_DESCEND",
            UserSort::FansAscend => "USER_SORT_FANS_ASCEND",
            UserSort::LevelDescend => "USER_SORT_LEVEL_DESCEND",
            UserSort::LevelAscend => "USER_SORT_LEVEL_ASCEND",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "USER_SORT_DEFAULT" => Some(Self::Default),
            "USER_SORT_FANS_DESCEND" => Some(Self::FansDescend),
            "USER_SORT_FANS_ASCEND" => Some(Self::FansAscend),
            "USER_SORT_LEVEL_DESCEND" => Some(Self::LevelDescend),
            "USER_SORT_LEVEL_ASCEND" => Some(Self::LevelAscend),
            _ => None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UserType {
    ///
    All = 0,
    ///
    Up = 1,
    ///
    NormalUser = 2,
    ///
    AuthenticatedUser = 3,
}
impl UserType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UserType::All => "ALL",
            UserType::Up => "UP",
            UserType::NormalUser => "NORMAL_USER",
            UserType::AuthenticatedUser => "AUTHENTICATED_USER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ALL" => Some(Self::All),
            "UP" => Some(Self::Up),
            "NORMAL_USER" => Some(Self::NormalUser),
            "AUTHENTICATED_USER" => Some(Self::AuthenticatedUser),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod search_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///
    #[derive(Debug, Clone)]
    pub struct SearchClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SearchClient<tonic::transport::Channel> {
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
    impl<T> SearchClient<T>
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
        ) -> SearchClient<InterceptedService<T, F>>
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
            SearchClient::new(InterceptedService::new(inner, interceptor))
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
        /// 搜索所有类型结果
        pub async fn search_all(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchAllRequest>,
        ) -> Result<tonic::Response<super::SearchAllResponse>, tonic::Status> {
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
                "/bilibili.polymer.app.search.v1.Search/SearchAll",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// 搜索指定类型结果
        pub async fn search_by_type(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchByTypeRequest>,
        ) -> Result<tonic::Response<super::SearchByTypeResponse>, tonic::Status> {
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
                "/bilibili.polymer.app.search.v1.Search/SearchByType",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn search_comic(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchComicRequest>,
        ) -> Result<tonic::Response<super::SearchComicResponse>, tonic::Status> {
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
                "/bilibili.polymer.app.search.v1.Search/SearchComic",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod search_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with SearchServer.
    #[async_trait]
    pub trait Search: Send + Sync + 'static {
        /// 搜索所有类型结果
        async fn search_all(
            &self,
            request: tonic::Request<super::SearchAllRequest>,
        ) -> Result<tonic::Response<super::SearchAllResponse>, tonic::Status>;
        /// 搜索指定类型结果
        async fn search_by_type(
            &self,
            request: tonic::Request<super::SearchByTypeRequest>,
        ) -> Result<tonic::Response<super::SearchByTypeResponse>, tonic::Status>;
        ///
        async fn search_comic(
            &self,
            request: tonic::Request<super::SearchComicRequest>,
        ) -> Result<tonic::Response<super::SearchComicResponse>, tonic::Status>;
    }
    ///
    #[derive(Debug)]
    pub struct SearchServer<T: Search> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Search> SearchServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SearchServer<T>
    where
        T: Search,
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
                "/bilibili.polymer.app.search.v1.Search/SearchAll" => {
                    #[allow(non_camel_case_types)]
                    struct SearchAllSvc<T: Search>(pub Arc<T>);
                    impl<T: Search> tonic::server::UnaryService<super::SearchAllRequest>
                    for SearchAllSvc<T> {
                        type Response = super::SearchAllResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SearchAllRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).search_all(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SearchAllSvc(inner);
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
                "/bilibili.polymer.app.search.v1.Search/SearchByType" => {
                    #[allow(non_camel_case_types)]
                    struct SearchByTypeSvc<T: Search>(pub Arc<T>);
                    impl<
                        T: Search,
                    > tonic::server::UnaryService<super::SearchByTypeRequest>
                    for SearchByTypeSvc<T> {
                        type Response = super::SearchByTypeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SearchByTypeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).search_by_type(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SearchByTypeSvc(inner);
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
                "/bilibili.polymer.app.search.v1.Search/SearchComic" => {
                    #[allow(non_camel_case_types)]
                    struct SearchComicSvc<T: Search>(pub Arc<T>);
                    impl<
                        T: Search,
                    > tonic::server::UnaryService<super::SearchComicRequest>
                    for SearchComicSvc<T> {
                        type Response = super::SearchComicResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SearchComicRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).search_comic(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SearchComicSvc(inner);
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
    impl<T: Search> Clone for SearchServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Search> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Search> tonic::server::NamedService for SearchServer<T> {
        const NAME: &'static str = "bilibili.polymer.app.search.v1.Search";
    }
}
