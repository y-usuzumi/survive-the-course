package survive.the.course.java.lib.ds.segment_tree;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public class SegmentTree<T> {
    private ArrayList<T> arr;
    public SegmentTree(List<T> data) {
        int len = data.size();
        int arrLength = (int)(2 * Math.pow(2, Math.ceil(Math.log(len) / Math.log(2))));
        this.arr = new ArrayList<>(Collections.nCopies(arrLength, null));
        populate(data, 1, 0, len);
    }

    private int getMid(int left, int right) {
        return (left + right) / 2;
    }

    private void populate(List<T> data, int currIdx, int left, int right) {
        if (left > right) {
            return;
        }

        if (left == right) {
            arr.set(currIdx, data.get(left));
        } else {
            int mid = getMid(left, right);
            populate(data, currIdx * 2, left, mid);
            populate(data, currIdx * 2 + 1, mid+1, right);
        }
    }

    public T accumRange(int left, int right) {
        return null;
    }
}
