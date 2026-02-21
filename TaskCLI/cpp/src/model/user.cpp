// src/model/user.cpp

#include "model/user.h"

User::User(uint32_t id, const std::string& name) 
    : id(id), name(name) {
    created_at = std::chrono::system_clock::now();
}

json User::to_json() const {
    return json{
        {"id", id},
        {"name", name},
        {"created_at", std::chrono::system_clock::to_time_t(created_at)}
    };
}

User User::from_json(const json& j) {
    User user(j["id"].get<uint32_t>(), j["name"].get<std::string>());
    user.created_at = std::chrono::system_clock::from_time_t(j["created_at"].get<std::time_t>());
    return user;
}
