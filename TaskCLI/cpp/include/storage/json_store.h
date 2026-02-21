// include/storage/json_store.h
#pragma once

#include <string>
#include <vector>
#include <memory>

#include "model/task.h"
#include "model/user.h"

class Storage {
public:
    Storage(const std::string& data_dir);
    
    // User operations
    std::vector<std::shared_ptr<User>> get_users();
    std::shared_ptr<User> add_user(const std::string& name);
    void delete_user(uint32_t id);
    std::optional<uint32_t> get_user_id(const std::string& name);
    
    // Task operations
    std::vector<std::shared_ptr<Task>> get_tasks();
    std::shared_ptr<Task> get_task(uint32_t id);
    std::shared_ptr<Task> add_task(std::shared_ptr<Task> task);
    void update_task(std::shared_ptr<Task> task);
    void delete_task(uint32_t id);
    
    std::vector<std::shared_ptr<Task>> get_tasks_by_user(uint32_t user_id);
    std::vector<std::shared_ptr<Task>> get_tasks_by_status(Status status);
    std::vector<std::shared_ptr<Task>> get_tasks_by_tag(const std::string& tag);
    std::vector<std::shared_ptr<Task>> search_tasks(const std::string& keyword);
    
    struct Stats {
        size_t total;
        size_t todo;
        size_t in_progress;
        size_t done;
        size_t cancelled;
    };
    
    Stats get_stats(std::optional<uint32_t> user_id);
    
    uint32_t next_task_id;
    uint32_t next_user_id;
    
private:
    void save();
    void load();
    
    std::string data_dir_;
    std::vector<std::shared_ptr<User>> users_;
    std::vector<std::shared_ptr<Task>> tasks_;
};
