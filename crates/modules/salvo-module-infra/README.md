- 创建数据库
    ```postgresql
        create schema infra;
        comment on schema infra is '基础设施';
        alter schema infra owner to ruoyivuerust;
    ```