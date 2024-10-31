#pragma once

#include <array>
#include <cmath>
#include <concepts>

namespace vec {

template <typename R, typename T, std::size_t dim, typename F>
    requires std::invocable<F, const T&> &&
    std::same_as<std::invoke_result_t<F, const T&>, R>
auto map(const std::array<T, dim>& input, F fun) -> std::array<R, dim> {
    std::array<R, dim> output;
    for (std::size_t i = 0; i < dim; ++i) {
        output[i] = fun(input[i]);
    }
    return output;
}

using Vec2f = std::array<float, 2>;

template <typename T>
concept Numeric = requires(T a, T b) {
    // { (a + b) } -> std::convertible_to<T>;
    { (a + b) } -> std::same_as<T>;
    { (a - b) } -> std::same_as<T>;
    { (a * b) } -> std::same_as<T>;
    { (a / b) } -> std::same_as<T>;
    { std::abs(a) } -> std::same_as<T>;
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

template <Numeric T, std::size_t dim, std::convertible_to<T> S>
auto operator+(S a, std::array<T, dim> b) -> std::array<T, dim> {
    auto result = std::array<T, dim>{};
    for (std::size_t i = 0; i < dim; i += 1) {
        result[i] = a + b[i];
    }
    return result;
}

template <Numeric T, std::size_t dim, std::convertible_to<T> S>
auto operator+(std::array<T, dim> a, S b) -> std::array<T, dim> {
    auto result = std::array<T, dim>{};
    for (std::size_t i = 0; i < dim; i += 1) {
        result[i] = a[i] + b;
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

template <Numeric T, std::size_t dim, std::convertible_to<T> S>
auto operator-(S a, std::array<T, dim> b) -> std::array<T, dim> {
    auto result = std::array<T, dim>{};
    for (std::size_t i = 0; i < dim; i += 1) {
        result[i] = a - b[i];
    }
    return result;
}

template <Numeric T, std::size_t dim, std::convertible_to<T> S>
auto operator-(std::array<T, dim> a, S b) -> std::array<T, dim> {
    auto result = std::array<T, dim>{};
    for (std::size_t i = 0; i < dim; i += 1) {
        result[i] = a[i] - b;
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

template <Numeric T, std::size_t dim, std::convertible_to<T> S>
auto operator*(S a, std::array<T, dim> b) -> std::array<T, dim> {
    auto result = std::array<T, dim>{};
    for (std::size_t i = 0; i < dim; i += 1) {
        result[i] = a * b[i];
    }
    return result;
}

template <Numeric T, std::size_t dim, std::convertible_to<T> S>
auto operator*(std::array<T, dim> a, S b) -> std::array<T, dim> {
    auto result = std::array<T, dim>{};
    for (std::size_t i = 0; i < dim; i += 1) {
        result[i] = a[i] * b;
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

template <Numeric T, std::size_t dim, std::convertible_to<T> S>
auto operator/(S a, std::array<T, dim> b) -> std::array<T, dim> {
    auto result = std::array<T, dim>{};
    for (std::size_t i = 0; i < dim; i += 1) {
        result[i] = a / b[i];
    }
    return result;
}

template <Numeric T, std::size_t dim, std::convertible_to<T> S>
auto operator/(std::array<T, dim> a, S b) -> std::array<T, dim> {
    auto result = std::array<T, dim>{};
    for (std::size_t i = 0; i < dim; i += 1) {
        result[i] = a[i] / b;
    }
    return result;
}

template <Numeric T, std::size_t dim>
auto abs(std::array<T, dim> a) -> std::array<T, dim> {
    auto result = std::array<T, dim>{};
    for (std::size_t i = 0; i < dim; i += 1) {
        result[i] = std::abs(a[i]);
    }
    return result;
}

template <Numeric T, std::size_t dim>
auto floor(std::array<T, dim> a) -> std::array<T, dim> {
    auto result = std::array<T, dim>{};
    for (std::size_t i = 0; i < dim; i += 1) {
        result[i] = std::floor(a[i]);
    }
    return result;
}

template <Numeric T, std::size_t dim>
auto inside(
    std::array<T, dim> test,
    std::array<T, dim> center,
    std::array<T, dim> extent
) -> bool {
    auto scaled = abs(center - test) / extent;
    for (auto x : scaled) {
        if (x > 1) {
            return false;
        }
    }
    return true;
}

} // namespace vec
