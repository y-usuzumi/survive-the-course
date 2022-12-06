package survive.the.course.java.leetcode._731_My_Calendar_II;

import java.util.TreeMap;

class MyCalendarTwo {
    private TreeMap<Integer, Integer> treemap;

    public MyCalendarTwo() {
        treemap = new TreeMap<>();
    }

    // Prefix sum. For each start, +1; for each end, -1;
    public boolean book(int start, int end) {
        // Counter for only the start point
        var counter = 0;
        var startFloorEntry = treemap.floorEntry(start);
        // Use the value of the closest point to the left of the start point
        // or itself
        if (startFloorEntry != null) {
            counter = startFloorEntry.getValue();
        } else {
            // If there is no point to the left, start from zero.
            counter = 0;
        }

        // the new booking request will cover the start point.
        // If there are already two booking requests over the start point,
        // we can't book more.
        if (counter == 2) {
            return false;
        }

        // Left exclusive because we already discussed the start point.
        var subMap = treemap.subMap(start, false, end, false);
        // Same here. If any point is already booked twice, we can't book more.
        for (Integer v : subMap.values()) {
            if (v == 2) {
                return false;
            }
        }

        // Increment the counter for start point
        treemap.put(start, counter + 1);
        // Increment the counter for everything between start and end
        // (both exclusive)
        for (var entry : subMap.entrySet()) {
            treemap.put(entry.getKey(), entry.getValue() + 1);
        }

        // Now we only need to discuss the end point.
        // If the end point already exists, we do not modify it, because its
        // counter was incremented from the left side and decremented on itself
        var endFloorEntry = treemap.floorEntry(end);
        // Otherwise, we decrement from the closest point to the left.
        if (endFloorEntry.getKey() != end) {
            treemap.put(end, endFloorEntry.getValue() - 1);
        }

        return true;
    }
}

/**
 * Your MyCalendarTwo object will be instantiated and called as such:
 * MyCalendarTwo obj = new MyCalendarTwo();
 * boolean param_1 = obj.book(start,end);
 */
