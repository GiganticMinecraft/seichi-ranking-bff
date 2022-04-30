# SeichiRankingBFF

[整地鯖ランキング](https://ranking-gigantic.seichi.click/)を[PHP](https://github.com/GiganticMinecraft/SeichiRanking)のバックエンド部分を分離してRustで再実装しているレポジトリです。

## 環境変数

| 名前            | 必要性          | 説明           |
|---------------|--------------|--------------|
| `DB_HOST`     | **required** | データベースのホスト   |
| `DB_PORT`     | **required** | データベースのポート   |
| `DB_USER`     | **required** | データベースのユーザー  |
| `DB_PASSWORD` | **required** | データベースのパスワード |

| 名前          | 必要性          | 説明                 |
|-------------|--------------|--------------------|
| `HTTP_PORT` | **required** | HTTPリクエストを受け付けるポート |

