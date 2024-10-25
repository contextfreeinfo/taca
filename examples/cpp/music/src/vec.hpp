#pragma once

#include <array>
#include <cmath>
#include <concepts>

namespace vec {

using Vec2f = std::array<float, 2>;

template <typename T>
concept Numeric = requires(T a, T b) {
    // { (a + b) } -> std::convertible_to<T>;
    { (a + b) } -> std::same_as<T>;
    { (a - b) } -> std::same_as<T>;
    { (a * b) } -> std::same_as<T>;
    { (a / b) } -> std::same_as<T>;
    { std::floor(a) } -> std::same_as<T>;
};

template <Numeric T, std::size_t dim>
auto operator+(std::array<T, dim> a, std::array<T, dim> b)
    -> std::array<T, dim> {
    auto result = std::array<T, dim>{};
    for (std::size_t i = 0; i < dim; i += 1) {
        result[i] = a[i] + b[i];
    }
    return result;
}

template <Numeric T, std::size_t dim>
auto operator-(std::array<T, dim> a, std::array<T, dim> b)
    -> std::array<T, dim> {
    auto result = std::array<T, dim>{};
    for (std::size_t i = 0; i < dim; i += 1) {
        result[i] = a[i] - b[i];
    }
    return result;
}

template <Numeric T, std::size_t dim>
auto operator*(std::array<T, dim> a, std::array<T, dim> b)
    -> std::array<T, dim> {
    auto result = std::array<T, dim>{};
    for (std::size_t i = 0; i < dim; i += 1) {
        result[i] = a[i] * b[i];
    }
    return result;
}

template <Numeric T, std::size_t dim>
auto operator/(std::array<T, dim> a, std::array<T, dim> b)
    -> std::array<T, dim> {
    auto result = std::array<T, dim>{};
    for (std::size_t i = 0; i < dim; i += 1) {
        result[i] = a[i] / b[i];
    }
    return result;
}

template <Numeric T, std::size_t dim>
auto floor(std::array<T, dim> a)
    -> std::array<T, dim> {
    auto result = std::array<T, dim>{};
    for (std::size_t i = 0; i < dim; i += 1) {
        result[i] = std::floor(a[i]);
    }
    return result;
}

} // namespace vec
