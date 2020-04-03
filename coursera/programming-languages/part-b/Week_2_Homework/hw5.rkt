;; Programming Languages, Homework 5

#lang racket
(provide (all-defined-out)) ;; so we can put tests in a second file

;; definition of structures for MUPL programs - Do NOT change
(struct var  (string) #:transparent)  ;; a variable, e.g., (var "foo")
(struct int  (num)    #:transparent)  ;; a constant number, e.g., (int 17)
(struct add  (e1 e2)  #:transparent)  ;; add two expressions
(struct ifgreater (e1 e2 e3 e4)    #:transparent) ;; if e1 > e2 then e3 else e4
(struct fun  (nameopt formal body) #:transparent) ;; a recursive(?) 1-argument function
(struct call (funexp actual)       #:transparent) ;; function call
(struct mlet (var e body) #:transparent) ;; a local binding (let var = e in body) 
(struct apair (e1 e2)     #:transparent) ;; make a new pair
(struct fst  (e)    #:transparent) ;; get first part of a pair
(struct snd  (e)    #:transparent) ;; get second part of a pair
(struct aunit ()    #:transparent) ;; unit value -- good for ending a list
(struct isaunit (e) #:transparent) ;; evaluate to 1 if e is unit else 0

;; a closure is not in "source" programs but /is/ a MUPL value; it is what functions evaluate to
(struct closure (env fun) #:transparent)

;; Problem 1

(define (racketlist->mupllist es)
  (if (null? es) (aunit) (apair (car es) (racketlist->mupllist (cdr es)))))

(define (mupllist->racketlist ml)
  (if (aunit? ml) null (cons (apair-e1 ml) (mupllist->racketlist (apair-e2 ml)))))

;; Problem 2

;; lookup a variable in an environment
;; Do NOT change this function
(define (envlookup env str)
  (cond [(null? env) (error "unbound variable during evaluation" str)]
        [(equal? (car (car env)) str) (cdr (car env))]
        [#t (envlookup (cdr env) str)]))

;; Do NOT change the two cases given to you.
;; DO add more cases for other kinds of MUPL expressions.
;; We will test eval-under-env by calling it directly even though
;; "in real life" it would be a helper function of eval-exp.
(define (eval-under-env e env)
  (cond [(var? e)
         (envlookup env (var-string e))]
        [(add? e)
         (let ([v1 (eval-under-env (add-e1 e) env)]
               [v2 (eval-under-env (add-e2 e) env)])
           (if (and (int? v1)
                    (int? v2))
               (int (+ (int-num v1)
                       (int-num v2)))
               (error "MUPL addition applied to non-number")))]
        ;; CHANGE add more cases here
        [(int? e) e]
        [(ifgreater? e)
         (let ([v1 (eval-under-env (ifgreater-e1 e) env)]
               [v2 (eval-under-env (ifgreater-e2 e) env)])
           (if (and (int? v1)
                    (int? v2))
               (if (> (int-num v1) (int-num v2))
                   (eval-under-env (ifgreater-e3 e) env)
                   (eval-under-env (ifgreater-e4 e) env))
               (error "MUPL ifgreater applied to non-number")))]
        [(fun? e)
         (closure env e)]
        [(call? e)
         (let ([cl (eval-under-env (call-funexp e) env)]
               [actual-v (eval-under-env (call-actual e) env)])
           (if (closure? cl)
               (letrec ([clenv (closure-env cl)]
                        [clfun (closure-fun cl)]
                        [clfun-nameopt (fun-nameopt clfun)]
                        [extended-arg (cons (cons (fun-formal clfun) actual-v) clenv)]
                        [extended-rec (if clfun-nameopt
                                          (cons (cons clfun-nameopt cl) extended-arg)
                                          extended-arg)])
                 (eval-under-env (fun-body clfun) extended-rec))
               (error "MUPL call applied to non-closure")))]
        [(mlet? e)
         (let ([v (eval-under-env (mlet-e e) env)])
           (eval-under-env (mlet-body e) (cons (cons (mlet-var e) v) env)))]
        [(apair? e)
         (let ([v1 (eval-under-env (apair-e1 e) env)]
               [v2 (eval-under-env (apair-e2 e) env)])
           (apair v1 v2))]
        [(fst? e)
         (let ([v (eval-under-env (fst-e e) env)])
           (if (apair? v)
               (apair-e1 v)
               (error "MUPL fst applied to non-apair")))]
        [(snd? e)
         (let ([v (eval-under-env (snd-e e) env)])
           (if (apair? v)
               (apair-e2 v)
               (error "MUPL snd applied to non-apair")))]
        [(aunit? e) e]
        [(isaunit? e)
         (let ([v (eval-under-env (isaunit-e e) env)])
           (if (aunit? v)
               (int 1)
               (int 0)))]
        [(closure? e) e]
        [#t (error (format "bad MUPL expression: ~v" e))]))

;; Do NOT change
(define (eval-exp e)
  (eval-under-env e null))

;; Problem 3

(define (ifaunit e1 e2 e3)
  (ifgreater (isaunit e1) (int 0) e2 e3))

(define (mlet* lstlst e2)
  (if (null? lstlst)
      e2
      (mlet (car (car lstlst)) (cdr (car lstlst)) (mlet* (cdr lstlst) e2))))

(define (ifeq e1 e2 e3 e4)
  (mlet* (list (cons "left" e1) (cons "right" e2))
         (ifgreater (var "left") (var "right") e4 (ifgreater (var "right") (var "left") e4 e3))))

;; Problem 4

(define mupl-map
  (fun #f "mapper"
       (fun "rec" "l"
            (ifaunit (var "l") (aunit) (apair (call (var "mapper") (fst (var "l"))) (call (var "rec") (snd (var "l"))))))))

(define mupl-mapAddN
  (mlet "map" mupl-map
        (fun #f "i"
             (fun #f "l"
                  (call (call (var "map") (fun #f "item" (add (var "item") (var "i")))) (var "l"))))))

;; Challenge Problem

(struct fun-challenge (nameopt formal body freevars) #:transparent) ;; a recursive(?) 1-argument function

;; We will test this function directly, so it must do
;; as described in the assignment
(define (compute-free-vars e)
  (define (helper e)
    (cond [(var? e) (cons e (set (var-string e)))]
          [(int? e) (cons e (set))]
          [(add? e) (let ([e1 (helper (add-e1 e))]
                          [e2 (helper (add-e2 e))])
                      (cons (add (car e1) (car e2))
                            (set-union (cdr e1) (cdr e2))))]
          [(ifgreater? e) (let ([e1 (helper (ifgreater-e1 e))]
                                [e2 (helper (ifgreater-e2 e))]
                                [e3 (helper (ifgreater-e3 e))]
                                [e4 (helper (ifgreater-e4 e))])
                            (cons (ifgreater (car e1) (car e2) (car e3) (car e4))
                                  (set-union (cdr e1) (cdr e2) (cdr e3) (cdr e4))))]
          [(fun? e) (letrec ([funbody (fun-body e)]
                             [body (helper funbody)]
                             [funformal (fun-formal e)]
                             [freevars-exclude-arg (set-remove (cdr body) funformal)]
                             [funname (fun-nameopt e)]
                             [freevars (if funname
                                           (set-remove freevars-exclude-arg funname)
                                           freevars-exclude-arg)])
                      (cons (fun-challenge funname funformal (car body) freevars) freevars))]
          [(call? e) (let ([funexp (helper (call-funexp e))]
                           [actual (helper (call-actual e))])
                       (cons (call (car funexp) (car actual))
                             (set-union (cdr funexp) (cdr actual))))]
          [(mlet? e) (let ([var (mlet-var e)]
                           [esub (helper (mlet-e e))]
                           [body (helper (mlet-body e))])
                       (cons (mlet var (car esub) (car body))
                             (set-union (cdr esub)
                                        (set-remove (cdr body) var))))]
          [(apair? e) (let ([e1 (helper (apair-e1 e))]
                            [e2 (helper (apair-e2 e))])
                        (cons (apair (car e1) (car e2))
                              (set-union (cdr e1) (cdr e2))))]
          [(fst? e) (let ([esub (helper (fst-e e))])
                      (cons (fst (car esub)) (cdr esub)))]
          [(snd? e) (let ([esub (helper (snd-e e))])
                      (cons (snd (car esub)) (cdr esub)))]
          [(aunit? e) (cons e (set))]
          [(isaunit? e) (let ([esub (helper (isaunit-e e))])
                          (cons (isaunit (car esub)) (cdr esub)))]))
  (car (helper e)))

;; Do NOT share code with eval-under-env because that will make
;; auto-grading and peer assessment more difficult, so
;; copy most of your interpreter here and make minor changes
(define (eval-under-env-c e env)
  (cond [(var? e)
         (envlookup env (var-string e))]
        [(add? e)
         (let ([v1 (eval-under-env-c (add-e1 e) env)]
               [v2 (eval-under-env-c (add-e2 e) env)])
           (if (and (int? v1)
                    (int? v2))
               (int (+ (int-num v1)
                       (int-num v2)))
               (error "MUPL addition applied to non-number")))]
        ;; CHANGE add more cases here
        [(int? e) e]
        [(ifgreater? e)
         (let ([v1 (eval-under-env-c (ifgreater-e1 e) env)]
               [v2 (eval-under-env-c (ifgreater-e2 e) env)])
           (if (and (int? v1)
                    (int? v2))
               (if (> (int-num v1) (int-num v2))
                   (eval-under-env-c (ifgreater-e3 e) env)
                   (eval-under-env-c (ifgreater-e4 e) env))
               (error "MUPL ifgreater applied to non-number")))]
        [(fun-challenge? e)
         (closure (filter (lambda (v) (set-member? (fun-challenge-freevars e) (car v))) env) e)]
        [(call? e)
         (let ([cl (eval-under-env-c (call-funexp e) env)]
               [actual-v (eval-under-env-c (call-actual e) env)])
           (if (closure? cl)
               (letrec ([clenv (closure-env cl)]
                        [clfun (closure-fun cl)]
                        [clfun-nameopt (fun-challenge-nameopt clfun)]
                        [extended-arg (cons (cons (fun-challenge-formal clfun) actual-v) clenv)]
                        [extended-rec (if clfun-nameopt
                                          (cons (cons clfun-nameopt cl) extended-arg)
                                          extended-arg)])
                 (eval-under-env-c (fun-challenge-body clfun) extended-rec))
               (error "MUPL call applied to non-closure")))]
        [(mlet? e)
         (letrec ([v (eval-under-env-c (mlet-e e) env)]
                  [extended-env (cons (cons (mlet-var e) v) env)])
           (eval-under-env-c (mlet-body e) extended-env))]
        [(apair? e)
         (let ([v1 (eval-under-env-c (apair-e1 e) env)]
               [v2 (eval-under-env-c (apair-e2 e) env)])
           (apair v1 v2))]
        [(fst? e)
         (let ([v (eval-under-env-c (fst-e e) env)])
           (if (apair? v)
               (apair-e1 v)
               (error "MUPL fst applied to non-apair")))]
        [(snd? e)
         (let ([v (eval-under-env-c (snd-e e) env)])
           (if (apair? v)
               (apair-e2 v)
               (error "MUPL snd applied to non-apair")))]
        [(aunit? e) e]
        [(isaunit? e)
         (let ([v (eval-under-env-c (isaunit-e e) env)])
           (if (aunit? v)
               (int 1)
               (int 0)))]
        [(closure? e) e]
        [#t (error (format "bad MUPL expression: ~v" e))]))

;; Do NOT change this
(define (eval-exp-c e)
  (eval-under-env-c (compute-free-vars e) null))
