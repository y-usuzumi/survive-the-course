#lang racket

(provide (all-defined-out)) ;; so we can put tests in a second file

;; put your code below
(define (sequence low high stride)
  (if (> low high)
      null
      (cons low (sequence (+ low stride) high stride))))

(define (string-append-map xs suffix)
  (map (lambda (v) (string-append v suffix)) xs))

(define (list-nth-mod xs n)
  (cond [(< n 0) (error "list-nth-mod: negative number")]
        [(null? xs) (error "list-nth-mod: empty list")]
        [#t (car (list-tail xs (remainder n (length xs))))]))

(define (stream-for-n-steps s n)
  (if (<= n 0)
      null
      (let ([result (s)])
        (cons (car result) (stream-for-n-steps (cdr result) (- n 1))))))

(define (funny-number-stream)
  (define (helper curr)
    (cons (if (= 0 (remainder curr 5)) (- curr) curr) (lambda () (helper (+ 1 curr)))))
  (helper 1))

(define (dan-then-dog)
  (define (dog-then-dan)
    (cons "dog.jpg" dan-then-dog))
  (cons "dan.jpg" dog-then-dan))

(define (stream-add-zero s)
  (let ([v (s)])
    (lambda () (cons (cons 0 (car v)) (stream-add-zero (cdr v))))))

(define (cycle-lists xs ys)
  (define (helper inner-xs inner-ys)
    (cond [(null? inner-xs) (helper xs inner-ys)]
          [(null? inner-ys) (helper inner-xs ys)]
          [#t (lambda () (cons (cons (car inner-xs) (car inner-ys)) (helper (cdr inner-xs) (cdr inner-ys))))]))
  (helper xs ys))

(define (vector-assoc v vec)
  (define (check-position n)
    (if (>= n (vector-length vec))
        #f
        (let ([curr (vector-ref vec n)])
          (if (and (pair? curr) (equal? (car curr) v)) curr (check-position (+ n 1))))))
  (check-position 0))

(define (cached-assoc xs n)
  (let ([memo (make-vector n #f)]
        [cursor 0])
    (lambda (v)
      (let ([cached-result (vector-assoc v memo)])
        (if cached-result
            cached-result
            (let ([calc-result (assoc v xs)])
              (if calc-result
                  (begin
                    (vector-set! memo cursor calc-result)
                    (set! cursor (remainder (+ cursor 1) n))
                    calc-result)
                  #f)))))))

(define-syntax while-less
  (syntax-rules (do)
    [(while-less e1 do e2)
     (letrec ([e1result e1]
              [loop (lambda ()
                      (if (< e2 e1result) (loop) #t))])
       (loop))]))
