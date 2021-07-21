function countingsort(values)
    #=
    In julia arrays start with index 1 so we need
    to add 1 position to handle the 0.
    The index 1 will represent number 0, index 2
    will represent number 1, ...
    =#
    number_count = zeros(Int, maximum(values) + 1)
    for value in values
        number_count[value + 1] += 1
    end
    cumulative_sum_number_count = cumsum(number_count)

    #=
    Since we know how many numbers are equal or less
    than the actual number, we can put it in the
    correct position. After that we decrement the
    cumulative sum by 1.

    Example:
     1 2 3 4 5                   1 2 3 4
    [1,3,2,4,2] -> cumulative = [1,3,4,5]

    Step 1:
    [1,3,2,4,2] -> result = [0,0,2,0,0], cumulative = [1,2,4,5]
             ^
    Step 2:
    [1,3,2,4,2] -> result = [0,0,2,0,4], cumulative = [1,2,4,4]
           ^
    Step 3:
    [1,3,2,4,2] -> result = [0,2,2,0,4], cumulative = [1,1,4,4]
         ^
    Step 4:
    [1,3,2,4,2] -> result = [0,2,2,3,4], cumulative = [1,2,3,4]
       ^
    Step 5:
    [1,3,2,4,2] -> result = [1,2,2,3,4], cumulative = [0,2,4,4]
     ^
    =#
    sorted_array = zeros(Int, length(values))
    for value in reverse(values)
        sorted_array[cumulative_sum_number_count[value + 1]] = value
        cumulative_sum_number_count[value + 1] -= 1
    end

    return sorted_array
end

unsorted_array = [29, 1, 31, 4, 0, 3, 2, 15, 10, 20]
print(countingsort(unsorted_array))
