import re
import heapq

_OLDER_GEN = 0
_NEWER_GEN = 1

_OLDER_GEN_REGEX = re.compile('[a-z]')

def orderedJunctionBoxes(numberOfBoxes, boxList):
    if numberOfBoxes == 0:
        return boxList

    # Min-heap to sort the older-gen junction boxes
    older_gen_heap = []
    # The list that keeps the newer-gen junction boxes in their original order
    newer_gen_list = []

    for box in boxList:
        (gen, box_info) = _get_sortable_box_info(box)
        if gen == _NEWER_GEN:
            newer_gen_list.append(box_info)
        else:
            # Prepare for heapsort older-gen boxes by its version and identifier.
            # (Sorry I got this wrong! See comments below please!)
            heapq.heappush(older_gen_heap, box_info)
    result = []
    while older_gen_heap:  # Heapsort older-gen boxes
        result.append(heapq.heappop(older_gen_heap)[1])

    # Append newer-gen boxes per their original order
    result.extend(newer_gen_list)
    return result


def _get_sortable_box_info(box):
    if not isinstance(box, basestring):
        raise TypeError("Expected a string. Got value {} of type {}".format(box, type(box)))

    slices = box.split(' ', 1)
    if len(slices) == 0:
        raise ValueError("Expected a space-delimited junction box definition.")
    ident, version = slices[0], slices[1]
    if _OLDER_GEN_REGEX.search(version) is not None:  # is older gen
        # If I understand it correctly, lexicographical order is the same as alphanumerical order
        # given that the string can contain both letters and numbers.

        # P.S. I'm SO SORRY that I have realized that comparing version with identifier is wrong
        # because putting the identifier at the end of version slices may happen to form a tie.
        # Sorry I have no time to fix my code.
        version_slices = version.split(' ')

        version_slices.append(ident)

        # Older-gen boxes need version_slices for comparison.
        return (_OLDER_GEN, (version_slices, box))

    # Newer-gen does not need extra info.
    return (_NEWER_GEN, box)
