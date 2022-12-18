use super::grpc_api::{
    grpc_headers,
    grpc_metadata::{self, Metadata},
    grpc_playurl, grpc_search,
};

use super::types::{PlayurlParams, SearchParams};

const GRPC_API_HOST: &'static str = "https://grpc.biliapi.net";

// 参考资料: https://blog.seeflower.dev/archives/4/  => https://github.com/SeeFlowerX/bilibili-grpc-api 可以尝试转写python代码过来
// 尤其注意: gzip数据如此一来除了前两位之外的部分都有可能被用来做风控
// TODO:
// 抓包查看实际请求是个啥样子
// 完成client端
// 编写自定义server端proto, 完成server端
// 暂时不适配东南亚区域(砍了)
// 暂时不管概念版, 国际版, play版
fn gen_metadata(params: &PlayurlParams) -> Metadata {
    let metadata = grpc_metadata::Metadata {
        access_key: String::from(params.access_key),
        mobi_app: String::from(params.mobi_app),
        device: String::from(params.device),
        build: params.build.parse().unwrap_or(6800300),
        channel: String::from(params.channel),
        buvid: todo!(),
        platform: String::from(params.platform),
    };
    metadata
}

pub async fn get_upstream_bili_playurl_grpc(params: &PlayurlParams<'_>) {
    let client = grpc_playurl::play_url_client::PlayUrlClient::connect(GRPC_API_HOST)
        .await
        .unwrap();
    // 需要抓包看看下列数据实际情况, 然后去proto文件那里完善一下注释便于开发
    let request_content = grpc_playurl::PlayViewReq {
        epid: todo!(),
        cid: todo!(),
        qn: todo!(),
        fnver: todo!(),
        fnval: todo!(),
        download: todo!(),
        force_host: todo!(),
        fourk: todo!(),
        // spmid: todo!(),
        // from_spmid: todo!(),
        // teenagers_mode: todo!(),
        prefer_codec_type: todo!(),
        // is_preview: todo!(),
        // room_id: todo!(),
        // is_need_view_info: todo!(),
        // scene_control: todo!(),
        // inline_scene: todo!(),
        // material_no: todo!(),
        ..Default::default()
    };
    todo!()
}

pub async fn get_upstream_bili_search_grpc(params: &SearchParams<'_>) {
    let client = grpc_search::search_client::SearchClient::connect(GRPC_API_HOST)
        .await
        .unwrap();
    todo!()
}
