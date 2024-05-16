use crate::{net::tools::JsonData, util};

use super::*;

pub async fn on_get_basic_info_cs_req(
    session: &mut PlayerSession,
    _body: &GetBasicInfoCsReq,
) -> Result<()> {
    session
        .send(
            CMD_GET_BASIC_INFO_SC_RSP,
            GetBasicInfoScRsp {
                retcode: 0,
                player_setting_info: Some(PlayerSettingInfo::default()),
                ..Default::default()
            },
        )
        .await
}

pub async fn on_get_hero_basic_type_info_cs_req(
    session: &mut PlayerSession,
    _body: &GetHeroBasicTypeInfoCsReq,
) -> Result<()> {
    let mc = JsonData::load().await.main_character;
    session
        .send(
            CMD_GET_HERO_BASIC_TYPE_INFO_SC_RSP,
            GetHeroBasicTypeInfoScRsp {
                retcode: 0,
                gender: mc.get_gender().into(),
                cur_basic_type: mc.get_type().into(),
                basic_type_info_list:vec![HeroBasicTypeInfo {
                    basic_type: mc.get_type().into(),
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
        .await
}

pub async fn on_player_heart_beat_cs_req(
    session: &mut PlayerSession,
    body: &PlayerHeartBeatCsReq,
) -> Result<()> {
    session
        .send(
            CMD_PLAYER_HEART_BEAT_SC_RSP,
            PlayerHeartBeatScRsp {
                retcode: 0,
                client_time_ms: body.client_time_ms,
                server_time_ms: util::cur_timestamp_ms(),
                download_data: Some(ClientDownloadData {
                    version: 51,
                    time: util::cur_timestamp_ms() as i64,
                    data: rbase64::decode("bG9jYWwgZnVuY3Rpb24gYmV0YV90ZXh0KCkKICAgIGxvY2FsIGdhbWVPYmplY3QgPSBDUy5Vbml0eUVuZ2luZS5HYW1lT2JqZWN0LkZpbmQoIlVJUm9vdC9BYm92ZURpYWxvZy9CZXRhSGludERpYWxvZyhDbG9uZSkiKQogICAgaWYgZ2FtZU9iamVjdCB0aGVuCiAgICAgICAgbG9jYWwgdGV4dENvbXBvbmVudCA9IGdhbWVPYmplY3Q6R2V0Q29tcG9uZW50SW5DaGlsZHJlbih0eXBlb2YoQ1MuUlBHLkNsaWVudC5Mb2NhbGl6ZWRUZXh0KSkKICAgICAgICBpZiB0ZXh0Q29tcG9uZW50IHRoZW4KICAgICAgICAgICAgdGV4dENvbXBvbmVudC50ZXh0ID0gJ1JvYmluU1IgaXMgYSBmcmVlIGFuZCBvcGVuIHNvdXJjZSBzb2Z0d2FyZS4gRm9ya2VkIGJ5IDxjb2xvcj0jNDgwMEYwPlA8L2NvbG9yPjxjb2xvcj0jNUMwMEYwPnU8L2NvbG9yPjxjb2xvcj0jNzAwMEYxPm48L2NvbG9yPjxjb2xvcj0jODUwMEYyPms8L2NvbG9yPjxjb2xvcj0jOTkwMEYzPmw8L2NvbG9yPjxjb2xvcj0jQUQwMEYzPm88L2NvbG9yPjxjb2xvcj0jQzIwMEY0PnI8L2NvbG9yPjxjb2xvcj0jRDYwMEY1PmQ8L2NvbG9yPjxjb2xvcj0jRUEwMEY2PmU8L2NvbG9yPicKICAgICAgICBlbmQKICAgIGVuZAplbmQKCmxvY2FsIGZ1bmN0aW9uIHZlcnNpb25fdGV4dCgpCiAgICBsb2NhbCBnYW1lT2JqZWN0ID0gQ1MuVW5pdHlFbmdpbmUuR2FtZU9iamVjdC5GaW5kKCJWZXJzaW9uVGV4dCIpCiAgICBpZiBnYW1lT2JqZWN0IHRoZW4KICAgICAgICBsb2NhbCB0ZXh0Q29tcG9uZW50ID0gZ2FtZU9iamVjdDpHZXRDb21wb25lbnRJbkNoaWxkcmVuKHR5cGVvZihDUy5SUEcuQ2xpZW50LkxvY2FsaXplZFRleHQpKQogICAgICAgIGlmIHRleHRDb21wb25lbnQgdGhlbgogICAgICAgICAgICB0ZXh0Q29tcG9uZW50LnRleHQgPSAnRm9yIDIuMi41WCwgVG90YWxseSA8Y29sb3I9I2ZmMDAwMD5Ob3QgQmV0YTwvY29sb3I+JwogICAgICAgIGVuZAogICAgZW5kCmVuZAoKbG9jYWwgZnVuY3Rpb24gbWh5X3RleHQob2JqKQogICAgbG9jYWwgZ2FtZU9iamVjdCA9IENTLlVuaXR5RW5naW5lLkdhbWVPYmplY3QuRmluZCgiSURNQVAxIikKICAgIGlmIGdhbWVPYmplY3QgdGhlbgogICAgICAgIGxvY2FsIHRleHRDb21wb25lbnQgPSBnYW1lT2JqZWN0OkdldENvbXBvbmVudEluQ2hpbGRyZW4odHlwZW9mKENTLlJQRy5DbGllbnQuTWVzc2FnZUJveERpYWxvZ1V0aWwpKQogICAgICAgIGlmIHRleHRDb21wb25lbnQgdGhlbgogICAgICAgICAgICB0ZXh0Q29tcG9uZW50LlNob3dBYm92ZURpYWxvZ1RleHQgPSBmYWxzZQogICAgICAgIGVuZAogICAgZW5kCmVuZAoKdmVyc2lvbl90ZXh0KCkKYmV0YV90ZXh0KCkKbWh5X3RleHQoKQ==").unwrap()
                }),
            },
        )
        .await
}
