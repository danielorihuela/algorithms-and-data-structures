INFINITE = 1/0

function mergesort(values)
    new_values = values
    mergesort(new_values, 1, length(new_values))

    return new_values
end

# Inplace variant (values are changed directly in the array received)
function mergesort(values, start_index, end_index)
    if start_index >= end_index
        return
    end

    middle_index = floor(Int, (start_index + end_index)/2)
    mergesort(values, start_index, middle_index)
    mergesort(values, middle_index + 1, end_index)
    merge(values, start_index, middle_index, end_index)
end

function merge(values, start_index, middle_index, end_index)
    #=
    INFINITE is our sentinel.
    In the loop of lane 33 it will be useful to
    copy all the remaining values of one of the
    arrays, once the other has been already copied.

    Example:

    Initialization:
    i = 1
    j = 1
    k = 1
    left = [3, 29, INF], left[i] = 3
    rigth = [15, INF], right[j] = 15
    values = [3, 29, 15]

    Step 1:
    i = 2
    j = 1
    k = 2
    left = [29, INF], left[i] = 29
    rigth = [15, INF], right[j] = 15
    values = [3, 29, 15]

    Step 2:
    i = 2
    j = 2
    k = 3
    left = [29, INF], left[i] = 29
    rigth = [INF], right[j] = INF
    values = [3, 15, 15]

    Termination:
    i = 3
    j = 2
    k = 3
    left = [INF], left[i] = INF
    rigth = [INF], right[j] = INF
    values = [3, 15, 29]
    =#
    left = [values[start_index:middle_index]; INFINITE]
    right = [values[middle_index + 1:end_index]; INFINITE]

    i = j = 1
    for k = start_index:end_index
        if left[i] < right[j]
            values[k] = left[i]
            i += 1
        else
            values[k] = right[j]
            j += 1            
        end
    end
end

unsorted_array = [29, 1, 31, 4, 3, 2, 15, 10, 20]
print(mergesort(unsorted_array))
