;; How to access Params..

(module
    (func (param i32) (param $p1 i32) (result i32)
        local.get 0  ;; 0 is index of param..
        local.get $p1 
        i32.add
    )
)

;; stack.. (LIFO)

;; to give name to param, we use $ symbol in front..
;; i32.add will add all the i32 params and store it as a single plate in the stack..