
- 生成 dev entity
  ```shell
    sea-orm-cli generate entity -s infra --with-serde both --model-extra-attributes 'serde(rename_all="camelCase")' --date-time-crate chrono -o ./crates/entities/salvo-entity-infra/src/entity
  ```