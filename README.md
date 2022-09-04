# Guava

A clean architecture web framework base on axum. A fast development framework too.

refer to [rust web application clean architecture](https://kerkour.com/rust-web-application-clean-architecture)

axum + rbatis

# Dev steps

update

1. add req_vo and res_vo in server/api/model.rs
2. add input in service/$name_service.rs
3. add service_fn in service/$name_service.rs
4. add repo_fn in service/$name_repo.rs


# performance

Windows 10
CPU 4800H 8 cores 
RAM 64G

sample no cache crud with pg

query: 18000+/qps
update: 3000+/qps
add: 8000+ /qps
delete: 8000+/qps

sample query with cache

query: 58000+/qps