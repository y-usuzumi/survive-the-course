package survive.the.course.java.leetcode._715_Range_Module;

import java.util.TreeMap;

class RangeModule {
    private TreeMap<Integer, Integer> rangeMap;

    public RangeModule() {
        rangeMap = new TreeMap<>();
    }

    public void addRange(int left, int right) {
        var floorLeft = rangeMap.floorEntry(left);
        // When the right end of the range touches the left end of the new range,
        // they will merge into a single range.
        if (floorLeft != null && floorLeft.getValue() >= left) {
            left = floorLeft.getKey();
        }

        // Same goes for the right. `floorEntry` finds the entry less than or
        // equal to the target.
        var floorRight = rangeMap.floorEntry(right);
        if (floorRight != null && floorRight.getValue() >= right) {
            // In case `floorEntry` returns a key equal to `right`, we simply
            // need to include it in the keys we remove. This can be done in
            // different ways. By extending the right boundary, we can guarantee
            // `right` is included.
            right = floorRight.getValue();
        }
        rangeMap.subMap(left, right).clear();
        rangeMap.put(left, right);
    }

    public boolean queryRange(int left, int right) {
        // This is simple
        var floorLeft = rangeMap.floorEntry(left);
        return floorLeft != null && floorLeft.getValue() >= right;
    }

    public void removeRange(int left, int right) {
        // Find a range that is cut in half by the requested range
        // The reason we need to do right first is that there can
        // be cases where the a range fully contains the requested
        // range. If we remove from the left first, then we will
        // be missing the remainder of the right side range.
        var floorRight = rangeMap.lowerEntry(right);
        if (floorRight != null && floorRight.getValue() > right) {
            rangeMap.put(right, floorRight.getValue());
        }

        var floorLeft = rangeMap.lowerEntry(left);
        if (floorLeft != null && floorLeft.getValue() > left) {
            // The reason why we do not need to remove the old
            // entry is that the new entry will replace the old
            // one.
            rangeMap.put(floorLeft.getKey(), left);
        }
        // Right side is non-includsive, so the new range added
        // that was cut in half will not be removed.
        rangeMap.subMap(left, right).clear();
    }
}

/**
 * Your RangeModule object will be instantiated and called as such:
 * RangeModule obj = new RangeModule();
 * obj.addRange(left,right);
 * boolean param_2 = obj.queryRange(left,right);
 * obj.removeRange(left,right);
 */
