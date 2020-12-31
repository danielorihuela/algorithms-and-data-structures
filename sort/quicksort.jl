unsorted_array = [29, 1, 31, 4, 3, 2, 15, 10, 20]

#=
I wanted a function to swap values. However,
for some reason it did not correctly change
two values of an array. I could have done a
function to swap values in an array but I
wanted a general function to swap values from
variables and values from an array.
For that reason I ended with a macro. 
My expertise with macros is 0, but from what
I have understood it seems that the macro is
transforming the code in the Abstract Syntax Tree.
Transforming each line where @swap is used,
to the syntax: x, y = y, x.
Whereas "function swap(value1, value2)"
only swapped the values in the local context.

References:
https://docs.julialang.org/en/v1/manual/metaprogramming/
=#
macro swap(x, y)
    quote
        $(esc(x)), $(esc(y)) = $(esc(y)), $(esc(x))
    end
end

function quicksort(values)
    new_values = values
    quicksort(new_values, 1, length(new_values))

    return new_values
end

#=
Random version of quicksort to obtain good expected
performance over all inputs.
=#
function quicksort(values, start_index, end_index)
    if start_index >= end_index
        return
    end

    pivot_index = random_partition(values, start_index, end_index)
    quicksort(values, start_index, pivot_index - 1)
    quicksort(values, pivot_index + 1, end_index)
end

function random_partition(values, start_index, end_index)
    random_index = rand(start_index:end_index)
    @swap(values[random_index], values[end_index])

    #=
    Set lower values than the pivot on the left,
    and higher values on the right.
    Finally, put the pivot in between those arrays.


    Example:

    (Initialization) pivot = 8, pivot_index = 6
    1 - [5|9,13,4,8] pivot_new_index = 1
         ^ 5 < pivot

    2 - [5,9|13,4,8] pivot_new_index = 2
           ^ 9 > pivot

    3 - [5,9|13,4,8] pivot_new_index = 2
              ^ 13 > pivot

    4 - [5,9|13,4,8] pivot_new_index = 2
                ^ 4 < pivot

    (Termination) pivot = 8, pivot_index = 3
    5 - [5,4|8|9,13] -> lower values | pivot | bigger values
    =#
    pivot_index = end_index
    pivot_new_index = start_index
    for element in start_index:end_index - 1
        if values[element] <= values[pivot_index]
            @swap(values[pivot_new_index], values[element])
            pivot_new_index += 1
        end
    end
    @swap(values[pivot_new_index], values[pivot_index])

    return pivot_new_index
end

print(quicksort(unsorted_array))
