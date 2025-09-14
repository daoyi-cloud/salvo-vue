
- 生成 dev entity
  ```shell
    sea-orm-cli generate entity -s system --with-serde both --model-extra-attributes 'serde(rename_all="camelCase")' --date-time-crate chrono -o ./crates/entities/salvo-entity-system/src/entity
  ```