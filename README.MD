## 使用rust构建一个api的模板项目,方便后续自己的使用

### 目标

1. 登录和添加账户
2. 基本的权限管理
3. 分类模块
4. 系统配置
5. 文件上传/下载

### 技术栈

* api框架:使用axum,相关的运行时tokio
* 序列化工具serde serde_json serde_toml
* 数据库:sqlx

### 目录结构

```
    app  应用目录(一个项目提供多个应用)
        admins 
    initialize  项目初始化相关
```

### 任务列表

1. 先构建一个路由体系，将项目跑起来
2. 使用数据库