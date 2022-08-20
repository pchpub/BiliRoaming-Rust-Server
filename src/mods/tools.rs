use super::types::PlayurlType;

pub fn remove_parameters_playurl(playurl_type: PlayurlType,data: &mut serde_json::Value) -> Result<(),()> {
    match playurl_type {
        PlayurlType::Thailand => {
            if data["code"].as_i64().unwrap() == 0 {
                let items = if let Some(value) = data["data"]["video_info"]["stream_list"].as_array_mut(){
                    value
                }else{
                    return Err(());
                };
                for item in items {
                    item["stream_info"]["need_vip"] = serde_json::Value::Bool(false);
                    item["stream_info"]["need_login"] = serde_json::Value::Bool(false);
                }
                return Ok(());
            }else{
                return Err(());
            }
        },
        PlayurlType::China => {
            if data["code"].as_i64().unwrap() == 0 {
                let items = if let Some(value) = data["support_formats"].as_array_mut(){
                    value
                }else{
                    return Err(());
                };
                for item in items {
                    //item["need_login"] = serde_json::Value::Bool(false);
                    item.as_object_mut().unwrap().remove("need_login");
                    item.as_object_mut().unwrap().remove("need_vip");
                }
                return Ok(());
            }else{
                return Err(());
            }
        },
    }
}