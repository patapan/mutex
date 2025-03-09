#pragma once

#include <cstdint>
#include <compare>

class Order {
public:
    int64_t price;
    uint64_t quantity;
    int64_t filled_volume;
    int64_t timestamp; // Unix time the order was placed

    // Use the spaceship operator for custom ordering
    auto operator<=>(const Order& rhs) const {
        // First compare by price (ascending)
        if (auto cmp = price <=> rhs.price; cmp != 0) {
            return cmp;
        }
        // If prices are equal, compare by timestamp (ascending)
        return timestamp <=> rhs.timestamp;
    }

    // Optionally, define equality explicitly
    bool operator==(const Order& rhs) const = default;
};
