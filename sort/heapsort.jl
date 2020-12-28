unsorted_array = [29, 1, 31, 4, 3, 2, 15, 10, 20]

function heapsort(values)
    heap = build_max_heap(values)
    heap_size = length(heap)

    #=
    Put the biggest element at the end and
    and call max_heapify to mantain the
    max-heap properties.
    =#
    for index in [heap_size:-1:2;]
        heap[index], heap[1] = heap[1], heap[index]
        heap_size -= 1
        max_heapify(heap, heap_size, 1)
    end
    
    return heap
end
    
function build_max_heap(values)
    values_copy = values
    values_size = length(values_copy)
    half_array = floor(Int, values_size/2)

    for index in [half_array:-1:1;]
        max_heapify(values_copy, values_size, index)
    end

    return values_copy
end

function max_heapify(values, length, i)
    left = i * 2
    right = i * 2 + 1

    #=
    Get the index of the biggest value from
    the subtree.

         i
        / \
    left  right
    =#
    largest = i
    if left <= length && values[left] > values[i]
        largest = left
    end
    if right <= length && values[right] > values[largest]
        largest = right
    end

    #=
    If the root of the subtree is not the largest value,
    we have to swap them and call max_heapify for the
    next subtree.

    Example:
    
      right
       / \
    left  i <- max_heapify
         / \
      left right
    =#     
    if largest != i
        values[largest], values[i] = values[i], values[largest]
        max_heapify(values, length, largest)
    end
end

print(heapsort(unsorted_array))