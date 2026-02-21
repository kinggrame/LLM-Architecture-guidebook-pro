// src/model/task.cpp

#include "model/task.h"
#include <algorithm>

using namespace std::chrono;

Task::Task(uint32_t id, const std::string& title, uint32_t user_id)
    : id(id), title(title), user_id(user_id) {
    priority = Priority::Medium;
    status = Status::Todo;
    created_at = system_clock::now();
    updated_at = system_clock::now();
}

void Task::set_description(const std::string& desc) {
    description = desc;
    updated_at = system_clock::now();
}

void Task::set_priority(Priority p) {
    priority = p;
    updated_at = system_clock::now();
}

void Task::set_tags(const std::vector<std::string>& t) {
    tags = t;
    updated_at = system_clock::now();
}

void Task::update_status(Status s) {
    status = s;
    updated_at = system_clock::now();
}

void Task::update_priority(Priority p) {
    priority = p;
    updated_at = system_clock::now();
}

void Task::update_title(const std::string& new_title) {
    title = new_title;
    updated_at = system_clock::now();
}

bool Task::matches(const std::string& keyword) const {
    std::string lower_keyword = keyword;
    std::transform(lower_keyword.begin(), lower_keyword.end(), lower_keyword.begin(), ::tolower);
    
    // 检查标题
    std::string lower_title = title;
    std::transform(lower_title.begin(), lower_title.end(), lower_title.begin(), ::tolower);
    if (lower_title.find(lower_keyword) != std::string::npos) return true;
    
    // 检查描述
    if (description.has_value()) {
        std::string lower_desc = *description;
        std::transform(lower_desc.begin(), lower_desc.end(), lower_desc.begin(), ::tolower);
        if (lower_desc.find(lower_keyword) != std::string::npos) return true;
    }
    
    // 检查标签
    for (const auto& tag : tags) {
        std::string lower_tag = tag;
        std::transform(lower_tag.begin(), lower_tag.end(), lower_tag.begin(), ::tolower);
        if (lower_tag.find(lower_keyword) != std::string::npos) return true;
    }
    
    return false;
}

bool Task::has_status(Status s) const {
    return status == s;
}

bool Task::has_tag(const std::string& tag) const {
    return std::find(tags.begin(), tags.end(), tag) != tags.end();
}

std::string Task::priority_to_string() const {
    switch (priority) {
        case Priority::Low: return "low";
        case Priority::Medium: return "medium";
        case Priority::High: return "high";
        case Priority::Urgent: return "urgent";
    }
    return "medium";
}

std::string Task::status_to_string() const {
    switch (status) {
        case Status::Todo: return "todo";
        case Status::InProgress: return "in_progress";
        case Status::Done: return "done";
        case Status::Cancelled: return "cancelled";
    }
    return "todo";
}

Priority Task::priority_from_string(const std::string& s) {
    if (s == "low") return Priority::Low;
    if (s == "high") return Priority::High;
    if (s == "urgent") return Priority::Urgent;
    return Priority::Medium;
}

Status Task::status_from_string(const std::string& s) {
    if (s == "in_progress" || s == "inprogress") return Status::InProgress;
    if (s == "done") return Status::Done;
    if (s == "cancelled" || s == "cancel") return Status::Cancelled;
    return Status::Todo;
}

json Task::to_json() const {
    json j;
    j["id"] = id;
    j["title"] = title;
    j["priority"] = priority_to_string();
    j["status"] = status_to_string();
    j["user_id"] = user_id;
    j["tags"] = tags;
    if (description.has_value()) {
        j["description"] = *description;
    }
    j["created_at"] = std::chrono::system_clock::to_time_t(created_at);
    j["updated_at"] = std::chrono::system_clock::to_time_t(updated_at);
    return j;
}

Task Task::from_json(const json& j) {
    Task task(j["id"].get<uint32_t>(), j["title"].get<std::string>(), j["user_id"].get<uint32_t>());
    task.priority = priority_from_string(j.value("priority", "medium"));
    task.status = status_from_string(j.value("status", "todo"));
    if (j.contains("description") && !j["description"].is_null()) {
        task.description = j["description"].get<std::string>();
    }
    if (j.contains("tags")) {
        task.tags = j["tags"].get<std::vector<std::string>>();
    }
    if (j.contains("created_at")) {
        task.created_at = std::chrono::system_clock::from_time_t(j["created_at"].get<std::time_t>());
    }
    if (j.contains("updated_at")) {
        task.updated_at = std::chrono::system_clock::from_time_t(j["updated_at"].get<std::time_t>());
    }
    return task;
}
