import {describe, expect, test} from '@jest/globals';
import { LinkedList } from ".";

describe('LinkedList push and unshift', () => {
    test('push single', () => {
        const l: LinkedList<number> = new LinkedList();
        l.push(3);
        expect(l.toArray()).toStrictEqual([3]);
    });

    test('push double', () => {
        const l: LinkedList<number> = new LinkedList();
        l.push(3);
        l.push(4);
        expect(l.toArray()).toStrictEqual([3, 4]);
    });

    test('unshift single', () => {
        const l: LinkedList<number> = new LinkedList();
        l.unshift(3);
        expect(l.toArray()).toStrictEqual([3]);
    });

    test('unshift double', () => {
        const l: LinkedList<number> = new LinkedList();
        l.unshift(3);
        l.unshift(4);
        expect(l.toArray()).toStrictEqual([4, 3]);
    });

    test('unshift and push', () => {
        const l: LinkedList<number> = new LinkedList();
        l.unshift(3);
        l.push(4);
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