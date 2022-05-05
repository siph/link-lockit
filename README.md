# url-wrapper
Url-wrapper is a functional web-application demo project built using the rust framework [Axum](https://github.com/tokio-rs/axum/).
Url-wrapper accepts and processes encoded urls through the external services of [exe.io](exe.io) and stores them in a postgres database.

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
