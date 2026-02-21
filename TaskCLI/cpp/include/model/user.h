// include/model/user.h
#pragma once

#include <string>
#include <chrono>
#include <nlohmann/json.hpp>

using json = nlohmann::json;

class User {
public:
    uint32_t id;
    std::string name;
    std::chrono::system_clock::time_point created_at;
    
    User(uint32_t id, const std::string& name);
    
    json to_json() const;
    static User from_json(const json& j);
};
