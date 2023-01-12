_POSSIBLE_OFFSETS = [(1, 0), (0, 1), (-1, 0), (0, -1)]


def minimumDistance(numRows, numColumns, area):
    # visited_map records what positions have been visited
    visited_map = []

    # Generate visited_map
    rowIdx = 0
    while rowIdx < numRows:
        visited_map.append([0] * numColumns)
        rowIdx += 1

    result = _traverse(numRows, numColumns, area, visited_map, (0, 0), 0, None)

    if result is None:
        return -1
    return result


def _add_offset(pos, offset):
    '''Coordinates calculation
    '''
    return (pos[0] + offset[0], pos[1] + offset[1])


def _traverse(num_rows, num_columns, area, visited_map, curr_pos, curr_step, min_step):
    '''Recursive traversal to find the shortest distance.

    Arguments: num_rows    -- total rows of the input grid
               num_columns -- total columns of the input grid
               area        -- the input grid
               visited_map -- a map recording the visited locations (same shape with area)
               curr_pos    -- current location for current traversal
               curr_step   -- how many steps have we used in this traversal
               min_step    -- best solution so far

    Returns: None     -- there is a better solution or the traversal has been invalid.
             a number -- A solution found by current traversal process
    '''
    row, col = curr_pos

    # Position needs to be within the grid
    if row >= num_rows or row < 0 or col >= num_columns or col < 0:
        return None

    # If a coordinate happens to be in visited_map then there is a shorter path already
    if visited_map[row][col] > 0:
        return None

    # min_step is None if solution has been founded yet
    # if we have exceeded the current minimum steps there will be no better solutions
    if min_step is not None and curr_step >= min_step:
        return None

    # 0 represents areas without roads. So current traversal becomes invalid.
    if area[row][col] == 0:
        return None

    # 9 is the destination. So we have found a solution.
    if area[row][col] == 9:
        return curr_step

    # Mark current location as visited
    visited_map[row][col] = 1

    for offset in _POSSIBLE_OFFSETS:

        # Recursive traversal
        new_min_step = _traverse(num_rows, num_columns, area, visited_map, _add_offset(curr_pos, offset), curr_step + 1, min_step)

        # Have we found better solution yet? If so, update the best solution
        if new_min_step is not None:
            min_step = new_min_step

    # Mark the current location as unvisited to allow other paths to use the same location.
    visited_map[row][col] = 0

    return min_step

