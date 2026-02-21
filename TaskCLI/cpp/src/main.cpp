// src/main.cpp - C++ 版本入口

#include <iostream>
#include <memory>
#include <string>
#include <vector>
#include <optional>
#include <filesystem>

#include "model/task.h"
#include "model/user.h"
#include "storage/json_store.h"

namespace fs = std::filesystem;

void print_usage() {
    std::cout << "Usage: taskcli <command> [options]\n";
    std::cout << "\nCommands:\n";
    std::cout << "  user add <name>     - Add a new user\n";
    std::cout << "  user list           - List all users\n";
    std::cout << "  user delete <id>    - Delete a user\n";
    std::cout << "  add <title>         - Add a new task\n";
    std::cout << "  list                - List all tasks\n";
    std::cout << "  show <id>           - Show task details\n";
    std::cout << "  modify <id>         - Modify a task\n";
    std::cout << "  delete <id>         - Delete a task\n";
    std::cout << "  search <keyword>    - Search tasks\n";
    std::cout << "  stats               - Show statistics\n";
}

int main(int argc, char* argv[]) {
    if (argc < 2) {
        print_usage();
        return 1;
    }
    
    std::string data_dir = "./data";
    
    std::string cmd = argv[1];
    
    try {
        auto storage = std::make_unique<Storage>(data_dir);
        
        if (cmd == "user") {
            if (argc < 3) {
                std::cerr << "Error: missing user command\n";
                return 1;
            }
            std::string subcmd = argv[2];
            
            if (subcmd == "add") {
                if (argc < 4) {
                    std::cerr << "Error: missing user name\n";
                    return 1;
                }
                std::string name = argv[3];
                auto user = storage->add_user(name);
                std::cout << "User created: " << user->name << " (ID: " << user->id << ")\n";
            } else if (subcmd == "list") {
                auto users = storage->get_users();
                for (const auto& u : users) {
                    std::cout << u.id << " - " << u.name << "\n";
                }
            } else {
                std::cerr << "Unknown user command: " << subcmd << "\n";
                return 1;
            }
        } else if (cmd == "add") {
            if (argc < 3) {
                std::cerr << "Error: missing task title\n";
                return 1;
            }
            std::string title = argv[2];
            
            auto task = std::make_shared<Task>(storage->next_task_id(), title, 1);
            storage->add_task(task);
            std::cout << "Task created: #" << task->id << " - " << task->title << "\n";
        } else if (cmd == "list") {
            auto tasks = storage->get_tasks();
            for (const auto& t : tasks) {
                std::cout << "#" << t->id << " [" << t->status_to_string() << "] "
                          << t->title << " (" << t->priority_to_string() << ")\n";
            }
            std::cout << "Total: " << tasks.size() << " tasks\n";
        } else if (cmd == "search") {
            if (argc < 3) {
                std::cerr << "Error: missing keyword\n";
                return 1;
            }
            std::string keyword = argv[2];
            auto results = storage->search_tasks(keyword);
            for (const auto& t : results) {
                std::cout << "#" << t->id << " " << t->title << "\n";
            }
        } else if (cmd == "stats") {
            auto stats = storage->get_stats(std::nullopt);
            std::cout << "Total: " << stats.total << "\n";
            std::cout << "  Todo: " << stats.todo << "\n";
            std::cout << "  Done: " << stats.done << "\n";
        } else {
            std::cerr << "Unknown command: " << cmd << "\n";
            return 1;
        }
        
    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << "\n";
        return 1;
    }
    
    return 0;
}
