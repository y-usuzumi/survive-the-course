package survive.the.course.java.lib.ds;

import java.util.List;

/**
 * Efficient Segment Tree implementation
 */
public class SegmentTree {
    private int[] arr;
    private int length;

    /**
     * Creates a Segment Tree from a list of integers
     * @param data
     */
    public SegmentTree(List<Integer> data) {
        this.length = data.size();
        arr = new int[data.size() * 2];
        for (int idx = 0; idx < data.size(); idx++) {
            arr[this.length + idx] = data.get(idx);
        }
        populate();
    }

    public void update(int index, int number) {
        index += length;
        this.arr[index] = number;
        while (index > 1) {
            this.arr[index >> 1] = this.arr[index] + this.arr[index ^ 1];
            index >>= 1;
        }
    }

    /**
     * Query the sum of the range [start, end).
     * @param start Start index (inclusive)
     * @param end End index (exclusive)
     * @return The result
     */
    public int sumRange(int start, int end) {
        if (start >= this.length || end <= start) {
            return 0;
        }
        if (start < 0) {
            start = 0;
        }
        if (end > this.length) {
            end = this.length;
        }
        start += length;
        end += length;
        int result = 0;
        while (start < end) {
            if ((start & 1) == 1) {
                result += arr[start];
                start += 1;
            }
            start >>= 1;
            if ((end & 1) == 1) {
                result += arr[end - 1];
            }
            end >>= 1;
        }
        return result;
    }

    private void populate() {
        for (int idx = this.length - 1; idx >= 1; idx--) {
            this.arr[idx] = this.arr[idx * 2] + this.arr[idx * 2 + 1];
        }
    }
}
