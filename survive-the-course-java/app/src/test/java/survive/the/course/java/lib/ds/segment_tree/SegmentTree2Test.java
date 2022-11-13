package survive.the.course.java.lib.ds.segment_tree;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.util.List;

import org.junit.jupiter.api.Test;

public class SegmentTree2Test {
    @Test
    public void test_sumRange() {
        SegmentTree2 st = new SegmentTree2(List.of(1, 3, 5, 7, 9, 11));
        assertEquals(36, st.sumRange(0, 6));
        assertEquals(15, st.sumRange(1, 4));
    }

    @Test
    public void test_update() {
        SegmentTree2 st = new SegmentTree2(List.of(1, 3, 5, 7, 9, 11));
        st.update(0, 8);
        assertEquals(20, st.sumRange(4, 6));
        assertEquals(32, st.sumRange(0, 5));
        assertEquals(43, st.sumRange(0, 6));
    }
}
