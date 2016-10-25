FROM alpine:latest

ENV WAIT_TIME 5
ENV INFLUXDB_ADDRESS http://localhost:8086
ENV INFLUXDB_USERNAME wsm_rs
ENV INFLUXDB_PASSWORD wsm_rs
ENV INFLUXDB_NAME wsm_rs
ENV WEBSPHERE_ADDRESS http://localhost
ENV WEBSPHERE_USER wsm_rs
ENV WEBSPHERE_PASSWORD wsm_rs

ADD target/x86_64-unknown-linux-musl/release/wsm_rs /
CMD RUST_BACKTRACE=1 /wsm_rs \
    --wait_time $WAIT_TIME \
    --influxdb_address $INFLUXDB_ADDRESS \
    --influxdb_username $INFLUXDB_USERNAME \
    --influxdb_password $INFLUXDB_PASSWORD \
    --influxdb_name $INFLUXDB_NAME \
    --websphere_address $WEBSPHERE_ADDRESS \
    --websphere_user $WEBSPHERE_USER \
    --websphere_password $WEBSPHERE_PASSWORD