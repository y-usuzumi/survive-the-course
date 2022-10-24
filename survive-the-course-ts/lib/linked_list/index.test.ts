import {describe, expect, test} from '@jest/globals';
import { LinkedList } from ".";

describe('LinkedList append and prepend', () => {
    test('append single', () => {
        const l: LinkedList<number> = new LinkedList();
        l.append(3);
        expect(l.toArray()).toStrictEqual([3]);
    });

    test('append double', () => {
        const l: LinkedList<number> = new LinkedList();
        l.append(3);
        l.append(4);
        expect(l.toArray()).toStrictEqual([3, 4]);
    });

    test('prepend single', () => {
        const l: LinkedList<number> = new LinkedList();
        l.prepend(3);
        expect(l.toArray()).toStrictEqual([3]);
    });

    test('prepend double', () => {
        const l: LinkedList<number> = new LinkedList();
        l.prepend(3);
        l.prepend(4);
        expect(l.toArray()).toStrictEqual([4, 3]);
    });

    test('prepend and append', () => {
        const l: LinkedList<number> = new LinkedList();
        l.prepend(3);
        l.append(4);
        expect(l.toArray()).toStrictEqual([3, 4]);
    });
});

describe('LinkedList remove', () => {
    test('remove empty', () => {
        const l = new LinkedList();
        l.remove(t => true);
        expect(l.toArray()).toStrictEqual([]);
    });

    test('remove single', () => {
        const l = LinkedList.fromArray([1]);

        l.remove(t => t === 1);
        expect(l.toArray()).toStrictEqual([]);
    });

    test('remove nothing', () => {
        const l = LinkedList.fromArray([1]);

        l.remove(t => false);
        expect(l.toArray()).toStrictEqual([1]);
    });

    test('remove head', () => {
        const l = LinkedList.fromArray([1, 2]);

        l.remove(t => t === 1);
        expect(l.toArray()).toStrictEqual([2]);
    });

    test('remove last', () => {
        const l = LinkedList.fromArray([2, 3]);

        l.remove(t => t === 3);
        expect(l.toArray()).toStrictEqual([2]);
    });

    test('remove multiple from last', () => {
        const l = LinkedList.fromArray([2, 3, 4]);

        l.remove(t => t === 4);
        expect(l.toArray()).toStrictEqual([2, 3]);
        l.remove(t => t === 3);
        expect(l.toArray()).toStrictEqual([2]);
        l.remove(t => t === 2);
        expect(l.toArray()).toStrictEqual([]);
    });
});