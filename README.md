# Edgeless Backend V3

全新的完全版本 Edgeless 后端服务器程序。此版本为 Edgeless 服务端的完全实现版本，遵守 `Edgeless Backend Protocol V3` 协议，提供全部的 Edgeless 服务。

## 安放测试目录

解压 `test.7z` 到项目根目录，然后执行 `cp config.toml.example config.toml`

## 启动

- 编辑 `config.toml` 然后运行 `cargo run` 进行测试。
- 若显示`Info:Config validation passed` 则说明已通过配置校验，然后请求 `localhost:8383/api/v3/hello` 查看是否正常响应。
- 若没有问题即可执行 `cargo build --release` 编译生产环境版本并部署。

## REST API

- `/api/v3/hello` 镜像站挥手接口

    单次请求即可获取当前镜像站的所有服务信息，减少频繁的网络请求。

    通过 `services` 字段标识此节点提供的所有 Edgeless 服务，然后通过对应字段提供服务信息。

    响应示例：

  ```json
  {
    "name": "菠萝云",
    "description": "Edgeless 官方主站点",
    "protocol": "3.1.0",
    "root": "https://pineapple.edgeless.top/",
    "property": {
      "domestic_server": true,
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
        "name": "kernel",
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
        "安装备份": [
          {
            "name": "DriveSnapShot_1.48_shiftz.7z",
            "size": 444722,
            "timestamp": 1628659081,
            "integrity": {
              "method": "blake3",
              "value": "5e40ce441dc6dcb728333b475a719b7b47c1b65c5850f8d26619015b8eebdb70"
            }
          }
        ],
        "安全急救": [
          {
            "name": "360CAD病毒专杀_1.0.0.0_汪凯.7z",
            "size": 740267,
            "timestamp": 1587457392,
            "integrity": {
              "method": "blake3",
              "value": "adf4313a13580bb4f9bed1bfad770600b713ef95f5cbaa6fb0f486f01652afbb"
            }
          }
        ]
      },
      "path": "https://pineapple.edgeless.top/disk/插件包/"
    },
    "kernel": {
      "name": "Edgeless_Beta_4.1.0.iso",
      "version": "4.1.0",
      "url": "https://pineapple.edgeless.top/disk/Socket/Edgeless_Beta_4.1.0.iso",
      "size": 4,
      "timestamp": 1614539928,
      "integrity": {
        "method": "blake3",
        "value": "865c4e216f723a9c95e689d976c605ba8f6202c6547b21688843771b74daabc9"
      }
    },
    "ventoy": {
      "windows": {
        "name": "ventoy-1.0.79-windows.zip",
        "version": "1.0.79",
        "url": "https://pineapple.edgeless.top/disk/Socket/Ventoy/ventoy-1.0.79-windows.zip",
        "size": 4,
        "timestamp": 1614539928,
        "integrity": {
          "method": "blake3",
          "value": "865c4e216f723a9c95e689d976c605ba8f6202c6547b21688843771b74daabc9"
        }
      },
      "linux": {
        "name": "ventoy-1.0.79-linux.tar.gz",
        "version": "1.0.79",
        "url": "https://pineapple.edgeless.top/disk/Socket/Ventoy/ventoy-1.0.79-linux.tar.gz",
        "size": 4,
        "timestamp": 1614539928,
        "integrity": {
          "method": "blake3",
          "value": "865c4e216f723a9c95e689d976c605ba8f6202c6547b21688843771b74daabc9"
        }
      },
      "plugin": {
        "name": "ventoy_wimboot.img",
        "version": "0.0.0",
        "url": "https://pineapple.edgeless.top/disk/Socket/Ventoy/ventoy_wimboot.img",
        "size": 4,
        "timestamp": 1614539928,
        "integrity": {
          "method": "blake3",
          "value": "865c4e216f723a9c95e689d976c605ba8f6202c6547b21688843771b74daabc9"
        }
      }
    },
    "hub": {
      "latest": {
        "version": "2.27",
        "page": "https://down.edgeless.top/"
      },
      "update": {
        "allow_normal_since": "3.0",
        "force_update_until": "3.0",
        "wide_gaps": ["3.0"]
      },
      "notices": [
        {
          "id": "220723",
          "channel": "Hub",
          "level": "info",
          "message": "通知标题",
          "description": "通知内容",
          "close_text": "关闭",
          "lower_than": "2.28",
          "repeat_after": 3
        }
      ],
      "packages": {
        "update": {
          "name": "update.7z",
          "version": "0.0.0",
          "url": "https://pineapple.edgeless.top/disk/Socket/Hub/update.7z",
          "size": 66,
          "timestamp": 1658507364,
          "integrity": {
            "method": "blake3",
            "value": "7d536353c0a78f6c4c249dddd7d4965b7b493f5c861500002eb91bc20afdb569"
          }
        },
        "extended_update": {
          "name": "extended_update.7z",
          "version": "0.0.0",
          "url": "https://pineapple.edgeless.top/disk/Socket/Hub/extended_update.7z",
          "size": 66,
          "timestamp": 1658507364,
          "integrity": {
            "method": "blake3",
            "value": "7d536353c0a78f6c4c249dddd7d4965b7b493f5c861500002eb91bc20afdb569"
          }
        },
        "full": {
          "name": "Edgeless Hub_Beta_2.27.7z",
          "version": "2.27",
          "url": "https://pineapple.edgeless.top/disk/Socket/Hub/Edgeless Hub_Beta_2.27.7z",
          "size": 1366,
          "timestamp": 1658512706,
          "integrity": {
            "method": "blake3",
            "value": "d650c3c4ca0b0d2cbd0c6560b95113326e451cb013eb577377d3493f5469ed3f"
          }
        }
      }
    }
  }
  ```

- `/api/v3/alpha?token={TOKEN}` Alpha 内测信息获取接口

    请求时需要携带 Alpha 用户令牌。响应 HTTP 状态码 `400` 时说明请求时未携带 TOKEN 或 TOKEN 无效。

    响应示例：

      ```json
      {
        "kernel_wim": {
          "name": "Edgeless_Alpha_4.1.2.wim",
          "version": "4.1.2",
          "url": "https://pineapple.edgeless.top/disk/Socket/Alpha/Edgeless_Alpha_4.1.2.wim",
          "size": 4,
          "timestamp": 1614539928,
          "integrity": {
            "method": "blake3",
            "value": "865c4e216f723a9c95e689d976c605ba8f6202c6547b21688843771b74daabc9"
          }
        },
        "cover": {
          "lower_than": "4.1.0",
          "file": {
            "name": "cover.7z",
            "version": "0.0.0",
            "url": "https://pineapple.edgeless.top/disk/Socket/Alpha/cover.7z",
            "size": 166,
            "timestamp": 1662883307,
            "integrity": {
              "method": "blake3",
              "value": "5eda7888db0a66ed76af8b159fe6f33e31ff74c0bc1e7459c9c9faf952e1b88d"
            }
          }
        }
      }
      ```

- `/api/v3/refresh?token={TOKEN}` 缓存刷新请求接口

    请求时需要携带超级管理员令牌。响应 HTTP 状态码 `400` 时说明请求时未携带 TOKEN 或 TOKEN 无效；响应 HTTP 状态码 `200` 时说明刷新请求已提交到 Daemon 线程，Daemon 会在空闲时调度刷新。

## 运行时产生的文件

- `hash_map(_METHOD)?.bin` ept 包的 SHA256 缓存，一般不建议删除
- `default-YYYY-MM-DD.log.toml` 程序运行日志，程序会自动清理 7 天之前的日志，一般不需要手动删除
