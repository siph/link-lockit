# link-locket
Link-locket is a functional web-application demo project built using the rust framework [Axum](https://github.com/tokio-rs/axum/).
Link-locket accepts and processes encoded urls through the external services of exe.io and stores them in a postgres database.   
Some of the technologies demonstrated in url-wrapper:
- Web framework:                    [Axum](https://github.com/tokio-rs/axum/)
- Database:                         [PostgreSQL](https://www.postgresql.org/)
- Object mapping:                   [Sea-Orm](https://github.com/SeaQL/sea-orm)
- Database schema management:       [Sea-Schema](https://github.com/SeaQL/sea-schema)
- OpenAPI documentation generation: [Utoipa](https://github.com/juhaku/utoipa)
- Swagger UI generation:            [Utoipa-Swagger-UI](https://docs.rs/utoipa-swagger-ui/latest/utoipa_swagger_ui/)


# Deployment
## Docker
The easiest way to deploy url-wrapper is to have docker-compose installed and run the start script.
```bash
git clone 'https://gitlab.com/xsiph/url-wrapper.git'
```
```bash
$ ./start.sh
```
By default the server can be reached though port 8081.
Swagger documentation can be found at 'http://localhost:8081/swagger-ui'

