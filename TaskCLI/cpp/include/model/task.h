// include/model/task.h
#pragma once

#include <string>
#include <vector>
#include <chrono>
#include <optional>
#include <nlohmann/json.hpp>

using json = nlohmann::json;

enum class Priority {
    Low,
    Medium,
    High,
    Urgent
};

enum class Status {
    Todo,
    InProgress,
    Done,
    Cancelled
};

class Task {
public:
    uint32_t id;
    std::string title;
    std::optional<std::string> description;
    Priority priority;
    Status status;
    std::vector<std::string> tags;
    std::chrono::system_clock::time_point created_at;
    std::chrono::system_clock::time_point updated_at;
    uint32_t user_id;
    
    Task(uint32_t id, const std::string& title, uint32_t user_id);
    
    void set_description(const std::string& desc);
    void set_priority(Priority priority);
    void set_tags(const std::vector<std::string>& tags);
    void update_status(Status status);
    void update_priority(Priority priority);
    void update_title(const std::string& new_title);
    
    bool matches(const std::string& keyword) const;
    bool has_status(Status s) const;
    bool has_tag(const std::string& tag) const;
    
    json to_json() const;
    static Task from_json(const json& j);
    
    std::string priority_to_string() const;
    std::string status_to_string() const;
    
    static Priority priority_from_string(const std::string& s);
    static Status status_from_string(const std::string& s);
};
