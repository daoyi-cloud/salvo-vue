### 初始化Salvo项目
  ```shell
  
          salvo new salvo-vue --lang=zh                                                                                                        ✔  54s   21:47:45  ▓▒░
      #  ____    _    _ __     _____
      # / ___|  / \  | |\ \   / / _ \
      # \___ \ / _ \ | | \ \ / / | | |
      #  ___) / ___ \| |__\ V /| |_| |
      # |____/_/   \_\_____\_/  \___/
      #
      #正在检查更新...
      #当前已经是最新版本
      #✔ 选择数据库连接类型 · sea-orm: 🐚 Rust 的异步和动态 ORM
      #✔ 选择数据库类型（选择 sqlite 可直接运行代码，选择其他的需要先修改配置，运行迁移等等，请查看 readme 或相关文档） · postgres - 最受欢迎的优秀的数据库
      #渲染 liquid 文件 "salvo-vue/Cargo.toml.liquid"
      #渲染 liquid 文件 "salvo-vue/README.md.liquid"
      #渲染 liquid 文件 "salvo-vue/config.toml.liquid"
      #渲染 liquid 文件 "salvo-vue/src/error.rs.liquid"
      #渲染 liquid 文件 "salvo-vue/src/hoops/custom_middleware_example.rs.liquid"
      #渲染 liquid 文件 "salvo-vue/src/main.rs.liquid"
      #渲染 liquid 文件 "salvo-vue/views/error_404.html.liquid"
      #渲染 liquid 文件 "salvo-vue/views/hello.html.liquid"
      #渲染 liquid 文件 "salvo-vue/views/login.html.liquid"
      #渲染 liquid 文件 "salvo-vue/views/user_list_frag.html.liquid"
      #渲染 liquid 文件 "salvo-vue/views/user_list_page.html.liquid"
      #渲染 liquid 文件 "salvo-vue/.cursorrules.liquid"
      #渲染 liquid 文件 "salvo-vue/.env.liquid"
      #
      #🎉 项目创建成功!
      #🚀 您现在可以移动到项目目录并开始运行                                                                                                                                                                        
      #✨ 使用命令 cd salvo-vue 进入项目文件夹                                                                                                                                                                       
      #🌪️  请查看 README.md 文件 开始享受你的赛风 (salvo) 之旅
      #😄 最新版的 Salvo 依赖 Rust 版本 1.80。如果编译失败，请尝试使用 `rustup update` 来升级版本。
      #🐼 在根目录下创建了一个.cursorrules 文件，来辅助 Cursor 编辑器理解项目。
      #🌟 如果你使用其他 IDE，请把里面的内容复制到对应的 AI 设置中，或添加到对话上下文中。                                                                                                                          
      #💫 这个文件的内容请根据你的实际情况修改。🍲                                                                                                                                                                  
      #
  
  ```
### 更新Rust版本
  ```toml
  [package]
  name = "salvo-vue"
  version = "0.1.1"
  edition = "2024"
  rust-version = "1.89.0"
  readme = "./README.md"
  ```
### 升级依赖包
  ```shell
      cargo upgrade --incompatible
  ```
  # Running Migrator CLI
  
  - Generate a new migration file
      ```sh
      cargo run -- migrate generate MIGRATION_NAME
      ```
    - Apply all pending migrations
        ```sh
        cargo run
        ```
        ```sh
        cargo run -- up
        ```
    - Apply first 10 pending migrations
        ```sh
        cargo run -- up -n 10
        ```
    - Rollback last applied migrations
        ```sh
        cargo run -- down
        ```
    - Rollback last 10 applied migrations
        ```sh
        cargo run -- down -n 10
        ```
    - Drop all tables from the database, then reapply all migrations
        ```sh
        cargo run -- fresh
        ```
    - Rollback all applied migrations, then reapply all migrations
        ```sh
        cargo run -- refresh
        ```
    - Rollback all applied migrations
        ```sh
        cargo run -- reset
        ```
    - Check the status of all migrations
        ```sh
        cargo run -- status
        ```
