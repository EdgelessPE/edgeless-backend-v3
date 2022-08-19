# Edgeless Backend V3

全新的完全版本 Edgeless 后端服务器程序。此版本为 Edgeless 服务端的完全实现版本，遵守 `Edgeless Backend Protocol V3` 协议，提供全部的 Edgeless 服务。

## REST API

- `/api/v3/hello` 镜像站挥手接口

      单次请求即可获取当前镜像站的所有服务信息，减少频繁的网络请求。

      通过 `services` 字段标识此节点提供的所有 Edgeless 服务，然后通过对应字段提供服务信息。

      响应示例：
      ```json
      {
      "name": "菠萝云",
      "description": "Edgeless 官方主站点",
      "protocol": "3.0.0",
      "root": "https://pineapple.edgeless.top/",
      "property": {
          "native_server": true,
          "upload_bandwidth": 1000,
          "sync_interval": 0,
          "official_maintained": true
      },
      "services": [
          {
              "name": "plugins",
              "path": "disk/插件包/"
          },
          {
              "name": "iso",
              "path": "disk/Socket/"
          },
          {
              "name": "alpha",
              "path": "disk/Socket/Alpha/"
          },
          {
              "name": "ventoy",
              "path": "disk/Socket/Ventoy/"
          },
          {
              "name": "hub",
              "path": "disk/Socket/Hub/"
          }
      ],
      "plugins": {
          "tree": {
              "下载上传": [
                  {
                      "name": "curl_7.84.0.1_undefined（bot）.7z",
                      "size": 5174331,
                      "timestamp": 1656360547,
                      "hash": "afb59d7ed9184bd788b0c5df789329a9558d42e0b9bc4dd1a4198bccd05b9ef8"
                  }
              ],
              "压缩镜像": [
                  {
                      "name": "7-Zip美化版_21.17.0.0_Horatio Shaw.7z",
                      "size": 3347919,
                      "timestamp": 1643440187,
                      "hash": "f30a5b542e8461883e3e08bc1de1f55eebff724b204b8e7d29c3379bf469d8ad"
                  }
              ]
          },
          "path": "disk/插件包/"
      },
      "iso": {
          "version": "4.10.0",
          "file_name": "Edgeless_Beta_4.10.0.iso",
          "url": "https://pineapple.edgeless.top/disk/Socket/Edgeless_Beta_4.10.0.iso"
      },
      "alpha": {
          "wim": {
              "version": "0.0.0",
              "file_name": "Edgeless_Alpha_0.0.0.wim",
              "url": "https://pineapple.edgeless.top/disk/Socket/Alpha/Edgeless_Alpha_0.0.0.wim"
          },
          "cover": {
              "lower_than": "4.1.0",
              "url": "https://pineapple.edgeless.top/disk/Socket/Alpha/cover.7z"
          }
      },
      "ventoy": {
          "version": "1.0.71",
          "file_name": "ventoy-1.0.71-windows.zip",
          "url": "https://pineapple.edgeless.top/disk/Socket/Ventoy/ventoy-1.0.71-windows.zip"
      },
      "hub": {
          "latest": {
              "version": "2.16",
              "page": "https://down.edgeless.top/"
          },
          "update": {
              "allow_normal_since": "3.0",
              "force_update_until": "3.0",
              "wide_gaps": [
                  "3.0"
              ]
          },
          "notices": [
              {
                  "id": "220723",
                  "channel": "Down",
                  "level": "info",
                  "message": "消息标题",
                  "description": "消息内容",
                  "close_text": "我知道了",
                  "lower_than": "0",
                  "repeat_after": 0
              },
              {
                  "id": "220723",
                  "channel": "Hub",
                  "level": "info",
                  "message": "消息标题",
                  "description": "消息内容",
                  "close_text": "不再提示",
                  "lower_than": "2.28",
                  "repeat_after": 30
              }
          ],
          "packages": {
              "update": "https://pineapple.edgeless.top/disk/Socket/Hub/Update/update.7z",
              "extended_update": "https://pineapple.edgeless.top/disk/Socket/Hub/Update/extended_update.7z",
              "full": "https://pineapple.edgeless.top/disk/Socket/Hub/Edgeless Hub_Beta_2.16.7z"
          }
      }
  }
  ```

- `/api/v3/ept/refresh?token={TOKEN}` ept 索引刷新请求接口

    请求时需要携带令牌，分为 Alpha 用户令牌和超级管理员令牌,分别对应 普通刷新调度 和 强制刷新调度。

    响应 HTTP 状态码 `400` 时说明请求时未携带 TOKEN 或 TOKEN 无效；响应 HTTP 状态码 `200` 时说明刷新请求已提交到 Daemon 线程，Daemon 会根据令牌的调度策略调度刷新。

## 运行时产生的文件
- `hash_map.bin` ept 包的 SHA256 缓存，一般不建议删除
- `default-YYYY-MM-DD.log.toml` 程序运行日志，程序会自动清理7天之前的日志，一般不需要手动删除