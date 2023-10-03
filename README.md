# 哔哩哔哩黑名单同步工具

用于 bilibili 多个帐号之间的黑名单同步

[blocklist.json](blocklist.json) 收集了包括且不限于：流量营销、剽窃转载、政治狂热、故意引战、心理年龄幼稚等帐号

## 使用方法

1. 编辑 `config.json`，根据帐号填充 sessdata 和 bili_jct，获取方式可参考：[爱发电 - bilibili 获取 sessdata](https://afdian.net/album/b80ef61c626411ea93f352540025c377/b341d694d72c11ea96c952540025c377)
2. `bbs pull` 拉取所有帐号的黑名单
3. `bbs push` 将黑名单同步到所有帐号

## 📄 License

[GNU Affero General Public License v3.0](https://choosealicense.com/licenses/agpl-3.0)
