# appms
app manage server


## structure
```text
src
    server web处理
    utils 工具类
    db 数据库
    cmd 外部命令
```


## 开发计划
### 第一期
* 实现基本增删改查
* 建表、数据操纵
* 开放http api
  * 创建新版本
  * 查询版本列表
  * 查询项目列表
  * 查询增量文件信息
  * 

### 第二期
* 外部命令调用
  * 独立线程执行，将进度写入数据库
  * 页面可实时查询执行进度
* 脚本
  * 对接jenkins api
  * 从ftp或者scp下载生成的文件
  * 从数据库读取版本号

* 将生效版本写入后端业务数据库