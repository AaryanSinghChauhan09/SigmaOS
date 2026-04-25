#ifndef SIGMA_LOCK_FREE_QUEUE_H
#define SIGMA_LOCK_FREE_QUEUE_H

#include <atomic>
#include <memory>

namespace sigma {
namespace core {

// Highly optimized Lock-Free Queue for parallel subsystem IPC (Logging, Networking)
// Eliminates mutex contention for maximum throughput.
template<typename T>
class LockFreeQueue {
private:
    struct Node {
        std::shared_ptr<T> data;
        Node* next;
        Node() : next(nullptr) {}
    };

    std::atomic<Node*> head;
    std::atomic<Node*> tail;

public:
    LockFreeQueue() {
        Node* dummy = new Node();
        head.store(dummy);
        tail.store(dummy);
    }

    ~LockFreeQueue() {
        while (Node* old_head = head.load()) {
            head.store(old_head->next);
            delete old_head;
        }
    }

    void enqueue(T value) {
        std::shared_ptr<T> new_data = std::make_shared<T>(std::move(value));
        Node* new_node = new Node();
        new_node->data = new_data;

        Node* old_tail = tail.exchange(new_node);
        old_tail->next = new_node;
    }

    std::shared_ptr<T> dequeue() {
        Node* old_head = head.load();
        Node* next_node = old_head->next;
        if (next_node == nullptr) {
            return std::shared_ptr<T>(); // Queue is empty
        }
        
        std::shared_ptr<T> result = next_node->data;
        head.store(next_node);
        delete old_head;
        return result;
    }
};

} // namespace core
} // namespace sigma

#endif // SIGMA_LOCK_FREE_QUEUE_H
