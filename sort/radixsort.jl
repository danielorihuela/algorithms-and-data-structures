#=
Slighly modified counting sort to use with
radix sort.
=#
function countingsort(values, digit_position)
    digit_values = map(number -> digit_from_position(number, digit_position), values)
    number_count = zeros(Int, maximum(digit_values) + 1)
    for value in digit_values
        number_count[value + 1] += 1
    end
    cumulative_sum_number_count = cumsum(number_count)

    sorted_array = zeros(Int, length(digit_values))
    for value in reverse(values)
        digit = digit_from_position(value, digit_position) + 1
        sorted_array[cumulative_sum_number_count[digit]] = value
        cumulative_sum_number_count[digit] -= 1
    end

    return sorted_array
end

function digit_from_position(number, digit_position)
    return floor(Int, number%(10^digit_position) / (10^(digit_position - 1)))
end

function radixsort(values)
    max_num_digits = length(digits(maximum(values)))

    sorted_array = values
    for digit_position = 1:max_num_digits
        sorted_array = countingsort(sorted_array, digit_position)
    end

    return sorted_array
end

unsorted_array = [123, 2, 25, 8293, 790, 50]
print(radixsort(unsorted_array))
