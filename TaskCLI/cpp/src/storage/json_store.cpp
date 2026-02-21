// src/storage/json_store.cpp

#include "storage/json_store.h"
#include <fstream>
#include <algorithm>

using json = nlohmann::json;

Storage::Storage(const std::string& data_dir) 
    : data_dir_(data_dir), next_task_id_(1), next_user_id_(1) {
    
    std::filesystem::create_directories(data_dir_);
    load();
}

void Storage::load() {
    std::ifstream file(data_dir_ + "/data.json");
    if (file.is_open()) {
        try {
            json j;
            file >> j;
            
            for (const auto& u : j["users"]) {
                users_.push_back(std::make_shared<User>(User::from_json(u)));
            }
            
            for (const auto& t : j["tasks"]) {
                // 简化的反序列化
                auto task = std::make_shared<Task>(t["id"], t["title"], t["user_id"]);
                task->priority = Task::priority_from_string(t["priority"]);
                task->status = Task::status_from_string(t["status"]);
                tasks_.push_back(task);
            }
            
            next_user_id_ = j.value("next_user_id", 1);
            next_task_id_ = j.value("next_task_id", 1);
        } catch (...) {
            // 如果加载失败，使用默认数据
        }
    }
    
    if (users_.empty()) {
        users_.push_back(std::make_shared<User>(User(1, "default")));
        next_user_id_ = 2;
    }
}

void Storage::save() {
    json j;
    
    for (const auto& u : users_) {
        j["users"].push_back(u->to_json());
    }
    
    for (const auto& t : tasks_) {
        json task_json;
        task_json["id"] = t->id;
        task_json["title"] = t->title;
        task_json["priority"] = t->priority_to_string();
        task_json["status"] = t->status_to_string();
        task_json["user_id"] = t->user_id;
        task_json["tags"] = t->tags;
        j["tasks"].push_back(task_json);
    }
    
    j["next_user_id"] = next_user_id_;
    j["next_task_id"] = next_task_id_;
    
    std::ofstream file(data_dir_ + "/data.json");
    file << j.dump(2);
}

std::vector<std::shared_ptr<User>> Storage::get_users() {
    return users_;
}

std::shared_ptr<User> Storage::add_user(const std::string& name) {
    auto user = std::make_shared<User>(User(next_user_id_++, name));
    users_.push_back(user);
    save();
    return user;
}

void Storage::delete_user(uint32_t id) {
    users_.erase(std::remove_if(users_.begin(), users_.end(),
        [id](const auto& u) { return u->id == id; }), users_.end());
    
    tasks_.erase(std::remove_if(tasks_.begin(), tasks_.end(),
        [id](const auto& t) { return t->user_id == id; }), tasks_.end());
    
    save();
}

std::optional<uint32_t> Storage::get_user_id(const std::string& name) {
    for (const auto& u : users_) {
        if (u->name == name) return u->id;
    }
    return std::nullopt;
}

std::vector<std::shared_ptr<Task>> Storage::get_tasks() {
    return tasks_;
}

std::shared_ptr<Task> Storage::get_task(uint32_t id) {
    for (const auto& t : tasks_) {
        if (t->id == id) return t;
    }
    return nullptr;
}

std::shared_ptr<Task> Storage::add_task(std::shared_ptr<Task> task) {
    task->id = next_task_id_++;
    tasks_.push_back(task);
    save();
    return task;
}

void Storage::update_task(std::shared_ptr<Task> task) {
    for (auto& t : tasks_) {
        if (t->id == task->id) {
            t = task;
            break;
        }
    }
    save();
}

void Storage::delete_task(uint32_t id) {
    tasks_.erase(std::remove_if(tasks_.begin(), tasks_.end(),
        [id](const auto& t) { return t->id == id; }), tasks_.end());
    save();
}

std::vector<std::shared_ptr<Task>> Storage::get_tasks_by_user(uint32_t user_id) {
    std::vector<std::shared_ptr<Task>> result;
    for (const auto& t : tasks_) {
        if (t->user_id == user_id) result.push_back(t);
    }
    return result;
}

std::vector<std::shared_ptr<Task>> Storage::get_tasks_by_status(Status status) {
    std::vector<std::shared_ptr<Task>> result;
    for (const auto& t : tasks_) {
        if (t->status == status) result.push_back(t);
    }
    return result;
}

std::vector<std::shared_ptr<Task>> Storage::get_tasks_by_tag(const std::string& tag) {
    std::vector<std::shared_ptr<Task>> result;
    for (const auto& t : tasks_) {
        if (t->has_tag(tag)) result.push_back(t);
    }
    return result;
}

std::vector<std::shared_ptr<Task>> Storage::search_tasks(const std::string& keyword) {
    std::vector<std::shared_ptr<Task>> result;
    for (const auto& t : tasks_) {
        if (t->matches(keyword)) result.push_back(t);
    }
    return result;
}

Storage::Stats Storage::get_stats(std::optional<uint32_t> user_id) {
    Stats s{0, 0, 0, 0, 0};
    
    for (const auto& t : tasks_) {
        if (user_id.has_value() && t->user_id != user_id.value()) continue;
        
        s.total++;
        switch (t->status) {
            case Status::Todo: s.todo++; break;
            case Status::InProgress: s.in_progress++; break;
            case Status::Done: s.done++; break;
            case Status::Cancelled: s.cancelled++; break;
        }
    }
    
    return s;
}
